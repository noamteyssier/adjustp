use num_traits::{Float, FromPrimitive};

/// Performs the Bonferroni Correction
pub struct Bonferroni {
    num_elements: usize,
}
impl Bonferroni {
    /// Creates a new instance of Bonferroni
    #[must_use]
    pub fn new(num_elements: usize) -> Self {
        Self { num_elements }
    }

    /// Calculates the adjusted pvalue given the pvalue
    #[must_use]
    pub fn adjust<T: Float + FromPrimitive>(&self, pvalue: T) -> T {
        (pvalue * T::from_usize(self.num_elements).unwrap()).min(T::one())
    }

    /// Performs the procedure on a slice of floats
    #[must_use]
    pub fn adjust_slice<T: Float + FromPrimitive>(slice: &[T]) -> Vec<T> {
        let b = Self::new(slice.len());
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
        let pvalues: Vec<f64> = vec![];
        let adj_pvalues = Bonferroni::adjust_slice(&pvalues);
        assert_eq!(adj_pvalues, vec![]);
    }

    #[test]
    fn example_adjust() {
        let b = Bonferroni::new(100);
        let p = 0.001;
        assert_eq!(b.adjust(p), 0.1);
    }
}
