use serde::{Deserialize, Serialize};

/// A valuation‑free physical property vector. All units are physical or title‑based.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertyVector {
    /// Consumed energy, E_C (MJ)
    pub energy_consumed_mj: f64,
    /// Useful work / exergy used, E_U (MJ)
    pub energy_useful_mj: f64,
    /// Entropic loss, E_L (MJ)
    pub energy_loss_mj: f64,
    /// Mass in kilograms
    pub mass_kg: f64,
    /// Assembly labor in hours
    pub labor_hrs: f64,
    /// Serial title count
    pub title_count: u64,
}

impl Default for PropertyVector {
    fn default() -> Self {
        Self {
            energy_consumed_mj: 0.0,
            energy_useful_mj: 0.0,
            energy_loss_mj: 0.0,
            mass_kg: 0.0,
            labor_hrs: 0.0,
            title_count: 0,
        }
    }
}

/// Raw REA (Resource‑Event‑Agent) primitive. No prices, no currencies.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct REAEvent {
    pub event_id: String,
    pub resource_id: String,
    pub agent_id: String,
    pub state_delta: PropertyVector,
    /// S.U.R.E. edge signature (hex‑encoded)
    pub sure_signature: String,
}
