use crate::utils::{rank_rev, reindex, sort_vector_rev};
use std::ops::Mul;

/// Performs the Benjamini-Yekutieli step-up procedure
pub struct BenjaminiYekutieli {
    num_elements: f64,
    current_max: f64,
    cumulative: f64,
}

impl BenjaminiYekutieli {
    /// Creates a new instance of BenjaminiYekutieli
    #[must_use]
    pub fn new(num_elements: f64) -> Self {
        let cumulative = (1..=num_elements as usize).fold(0.0, |acc, x| acc + (1.0 / x as f64));

        Self {
            num_elements,
            current_max: 1.0,
            cumulative,
        }
    }

    /// Calculates the adjust pvalue given the pvalue and the rank.
    /// Keep in mind that this funciton is not deterministic and may give different qvalues for the
    /// same call of pvalue depending on the internal state (i.e. if the current max has changed)
    pub fn adjust(&mut self, pvalue: f64, rank: usize) -> f64 {
        let qvalue = pvalue
            .mul(self.cumulative * (self.num_elements / rank as f64))
            .min(self.current_max)
            .min(1.0);
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
        if slice.is_empty() {
            return Vec::new();
        }

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
    use super::BenjaminiYekutieli;

    #[test]
    fn example() {
        let pvalues = vec![0.1, 0.2, 0.3, 0.4, 0.1];
        let adj_pvalues = BenjaminiYekutieli::adjust_slice(&pvalues);
        assert_eq!(
            adj_pvalues,
            vec![
                0.5708333333333333,
                0.7611111111111111,
                0.8562500,
                0.91333333333333333,
                0.5708333333333333
            ]
        );
    }

    #[test]
    fn example_null() {
        let pvalues = vec![];
        let adj_pvalues = BenjaminiYekutieli::adjust_slice(&pvalues);
        assert_eq!(adj_pvalues, vec![]);
    }

    #[test]
    fn example_adjust() {
        let mut b = BenjaminiYekutieli::new(100.0);
        assert_eq!(b.adjust(0.001, 1), 0.5187377517639621);
        assert_eq!(b.adjust(0.001, 2), 0.25936887588198104);
        assert_eq!(b.adjust(0.001, 3), 0.1729125839213207);
    }
}
