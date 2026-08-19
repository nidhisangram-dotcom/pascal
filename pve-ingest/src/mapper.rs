use pve_core::model::{PropertyVector, REAEvent};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RawPayload {
    event_id: String,
    resource_id: String,
    agent_id: String,
    state_delta: RawPropertyVector,
    sure_signature: String,
}

#[derive(Debug, Deserialize)]
struct RawPropertyVector {
    energy_consumed_mj: f64,
    energy_useful_mj: f64,
    energy_loss_mj: f64,
    mass_kg: f64,
    labor_hrs: f64,
    title_count: u64,
}

pub fn parse_raw(raw: &[u8]) -> Result<REAEvent, serde_json::Error> {
    let raw_payload: RawPayload = serde_json::from_slice(raw)?;
    Ok(REAEvent {
        event_id: raw_payload.event_id,
        resource_id: raw_payload.resource_id,
        agent_id: raw_payload.agent_id,
        state_delta: PropertyVector {
            energy_consumed_mj: raw_payload.state_delta.energy_consumed_mj,
            energy_useful_mj: raw_payload.state_delta.energy_useful_mj,
            energy_loss_mj: raw_payload.state_delta.energy_loss_mj,
            mass_kg: raw_payload.state_delta.mass_kg,
            labor_hrs: raw_payload.state_delta.labor_hrs,
            title_count: raw_payload.state_delta.title_count,
        },
        sure_signature: raw_payload.sure_signature,
    })
}
