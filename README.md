# Time-Decay Threshold Consensus

A Rust-based voting system using time-decay weights, dynamic thresholds, reputation-aware weighting, and adaptive progression logic.

## Features
- Time-weighted voting (Exponential, Linear, Stepped)
- Dynamic threshold escalation (Linear, Exponential, Adaptive)
- Voting window control with grace period
- Weight calculation with reputation bonus
- CLI interface for simulation
- Integration testing & modular design

## Usage
```bash
cargo run -- simulate --voter alice --weight 100 --reputation 0.25 --vote-delay-secs 5
cargo run -- threshold --base 0.51 --max 0.9 --k 0.05 --elapsed-secs 30
cargo run -- demo-all
