use memmap2::MmapMut;
use sha2::{Digest, Sha256};
use std::fs::OpenOptions;
use std::io;
use std::path::Path;
use thiserror::Error;

const CACHE_LINE_SIZE: usize = 64;
const MAGIC: u64 = 0x505645_4c454447;
const HEADER_LEN: usize = 64;

#[derive(Debug, Error)]
pub enum AppendError {
    #[error("capacity exceeded")]
    CapacityExceeded,
    #[error("storage error: {0}")]
    Io(#[from] io::Error),
}

#[repr(C)]
struct Header {
    magic: u64,
    length: u32,
    offset: u64,
    hash: [u8; 32],
    timestamp_ns: u64,
    _reserved: [u8; 4],
}

impl Header {
    fn to_bytes(&self) -> [u8; HEADER_LEN] {
        let mut bytes = [0u8; HEADER_LEN];
        bytes[0..8].copy_from_slice(&self.magic.to_le_bytes());
        bytes[8..12].copy_from_slice(&self.length.to_le_bytes());
        bytes[12..20].copy_from_slice(&self.offset.to_le_bytes());
        bytes[20..52].copy_from_slice(&self.hash);
        bytes[52..60].copy_from_slice(&self.timestamp_ns.to_le_bytes());
        bytes[60..64].copy_from_slice(&self._reserved);
        bytes
    }
}

fn align_up(size: usize) -> usize {
    (size + CACHE_LINE_SIZE - 1) / CACHE_LINE_SIZE * CACHE_LINE_SIZE
}

pub struct MmapLog {
    mmap: MmapMut,
    write_offset: usize,
    capacity: usize,
}

impl MmapLog {
    pub fn create(path: &Path, capacity: usize) -> Result<Self, AppendError> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;
        file.set_len(capacity as u64)?;
        let mmap = unsafe { MmapMut::map_mut(&file)? };
        Ok(Self {
            mmap,
            write_offset: 0,
            capacity,
        })
    }

    pub fn append(&mut self, data: &[u8]) -> Result<usize, AppendError> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock before epoch")
            .as_nanos() as u64;

        let header = Header {
            magic: MAGIC,
            length: data.len() as u32,
            offset: self.write_offset as u64,
            hash: Sha256::digest(data).into(),
            timestamp_ns: now,
            _reserved: [0u8; 4],
        };

        let header_bytes = header.to_bytes();
        let padded_len = align_up(HEADER_LEN + data.len());
        if self.write_offset + padded_len > self.capacity {
            return Err(AppendError::CapacityExceeded);
        }

        let start = self.write_offset;
        let dst = &mut self.mmap[start..start + padded_len];
        dst[..HEADER_LEN].copy_from_slice(&header_bytes);
        dst[HEADER_LEN..HEADER_LEN + data.len()].copy_from_slice(data);
        // remaining bytes are already zero from the file extension

        self.write_offset += padded_len;
        Ok(start)
    }

    pub fn current_offset(&self) -> usize {
        self.write_offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn append_only_and_aligned() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test_ledger.dat");
        let mut log = MmapLog::create(&path, 1024 * 64).unwrap();

        let payload = b"physical event payload".to_vec();
        let offset = log.append(&payload).unwrap();

        // Written region must be aligned to 64-byte cache lines
        assert_eq!(offset % CACHE_LINE_SIZE, 0);

        // The file must contain the payload bytes at the recorded offset
        let file = OpenOptions::new().read(true).open(&path).unwrap();
        let mut buf = Vec::new();
        let mut handle = file.take(1024 * 64);
        handle.read_to_end(&mut buf).unwrap();
        // The payload appears after the 64-byte header
        assert!(buf[offset + HEADER_LEN..offset + HEADER_LEN + payload.len()] == *payload);
    }
}

use std::io::Read;
