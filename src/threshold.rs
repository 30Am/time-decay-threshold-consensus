use chrono::{DateTime, Utc};
use std::f64::consts::E;

#[derive(Debug, Clone)]
pub enum EscalationType {
    Linear(f64),         // percent increase per second
    Exponential(f64),    // exponential growth constant k
    Sigmoid(f64),        // steepness factor
    CustomSteps(Vec<(i64, f64)>), // Vec<(elapsed seconds, threshold)>
}

pub struct ThresholdEscalator {
    pub base_threshold: f64,  // e.g. 0.51 for 51%
    pub max_threshold: f64,   // e.g. 0.90
    pub start_time: DateTime<Utc>,
    pub escalation: EscalationType,
}

impl ThresholdEscalator {
    pub fn current_threshold(&self, now: DateTime<Utc>) -> f64 {
        let elapsed_secs = (now - self.start_time).num_seconds() as f64;

        let raw = match self.escalation {
            EscalationType::Linear(rate) => self.base_threshold + rate * elapsed_secs,
            EscalationType::Exponential(k) => {
                self.base_threshold + (self.max_threshold - self.base_threshold) * (1.0 - E.powf(-k * elapsed_secs))
            }
            EscalationType::Sigmoid(k) => {
                let x = elapsed_secs;
                let sigmoid = 1.0 / (1.0 + E.powf(-k * (x - 50.0))); // sigmoid midpoint at 50s
                self.base_threshold + (self.max_threshold - self.base_threshold) * sigmoid
            }
            EscalationType::CustomSteps(ref steps) => {
                let mut threshold = self.base_threshold;
                for (secs, value) in steps {
                    if elapsed_secs >= *secs as f64 {
                        threshold = *value;
                    }
                }
                threshold
            }
        };

        raw.min(self.max_threshold)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Utc, Duration};

    #[test]
    fn test_linear_threshold() {
        let escalator = ThresholdEscalator {
            base_threshold: 0.5,
            max_threshold: 0.9,
            start_time: Utc::now() - Duration::seconds(60),
            escalation: EscalationType::Linear(0.005), // +0.5% per second
        };
        let now = Utc::now();
        let current = escalator.current_threshold(now);
        assert!(current >= 0.5 && current <= 0.9);
    }

    #[test]
    fn test_exponential_threshold() {
        let escalator = ThresholdEscalator {
            base_threshold: 0.5,
            max_threshold: 0.9,
            start_time: Utc::now() - Duration::seconds(30),
            escalation: EscalationType::Exponential(0.05),
        };
        let current = escalator.current_threshold(Utc::now());
        assert!(current > 0.5 && current < 0.9);
    }

    #[test]
    fn test_custom_step_threshold() {
        let steps = vec![(10, 0.6), (30, 0.7), (60, 0.85)];
        let escalator = ThresholdEscalator {
            base_threshold: 0.5,
            max_threshold: 0.9,
            start_time: Utc::now() - Duration::seconds(35),
            escalation: EscalationType::CustomSteps(steps),
        };
        let current = escalator.current_threshold(Utc::now());
        assert_eq!(current, 0.7);
    }
}

impl ThresholdEscalator {
    pub fn emergency_override(&self) -> f64 {
        0.5 // or any fixed override value
    }
}
