use std::ops::Mul;
use crate::utils::{rank_rev, reindex, sort_vector_rev};

/// Performs the Benjamini-Hochberg step-up procedure
pub struct BenjaminiHochberg {
    num_elements: f64,
    current_max: f64
}

impl BenjaminiHochberg {
    /// Creates a new instance of BenjaminiHochberg
    #[must_use] 
    pub fn new(num_elements: f64) -> Self {
        Self { num_elements, current_max: 1. }
    }

    /// Calculates the adjusted pvalue given the pvalue and the rank. 
    /// Keep in mind that this function is not deterministic and may give different qvalues for the same call of pvalue depending on the internal state (i.e. if the current max has changed).
    pub fn adjust(&mut self, pvalue: f64, rank: usize) -> f64 {
        let qvalue = pvalue.mul(self.num_elements / rank as f64)
            .min(self.current_max).min(1.0);
        self.current_max = qvalue;
        qvalue
    }

    /// Performs the procedure on a slice of floats. 
    /// 
    /// This first sorts the pvalues in a descending order.
    /// Then performs the correction using the ascending order ranks.
    /// Finally it reindexes the array to return it in the same order as provided.
    #[must_use] 
    pub fn adjust_slice(slice: &[f64]) -> Vec<f64> {
        if slice.is_empty() { return Vec::new() }

        let mut method = Self::new(slice.len() as f64);

        let original_index = rank_rev(slice);
        let max = original_index.len() - 1;

        let sorted_qvalues = sort_vector_rev(slice)
            .iter()
            .enumerate()
            .map(|(idx, p)| (max - idx + 1, p))
            .map(|(r, p)| method.adjust(*p, r))
            .collect::<Vec<f64>>();

        reindex(&sorted_qvalues, &original_index)
    }
}

#[cfg(test)]
mod testing {
    use super::BenjaminiHochberg;
    
    #[test]
    fn example() {
        let pvalues = vec![0.1, 0.2, 0.3, 0.4, 0.1];
        let adj_pvalues = BenjaminiHochberg::adjust_slice(&pvalues);
        assert_eq!(adj_pvalues, vec![0.25, 0.33333333333333337, 0.375, 0.4, 0.25]);
    }

    #[test]
    fn example_null() {
        let pvalues = vec![];
        let adj_pvalues = BenjaminiHochberg::adjust_slice(&pvalues);
        assert_eq!(adj_pvalues, vec![]);
    }

    #[test]
    fn example_adjust() {
        let mut b = BenjaminiHochberg::new(100.0);
        assert_eq!(b.adjust(0.001, 1), 0.1);
        assert_eq!(b.adjust(0.001, 2), 0.05);
        assert_eq!(b.adjust(0.001, 3), 0.03333333333333334);
    }
}
