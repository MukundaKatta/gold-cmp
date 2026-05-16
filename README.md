# gold-cmp

[![crates.io](https://img.shields.io/crates/v/gold-cmp.svg)](https://crates.io/crates/gold-cmp)

Pairwise comparison runner for gold-set LLM evals. Win/loss/tie
counting, normal-approx 95% CI, "meaningfully ahead" boolean.

```rust
use gold_cmp::{summarize, Outcome};
let outcomes = vec![Outcome::AWins, Outcome::BWins, Outcome::AWins, Outcome::Tie];
let s = summarize(&outcomes);
println!("A win rate (excl ties): {:.2} ± {:.2}", s.a_win_rate_excl_ties, s.ci95_half_width);
```

Zero deps. MIT or Apache-2.0.
