use crate::utils::{rank_rev, reindex, sort_vector_rev};
use num_traits::{Float, FromPrimitive};
use std::iter::Sum;

pub struct BenjaminiYekutieli<T: Float + FromPrimitive + Sum> {
    num_elements: usize,
    current_max: T,
    cumulative: T,
}

impl<T: Float + FromPrimitive + Sum> Default for BenjaminiYekutieli<T> {
    fn default() -> Self {
        Self::new(0)
    }
}

impl<T: Float + FromPrimitive + Sum> BenjaminiYekutieli<T> {
    /// Creates a new instance of BenjaminiYekutieli
    ///
    /// # Arguments
    ///
    /// * `num_elements` - The number of elements in the dataset
    #[must_use]
    pub fn new(num_elements: usize) -> Self {
        let cumulative = Self::calculate_cumulative(num_elements);
        Self {
            num_elements,
            current_max: T::one(),
            cumulative,
        }
    }

    /// Calculates the cumulative sum used in the BY procedure
    fn calculate_cumulative(num_elements: usize) -> T {
        (1..=num_elements)
            .map(|x| T::from_usize(x).unwrap().recip())
            .sum()
    }

    /// Calculates the adjusted p-value given the p-value and the rank.
    ///
    /// This function is not deterministic and may give different q-values for the same
    /// p-value depending on the internal state (i.e., if the current max has changed).
    ///
    /// # Arguments
    ///
    /// * `pvalue` - The p-value to adjust
    /// * `rank` - The rank of the p-value
    pub fn adjust(&mut self, pvalue: T, rank: usize) -> T {
        let n = T::from_usize(self.num_elements).unwrap();
        let r = T::from_usize(rank).unwrap();
        let qvalue = (pvalue * self.cumulative * (n / r))
            .min(self.current_max)
            .min(T::one());
        self.current_max = qvalue;
        qvalue
    }

    /// Performs the procedure on a slice of floats.
    ///
    /// This first sorts the p-values in descending order,
    /// then performs the correction using the ascending order ranks,
    /// and finally reindexes the array to return it in the same order as provided.
    ///
    /// # Arguments
    ///
    /// * `slice` - A slice of p-values to adjust
    #[must_use]
    pub fn adjust_slice(slice: &[T]) -> Vec<T> {
        if slice.is_empty() {
            return Vec::new();
        }

        let mut method = Self::new(slice.len());
        let original_index = rank_rev(slice);
        let max = original_index.len();

        let sorted_qvalues = sort_vector_rev(slice)
            .iter()
            .enumerate()
            .map(|(idx, &p)| method.adjust(p, max - idx))
            .collect::<Vec<T>>();

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
                0.7611111111111112,
                0.8562500,
                0.9133333333333333,
                0.5708333333333333
            ]
        );
    }

    #[test]
    fn example_null() {
        let pvalues: Vec<f64> = vec![];
        let adj_pvalues = BenjaminiYekutieli::adjust_slice(&pvalues);
        assert_eq!(adj_pvalues, vec![]);
    }

    #[test]
    fn example_adjust() {
        let mut b = BenjaminiYekutieli::new(100);
        assert_eq!(b.adjust(0.001, 1), 0.5187377517639621);
        assert_eq!(b.adjust(0.001, 2), 0.25936887588198104);
        assert_eq!(b.adjust(0.001, 3), 0.1729125839213207);
    }
}
