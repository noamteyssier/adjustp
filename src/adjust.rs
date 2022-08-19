use crate::{
    bonferroni::Bonferroni,
    benjamini_hochberg::BenjaminiHochberg,
    benjamini_yekutieli::BenjaminiYekutieli
};

#[derive(Copy, Clone)]
pub enum Procedure {
    Bonferroni,
    BenjaminiHochberg,
    BenjaminiYekutieli,
    BH,
    BY,
    FDR
}

#[must_use] pub fn adjust(pvalues: &[f64], method: Procedure) -> Vec<f64> {
    match method {
        Procedure::Bonferroni => 
            Bonferroni::adjust_slice(pvalues),
        Procedure::BenjaminiHochberg | Procedure::BH | Procedure::FDR => 
            BenjaminiHochberg::adjust_slice(pvalues),
        Procedure::BenjaminiYekutieli | Procedure::BY => 
            BenjaminiYekutieli::adjust_slice(pvalues)
    }
}

#[cfg(test)]
mod testing {
    use super::{adjust, Procedure};

    #[test]
    fn example() {
        let pvalues = vec![0.1, 0.2, 0.3, 0.4, 0.1];

        let adj_bonferroni = adjust(&pvalues, Procedure::Bonferroni);
        let adj_bh = adjust(&pvalues, Procedure::BenjaminiHochberg);
        let adj_by = adjust(&pvalues, Procedure::BenjaminiYekutieli);

        assert_eq!(
            adj_bonferroni,
            vec![0.5, 1.0, 1.0, 1.0, 0.5]
        );

        assert_eq!(
            adj_bh, 
            vec![0.25, 0.33333333333333337, 0.375, 0.4, 0.25]
        );

        assert_eq!(
            adj_by,
            vec![0.5708333333333333, 0.7611111111111111, 0.8562500, 0.91333333333333333, 0.5708333333333333]
        );
    }
}
