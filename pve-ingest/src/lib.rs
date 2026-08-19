pub mod listener;
pub mod mapper;

use pve_core::model::REAEvent;
use pve_core::thermo::validate_thermodynamic_conservation;
use pve_storage::MmapLog;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IngestError {
    #[error("parse error: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("thermodynamic validation failed: {0}")]
    Thermo(pve_core::thermo::ThermoError),
    #[error("storage error: {0}")]
    Storage(#[from] pve_storage::AppendError),
}

pub struct IngestPipeline {
    log: MmapLog,
}

impl IngestPipeline {
    pub fn new(log: MmapLog) -> Self {
        Self { log }
    }

    pub fn process_raw(&mut self, raw: &[u8]) -> Result<(), IngestError> {
        let event: REAEvent = mapper::parse_raw(raw)?;
        validate_thermodynamic_conservation(
            event.state_delta.energy_consumed_mj,
            event.state_delta.energy_useful_mj,
            event.state_delta.energy_loss_mj,
        )
        .map_err(IngestError::Thermo)?;

        let bytes = serde_json::to_vec(&event)?;
        self.log.append(&bytes)?;
        Ok(())
    }
}
