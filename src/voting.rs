use chrono::{DateTime, Utc};
use std::f64::consts::E;

#[derive(Debug, Clone)]
pub enum DecayType {
    Exponential(f64), // decay constant k
    Linear(f64),      // decay rate per second
    Stepped(Vec<(i64, f64)>), // Vec<(seconds, weight multiplier)>
}

#[derive(Debug)]
pub struct Vote {
    pub initial_weight: f64,
    pub timestamp: DateTime<Utc>,
}

impl Vote {
    pub fn current_weight(&self, now: DateTime<Utc>, decay: DecayType) -> f64 {
        let elapsed_secs = (now - self.timestamp).num_seconds() as f64;

        let raw_weight = match decay {
            DecayType::Exponential(k) => self.initial_weight * E.powf(-k * elapsed_secs),
            DecayType::Linear(k) => (self.initial_weight - k * elapsed_secs).max(0.0),
            DecayType::Stepped(steps) => {
                let mut weight = self.initial_weight;
                for (time_limit, multiplier) in steps {
                    if elapsed_secs >= time_limit as f64 {
                        weight *= multiplier;
                    }
                }
                weight
            }
        };

        // Enforce minimum floor (10%)
        raw_weight.max(self.initial_weight * 0.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn test_exponential_decay() {
        let vote = Vote { initial_weight: 100.0, timestamp: Utc::now() - Duration::seconds(10) };
        let weight = vote.current_weight(Utc::now(), DecayType::Exponential(0.1));
        assert!(weight < 100.0 && weight > 10.0);
    }

    #[test]
    fn test_linear_decay() {
        let vote = Vote { initial_weight: 100.0, timestamp: Utc::now() - Duration::seconds(20) };
        let weight = vote.current_weight(Utc::now(), DecayType::Linear(2.0)); // loses 2/sec
        assert_eq!(weight, 60.0); // 100 - (2*20) = 60
    }

    #[test]
    fn test_stepped_decay() {
        let steps = vec![(5, 0.8), (10, 0.5)];
        let vote = Vote { initial_weight: 100.0, timestamp: Utc::now() - Duration::seconds(12) };
        let weight = vote.current_weight(Utc::now(), DecayType::Stepped(steps));
        assert_eq!(weight, 100.0 * 0.8 * 0.5); // Two steps applied
    }

    #[test]
    fn test_weight_floor() {
        let vote = Vote { initial_weight: 100.0, timestamp: Utc::now() - Duration::seconds(1000) };
        let weight = vote.current_weight(Utc::now(), DecayType::Exponential(1.0));
        assert!(weight >= 10.0); // Floor at 10% of 100
    }
}
