/// Bonferroni Correction
pub mod bonferroni;

/// Benjamini-Hochberg Procedure
pub mod benjamini_hochberg;

/// Benjamini-Yekutieli Procedure
pub mod benjamini_yekutieli;

/// User API
pub mod adjust;

/// Utilities shared by Adjustment Procedures
pub mod utils;

/// Expose adjustment and procedure method to user
pub use adjust::{adjust, Procedure};
