use std::ops::Mul;

/// Performs the Bonferroni Correction
pub struct Bonferroni {
    num_elements: f64
}
impl Bonferroni {
    /// Creates a new instance of Bonferroni
    #[must_use] 
    pub fn new(num_elements: f64) -> Self {
        Self { num_elements }
    }

    /// Calculates the adjusted pvalue given the pvalue
    #[must_use] 
    pub fn adjust(&self, pvalue: f64) -> f64 {
        pvalue.mul(self.num_elements).min(1.)
    }

    /// Performs the procedure on a slice of floats
    #[must_use] 
    pub fn adjust_slice(slice: &[f64]) -> Vec<f64> {
        let b = Self::new(slice.len() as f64);
        slice.iter().map(|x| b.adjust(*x)).collect()
    }
}

#[cfg(test)]
mod testing {
    use super::Bonferroni;
    
    #[test]
    fn example() {
        let pvalues = vec![0.1, 0.2, 0.3, 0.4];
        let adj_pvalues = Bonferroni::adjust_slice(&pvalues);
        assert_eq!(adj_pvalues, vec![0.4, 0.8, 1.0, 1.0]);
    }

    #[test]
    fn example_null() {
        let pvalues = vec![];
        let adj_pvalues = Bonferroni::adjust_slice(&pvalues);
        assert_eq!(adj_pvalues, vec![]);
    }

    #[test]
    fn example_adjust() {
        let b = Bonferroni::new(100.0);
        let p = 0.001;
        assert_eq!(b.adjust(p), 0.1);
    }
}
