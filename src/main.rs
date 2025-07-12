mod voting;
mod threshold;
mod window;
mod weight;
mod progression;

use chrono::{Utc, Duration};
use rust_decimal_macros::dec;
use rust_decimal::prelude::{ToPrimitive, FromPrimitive};
use rust_decimal::Decimal;
use clap::{Parser, Subcommand};

use voting::{Vote, DecayType};
use threshold::{ThresholdEscalator, EscalationType};
use window::VotingWindow;
use weight::{Voter, calculate_weight};
use progression::{ThresholdStrategy, ProgressionMode};

#[derive(Parser)]
#[command(name = "Consensus CLI", version = "1.0", about = "Simulate voting consensus")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Simulate a full vote session
    Simulate {
        #[arg(short, long, default_value = "alice")]
        voter: String,

        #[arg(short, long, default_value_t = 100.0)]
        weight: f64,

        #[arg(long, default_value_t = 0.3)]
        reputation: f64,

        #[arg(long, default_value_t = 10)]
        vote_delay_secs: u64,
    },

    /// Print current threshold value
    Threshold {
        #[arg(short, long, default_value_t = 0.5)]
        base: f64,

        #[arg(short, long, default_value_t = 0.9)]
        max: f64,

        #[arg(long, default_value_t = 0.05)]
        k: f64,

        #[arg(long, default_value_t = 30)]
        elapsed_secs: i64,
    },

    /// Run all logic demo (for testing/debugging)
    DemoAll,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Simulate { voter, weight, reputation, vote_delay_secs } => {
            let now = Utc::now();
            let vote_time = now - Duration::seconds(vote_delay_secs as i64);

            let voter = Voter {
                id: voter,
                base_weight: Decimal::from_f64(weight).unwrap_or(Decimal::ZERO),
                reputation_score: Some(Decimal::from_f64(reputation).unwrap_or(Decimal::ZERO)),
            };

            let effective_weight = calculate_weight(&voter);
            let vote = Vote {
                initial_weight: effective_weight.to_f64().unwrap(),
                timestamp: vote_time,
            };

            let decayed = vote.current_weight(now, DecayType::Exponential(0.05));
            let window = VotingWindow::new(now - Duration::seconds(5), Duration::seconds(10), Duration::seconds(5));

            println!("Vote from {} has effective weight: {:.2}", voter.id, effective_weight);
            println!("Decayed weight after {}s: {:.2}", vote_delay_secs, decayed);
            println!("Voting open? {}", window.is_open(now));
        }

        Commands::Threshold { base, max, k, elapsed_secs } => {
            let start = Utc::now() - Duration::seconds(elapsed_secs);
            let escalator = ThresholdEscalator {
                base_threshold: base,
                max_threshold: max,
                start_time: start,
                escalation: EscalationType::Exponential(k),
            };
            let now = Utc::now();
            let current = escalator.current_threshold(now);
            println!("Threshold after {}s: {:.2}%", elapsed_secs, current * 100.0);
        }

        Commands::DemoAll => demo_all(),
    }
}

fn demo_all() {
    println!("\n--- Running demo_all() ---");

    // Threshold escalation
    let escalator = ThresholdEscalator {
        base_threshold: 0.51,
        max_threshold: 0.9,
        start_time: Utc::now(),
        escalation: EscalationType::Exponential(0.05),
    };

    std::thread::sleep(std::time::Duration::from_secs(2));
    let current = escalator.current_threshold(Utc::now());
    println!("Current threshold after 2s: {:.2}%", current * 100.0);

    // Voting window
    let now = Utc::now();
    let window = VotingWindow::new(now, Duration::seconds(3), Duration::seconds(2));
    println!("Voting open? {}", window.is_open(Utc::now()));
    std::thread::sleep(std::time::Duration::from_secs(4));
    println!("In grace period? {}", window.is_in_grace_period(Utc::now()));
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("Expired? {}", window.is_expired(Utc::now()));

    // Weight calculation
    let voter = Voter {
        id: "dave".to_string(),
        base_weight: Decimal::from_f64(80.0).unwrap_or(Decimal::ZERO),
        reputation_score: Some(Decimal::from_f64(0.3).unwrap_or(Decimal::ZERO)),
    };
    let final_weight = calculate_weight(&voter);
    println!("Final weight for {}: {}", voter.id, final_weight);

    // Adaptive progression
    let recent_outcomes = vec![false, true, false, true];
    let strategy = ThresholdStrategy {
        mode: ProgressionMode::Adaptive(4),
        base_threshold: 0.51,
        max_threshold: 0.9,
        start_time: Utc::now(),
        min_votes: 3,
    };

    let escalator = strategy.to_escalator(5, &recent_outcomes);
    let current = escalator.current_threshold(Utc::now());
    println!("Adaptive current threshold: {:.2}%", current * 100.0);
    println!("Minimum votes met? {}", strategy.meets_min_votes(4));
}
