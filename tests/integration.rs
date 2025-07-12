use chrono::{Utc, Duration};
use rust_decimal_macros::dec;
use time_decay_consensus::voting::{Vote, DecayType};
use time_decay_consensus::threshold::{ThresholdEscalator, EscalationType};
use time_decay_consensus::window::VotingWindow;
use time_decay_consensus::weight::{Voter, calculate_weight};
use time_decay_consensus::progression::{ProgressionMode, ThresholdStrategy};
use rust_decimal::prelude::ToPrimitive;


#[test]
fn end_to_end_voting_simulation() {
    let start = Utc::now() - Duration::seconds(12);
    let window = VotingWindow::new(start, Duration::seconds(10), Duration::seconds(5));
    let now = Utc::now();

    let voter = Voter {
        id: "alice".to_string(),
        base_weight: dec!(100.0),
        reputation_score: Some(dec!(0.3)),
    };
    let weight = calculate_weight(&voter);

    let vote = Vote {
        initial_weight: weight.to_f64().unwrap(),
        timestamp: now - Duration::seconds(8),
    };

    let decayed = vote.current_weight(now, DecayType::Exponential(0.05));
    assert!(decayed > 10.0);

    let escalator = ThresholdEscalator {
        base_threshold: 0.5,
        max_threshold: 0.9,
        start_time: start,
        escalation: EscalationType::Exponential(0.04),
    };
    let threshold = escalator.current_threshold(now);

    let open = window.is_open(now);
    let grace = window.is_in_grace_period(now);
    let expired = window.is_expired(now);

    let strategy = ThresholdStrategy {
        mode: ProgressionMode::Conservative,
        base_threshold: 0.5,
        max_threshold: 0.9,
        start_time: start,
        min_votes: 1,
    };
    let meets = strategy.meets_min_votes(1);

    assert!(open || grace);
    assert!(!expired);
    assert!(threshold >= 0.5 && threshold <= 0.9);
    assert!(meets);
}
