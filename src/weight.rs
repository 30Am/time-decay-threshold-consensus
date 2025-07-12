use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Voter {
    pub id: String,
    pub base_weight: Decimal,
    pub reputation_score: Option<Decimal>, // 0.0 to 1.0
}

#[derive(Debug)]
pub struct VoteRecord {
    pub voter_id: String,
    pub vote_time: DateTime<Utc>,
    pub final_weight: Decimal,
}

pub fn calculate_weight(voter: &Voter) -> Decimal {
    let base = voter.base_weight;
    match voter.reputation_score {
        Some(rep) => base * (dec!(1.0) + rep.min(dec!(0.5))), // Max +50% bonus
        None => base,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_weight_no_reputation() {
        let voter = Voter {
            id: "alice".to_string(),
            base_weight: dec!(100.0),
            reputation_score: None,
        };
        let weight = calculate_weight(&voter);
        assert_eq!(weight, dec!(100.0));
    }

    #[test]
    fn test_weight_with_reputation() {
        let voter = Voter {
            id: "bob".to_string(),
            base_weight: dec!(100.0),
            reputation_score: Some(dec!(0.2)),
        };
        let weight = calculate_weight(&voter);
        assert_eq!(weight, dec!(120.0)); // 100 + 20%
    }

    #[test]
    fn test_weight_max_cap() {
        let voter = Voter {
            id: "carol".to_string(),
            base_weight: dec!(100.0),
            reputation_score: Some(dec!(1.0)), // capped at 50%
        };
        let weight = calculate_weight(&voter);
        assert_eq!(weight, dec!(150.0));
    }
}
