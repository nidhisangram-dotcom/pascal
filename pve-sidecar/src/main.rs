mod config;

use pve_ingest::listener::MqttListener;
use pve_ingest::IngestPipeline;
use pve_storage::MmapLog;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::Config::from_file("configs/default_plant.toml")?;
    let log = MmapLog::create(Path::new(&cfg.ledger_path), cfg.capacity_bytes)?;
    let mut pipeline = IngestPipeline::new(log);
    let listener = MqttListener::new(cfg.broker, cfg.port, cfg.client_id, cfg.topic);

    listener.run(move |payload| {
        if let Err(e) = pipeline.process_raw(&payload) {
            eprintln!("Rejected payload: {}", e);
        }
    })?;

    Ok(())
}
