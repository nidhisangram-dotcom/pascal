use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct PacioliVector {
    components: Vec<f64>,
}

impl PacioliVector {
    pub fn new(components: Vec<f64>) -> Result<Self, PacioliError> {
        if components.iter().any(|&x| x < 0.0) {
            return Err(PacioliError::NegativeComponent);
        }
        Ok(Self { components })
    }

    pub fn components(&self) -> &[f64] {
        &self.components
    }

    pub fn add(&self, other: &Self) -> Result<Self, PacioliError> {
        if self.components.len() != other.components.len() {
            return Err(PacioliError::DimensionMismatch);
        }
        let sum = self
            .components
            .iter()
            .zip(other.components.iter())
            .map(|(&a, &b)| a + b)
            .collect();
        Ok(Self { components: sum })
    }

    pub fn sub(&self, other: &Self) -> Result<Self, PacioliError> {
        if self.components.len() != other.components.len() {
            return Err(PacioliError::DimensionMismatch);
        }
        let diff = self
            .components
            .iter()
            .zip(other.components.iter())
            .map(|(&a, &b)| a - b)
            .collect();
        Ok(Self { components: diff })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PacioliPair {
    pub debit: PacioliVector,
    pub credit: PacioliVector,
}

impl PacioliPair {
    pub fn new(debit: PacioliVector, credit: PacioliVector) -> Result<Self, PacioliError> {
        if debit.components().len() != credit.components().len() {
            return Err(PacioliError::DimensionMismatch);
        }
        Ok(Self { debit, credit })
    }

    /// Component‑wise Pacioli addition:
    /// [d1 // c1] ⊕ [d2 // c2] = [d1 + d2 // c1 + c2]
    pub fn add(&self, other: &Self) -> Result<Self, PacioliError> {
        let debit = self.debit.add(&other.debit)?;
        let credit = self.credit.add(&other.credit)?;
        Ok(Self { debit, credit })
    }

    /// d = c  ⇒  the pair balances to zero.
    pub fn is_zero_balance(&self) -> bool {
        self.debit == self.credit
    }

    /// Cross‑sum equivalence:
    /// (d1, c1) ∼ (d2, c2)  ⇔  d1 + c2 = d2 + c1
    pub fn equivalent(&self, other: &Self) -> Result<bool, PacioliError> {
        let d1_plus_c2 = self.debit.add(&other.credit)?;
        let d2_plus_c1 = other.debit.add(&self.credit)?;
        Ok(d1_plus_c2 == d2_plus_c1)
    }
}

#[derive(Debug, Error)]
pub enum PacioliError {
    #[error("vector components must be non‑negative")]
    NegativeComponent,
    #[error("vector dimensions do not match")]
    DimensionMismatch,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pacioli_componentwise_addition() {
        let d1 = PacioliVector::new(vec![10.0, 5.0]).unwrap();
        let c1 = PacioliVector::new(vec![10.0, 5.0]).unwrap();
        let d2 = PacioliVector::new(vec![1.0, 2.0]).unwrap();
        let c2 = PacioliVector::new(vec![1.0, 2.0]).unwrap();

        let pair1 = PacioliPair::new(d1, c1).unwrap();
        let pair2 = PacioliPair::new(d2, c2).unwrap();
        let sum = pair1.add(&pair2).unwrap();

        assert_eq!(sum.debit.components(), &[11.0, 7.0]);
        assert_eq!(sum.credit.components(), &[11.0, 7.0]);
        assert!(sum.is_zero_balance());
    }

    #[test]
    fn pacioli_equivalence_relation() {
        let d1 = PacioliVector::new(vec![5.0]).unwrap();
        let c1 = PacioliVector::new(vec![3.0]).unwrap();
        let d2 = PacioliVector::new(vec![2.0]).unwrap();
        let c2 = PacioliVector::new(vec![4.0]).unwrap();

        let p1 = PacioliPair::new(d1, c1).unwrap();
        let p2 = PacioliPair::new(d2, c2).unwrap();
        assert!(p1.equivalent(&p2).unwrap());
    }
}
