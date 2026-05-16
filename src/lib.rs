//! # gold-cmp
//!
//! Pairwise comparison runner for gold-set LLM evals.
//!
//! Pattern: for each gold case, ask a judge (LLM or human) which of two
//! candidate outputs is better, or call it a tie. Aggregate into a
//! win-rate with a 95% CI and a yes/no answer on "is A meaningfully
//! better than B" using a two-sided binomial test (normal
//! approximation; OK for n ≥ 30).
//!
//! ## Example
//!
//! ```
//! use gold_cmp::{summarize, Outcome};
//! let outcomes = vec![
//!     Outcome::AWins, Outcome::AWins, Outcome::BWins,
//!     Outcome::Tie,   Outcome::AWins, Outcome::AWins,
//!     Outcome::BWins, Outcome::AWins,
//! ];
//! let s = summarize(&outcomes);
//! assert_eq!(s.a_wins, 5);
//! assert!(s.a_win_rate_excl_ties > 0.5);
//! ```

#![deny(missing_docs)]

/// Outcome of one pairwise judgment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// A is better.
    AWins,
    /// B is better.
    BWins,
    /// The judge called it equivalent.
    Tie,
}

/// Aggregated summary.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Summary {
    /// Total judgments.
    pub n: u64,
    /// Wins for A.
    pub a_wins: u64,
    /// Wins for B.
    pub b_wins: u64,
    /// Ties.
    pub ties: u64,
    /// A's win rate excluding ties.
    pub a_win_rate_excl_ties: f64,
    /// Normal-approximation 95% CI half-width on the excl-ties win rate.
    pub ci95_half_width: f64,
    /// True when A is meaningfully ahead (CI excludes 0.5).
    pub a_meaningfully_ahead: bool,
}

/// Summarize a list of outcomes.
pub fn summarize(outcomes: &[Outcome]) -> Summary {
    let n = outcomes.len() as u64;
    let a_wins = outcomes.iter().filter(|o| **o == Outcome::AWins).count() as u64;
    let b_wins = outcomes.iter().filter(|o| **o == Outcome::BWins).count() as u64;
    let ties = outcomes.iter().filter(|o| **o == Outcome::Tie).count() as u64;
    let decided = a_wins + b_wins;
    let p = if decided == 0 {
        0.5
    } else {
        a_wins as f64 / decided as f64
    };
    let ci = if decided == 0 {
        0.0
    } else {
        1.96 * (p * (1.0 - p) / decided as f64).sqrt()
    };
    let ahead = decided > 0 && (p - ci) > 0.5;
    Summary {
        n,
        a_wins,
        b_wins,
        ties,
        a_win_rate_excl_ties: p,
        ci95_half_width: ci,
        a_meaningfully_ahead: ahead,
    }
}
