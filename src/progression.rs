use chrono::{DateTime, Utc};
use crate::threshold::{ThresholdEscalator, EscalationType};

#[derive(Debug)]
pub enum ProgressionMode {
    Conservative,
    Aggressive,
    Adaptive(u32), // pass/fail history window
}

#[derive(Debug)]
pub struct ThresholdStrategy {
    pub mode: ProgressionMode,
    pub base_threshold: f64,
    pub max_threshold: f64,
    pub start_time: DateTime<Utc>,
    pub min_votes: usize,
}

impl ThresholdStrategy {
    pub fn to_escalator(&self, proposal_index: usize, recent_outcomes: &[bool]) -> ThresholdEscalator {
        let escalation = match self.mode {
            ProgressionMode::Conservative => EscalationType::Linear(0.002), // slower rise
            ProgressionMode::Aggressive => EscalationType::Exponential(0.05), // fast convergence
            ProgressionMode::Adaptive(window) => {
                let failures = recent_outcomes.iter().rev().take(window as usize).filter(|&&ok| !ok).count();
                let penalty_factor = (failures as f64 / window as f64).min(1.0);
                EscalationType::Exponential(0.03 + penalty_factor * 0.05) // adjust k
            }
        };

        ThresholdEscalator {
            base_threshold: self.base_threshold,
            max_threshold: self.max_threshold,
            start_time: self.start_time,
            escalation,
        }
    }

    pub fn meets_min_votes(&self, vote_count: usize) -> bool {
        vote_count >= self.min_votes
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Utc, Duration};

    #[test]
    fn test_conservative_strategy() {
        let strategy = ThresholdStrategy {
            mode: ProgressionMode::Conservative,
            base_threshold: 0.5,
            max_threshold: 0.9,
            start_time: Utc::now() - Duration::seconds(60),
            min_votes: 3,
        };

        let escalator = strategy.to_escalator(0, &[]);
        let threshold = escalator.current_threshold(Utc::now());
        assert!(threshold > 0.5 && threshold < 0.9);
        assert!(strategy.meets_min_votes(3));
    }

    #[test]
    fn test_adaptive_behavior() {
        let outcomes = vec![false, false, true, false];
        let strategy = ThresholdStrategy {
            mode: ProgressionMode::Adaptive(4),
            base_threshold: 0.5,
            max_threshold: 0.95,
            start_time: Utc::now() - Duration::seconds(45),
            min_votes: 2,
        };

        let escalator = strategy.to_escalator(2, &outcomes);
        let threshold = escalator.current_threshold(Utc::now());
        assert!(threshold > 0.5);
    }
}


