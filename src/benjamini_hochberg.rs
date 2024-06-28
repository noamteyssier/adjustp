use crate::utils::{rank_rev, reindex, sort_vector_rev};
use num_traits::{Float, FromPrimitive};

/// Performs the Benjamini-Hochberg step-up procedure
pub struct BenjaminiHochberg<T: Float + FromPrimitive> {
    num_elements: usize,
    current_max: T,
}

impl<T: Float + FromPrimitive> BenjaminiHochberg<T> {
    /// Creates a new instance of BenjaminiHochberg
    #[must_use]
    pub fn new(num_elements: usize) -> Self {
        Self {
            num_elements,
            current_max: T::one(),
        }
    }

    /// Calculates the adjusted pvalue given the pvalue and the rank.
    /// Keep in mind that this function is not deterministic and may give different qvalues for the same call of pvalue depending on the internal state (i.e. if the current max has changed).
    pub fn adjust(&mut self, pvalue: T, rank: usize) -> T {
        let n = T::from_usize(self.num_elements)
            .expect("Failed to convert `self.num_elements` usize to T");
        let r = T::from_usize(rank).expect("Failed to convert `rank` usize to T");
        let qvalue = (pvalue * n / r).min(self.current_max).min(T::one());
        self.current_max = qvalue;
        qvalue
    }

    /// Performs the procedure on a slice of floats.
    ///
    /// This first sorts the pvalues in a descending order.
    /// Then performs the correction using the ascending order ranks.
    /// Finally it reindexes the array to return it in the same order as provided.
    #[must_use]
    pub fn adjust_slice(slice: &[T]) -> Vec<T> {
        if slice.is_empty() {
            return Vec::new();
        }

        let mut method = Self::new(slice.len());
        let original_index = rank_rev(slice);
        let max = original_index.len() - 1;

        let sorted_qvalues = sort_vector_rev(slice)
            .iter()
            .enumerate()
            .map(|(idx, &p)| method.adjust(p, max - idx + 1))
            .collect::<Vec<T>>();

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
        let expected = [0.25, 0.3333333333333333, 0.375, 0.4, 0.25];
        assert_eq!(adj_pvalues, expected);
    }

    #[test]
    fn example_null() {
        let pvalues = vec![];
        let adj_pvalues = BenjaminiHochberg::<f64>::adjust_slice(&pvalues);
        assert_eq!(adj_pvalues, vec![]);
    }

    #[test]
    fn example_adjust_f64() {
        let mut b = BenjaminiHochberg::<f64>::new(100);
        assert_eq!(b.adjust(0.001, 1), 0.1);
        assert_eq!(b.adjust(0.001, 2), 0.05);
        assert_eq!(b.adjust(0.001, 3), 0.03333333333333333);
    }

    #[test]
    fn example_adjust_f32() {
        let mut b = BenjaminiHochberg::<f32>::new(100);
        assert_eq!(b.adjust(0.001, 1), 0.1);
        assert_eq!(b.adjust(0.001, 2), 0.05);
        assert_eq!(b.adjust(0.001, 3), 0.033333335);
    }
}
