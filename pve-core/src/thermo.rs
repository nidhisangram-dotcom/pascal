use thiserror::Error;

pub fn validate_thermodynamic_conservation(
    consumed: f64,
    useful: f64,
    loss: f64,
) -> Result<(), ThermoError> {
    let delta = consumed - useful - loss;
    if (delta - 0.0).abs() > 1e-6 {
        return Err(ThermoError::Violation(delta));
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum ThermoError {
    #[error("Thermodynamic conservation violated: E_C - E_U - E_L = {0}")]
    Violation(f64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_balanced_flow() {
        assert!(validate_thermodynamic_conservation(100.0, 60.0, 40.0).is_ok());
    }

    #[test]
    fn rejects_unbalanced_flow() {
        assert!(validate_thermodynamic_conservation(100.0, 50.0, 40.0).is_err());
    }
}
