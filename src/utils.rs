/// Returns the indices which could be used to sort the vector in ascending order
pub fn argsort<T: PartialOrd>(slice: &[T]) -> Vec<usize> {
    let mut order = (0..slice.len()).collect::<Vec<usize>>();
    order.sort_by(|a, b| slice[*a].partial_cmp(&slice[*b]).unwrap());
    order
}

/// Returns the indices which could be used to sort the vector in descending order
pub fn argsort_rev<T: PartialOrd>(slice: &[T]) -> Vec<usize> {
    let order = argsort(slice);
    let max = order.iter().max().unwrap();
    order.iter().map(|x| max - x).collect()
}

/// Returns the ranks of the vector.        
pub fn rank<T: PartialOrd>(slice: &[T]) -> Vec<usize> {
    let order = argsort(slice);
    argsort(&order)
}

/// Returns the ranks of the vector in reverse order.
pub fn rank_rev<T: PartialOrd>(slice: &[T]) -> Vec<usize> {
    let order = argsort(slice);
    argsort_rev(&order)
}

/// Sorts a vector in ascending order
#[must_use]
pub fn sort_vector(slice: &[f64]) -> Vec<f64> {
    let mut sorted_vec = slice.to_vec();
    sorted_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    sorted_vec
}

/// Sorts a vector in descending order
#[must_use]
pub fn sort_vector_rev(slice: &[f64]) -> Vec<f64> {
    let mut sorted_vec = slice.to_vec();
    sorted_vec.sort_by(|a, b| b.partial_cmp(a).unwrap());
    sorted_vec
}

/// Reindexes a vector given the ranks
#[must_use]
pub fn reindex(slice: &[f64], ranks: &[usize]) -> Vec<f64> 
{
    ranks.iter().map(|x| slice[*x]).collect()
}

#[cfg(test)]
mod testing {
    use super::{
        argsort, rank, argsort_rev,
        sort_vector, sort_vector_rev, reindex
    };

    #[test]
    fn test_argsort() {
        let floats = vec![0.3, 0.2, 0.1, 0.4];
        let order = argsort(&floats);
        assert_eq!(order, vec![2, 1, 0, 3]);
    }

    #[test]
    fn test_argsort_precision() {
        let floats = vec![3e-300, 2e-300, 1e-300, 4e-300];
        let order = argsort(&floats);
        assert_eq!(order, vec![2, 1, 0, 3]);
    }

    #[test]
    fn test_argsort_rev() {
        let floats = vec![0.3, 0.2, 0.1, 0.4];
        let order = argsort_rev(&floats);
        assert_eq!(order, vec![1, 2, 3, 0]);
    }

    #[test]
    fn test_rank() {
        let floats = vec![0.3, 0.2, 0.1, 0.4];
        let ranks = rank(&floats);
        assert_eq!(ranks, vec![2, 1, 0, 3]);
    }

    #[test]
    fn test_sort_vector() {
        let v = vec![0.3, 0.2, 0.1];
        let sv = sort_vector(&v);
        assert_eq!(sv, vec![0.1, 0.2, 0.3]);
    }

    #[test]
    fn test_sort_vector_rev() {
        let v = vec![0.1, 0.2, 0.3];
        let sv = sort_vector_rev(&v);
        assert_eq!(sv, vec![0.3, 0.2, 0.1]);
    }

    #[test]
    fn test_reindex() {
        let v = vec![0.2, 0.1, 0.3];
        let r = vec![1, 0, 2];
        let ri = reindex(&v, &r);
        assert_eq!(ri, vec![0.1, 0.2, 0.3]);
    }
}
