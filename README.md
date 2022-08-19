# adjustp

# Summary
This is a crate to perform pvalue adjustments and is inspired by the R function `p.adjust`.
There are currently only three methods available: `Bonferroni`, `BenjaminiHochberg`, and
`BenjaminiYekutieli`.

This crate gives a single interface for each of these and does not expect the p-values to be
presorted before calculating.

# Usage

## Basic Usage
Here's an example for a `Bonferroni` correction.

```rust
use adjustp::{adjust, Procedure};

let pvalues = vec![0.1, 0.2, 0.3, 0.4, 0.1];
let qvalues = adjust(&pvalues, Procedure::Bonferroni);
assert_eq!(qvalues, vec![0.5, 1.0, 1.0, 1.0, 0.5]);
```

And another example for a `BenjaminiHochberg` adjustment.

```rust
use adjustp::{adjust, Procedure};

let pvalues = vec![0.1, 0.2, 0.3, 0.4, 0.1];
let qvalues = adjust(&pvalues, Procedure::BenjaminiHochberg);
assert_eq!(qvalues, vec![0.25, 0.33333333333333337, 0.375, 0.4, 0.25]);
```

And another example for a `BenjaminiYekutieli` adjustment.

```rust
use adjustp::{adjust, Procedure};

let pvalues = vec![0.1, 0.2, 0.3, 0.4, 0.1];
let qvalues = adjust(&pvalues, Procedure::BenjaminiYekutieli);
assert_eq!(qvalues, vec![0.5708333333333333, 0.7611111111111111, 0.8562500, 0.91333333333333333, 0.5708333333333333]);
```
