use gold_cmp::{summarize, Outcome};

#[test]
fn empty_is_zero() {
    let s = summarize(&[]);
    assert_eq!(s.n, 0);
    assert_eq!(s.a_wins, 0);
}

#[test]
fn all_a_wins() {
    let s = summarize(&vec![Outcome::AWins; 100]);
    assert!((s.a_win_rate_excl_ties - 1.0).abs() < 1e-9);
    assert!(s.a_meaningfully_ahead);
}

#[test]
fn all_b_wins_a_not_ahead() {
    let s = summarize(&vec![Outcome::BWins; 100]);
    assert_eq!(s.a_win_rate_excl_ties, 0.0);
    assert!(!s.a_meaningfully_ahead);
}

#[test]
fn coin_flip_not_meaningful() {
    let mut outs = Vec::new();
    for _ in 0..50 {
        outs.push(Outcome::AWins);
        outs.push(Outcome::BWins);
    }
    let s = summarize(&outs);
    assert!((s.a_win_rate_excl_ties - 0.5).abs() < 1e-9);
    assert!(!s.a_meaningfully_ahead);
}

#[test]
fn ties_excluded_from_rate() {
    // 5 A, 0 B, 95 ties.
    let mut outs = vec![Outcome::Tie; 95];
    outs.extend(vec![Outcome::AWins; 5]);
    let s = summarize(&outs);
    assert_eq!(s.ties, 95);
    assert_eq!(s.a_wins, 5);
    assert!((s.a_win_rate_excl_ties - 1.0).abs() < 1e-9);
    // n=5 decided, CI is wide; might not flag as meaningful — both are OK.
    assert!(s.ci95_half_width >= 0.0);
}

#[test]
fn small_lead_flagged_when_n_is_large() {
    // 60 A, 40 B over 100 decided → 95% CI half-width ≈ 0.096; lower bound ~0.504 > 0.5
    let mut outs = vec![Outcome::AWins; 60];
    outs.extend(vec![Outcome::BWins; 40]);
    let s = summarize(&outs);
    assert!(s.a_meaningfully_ahead);
}
