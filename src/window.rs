use chrono::{DateTime, Duration, Utc};

#[derive(Debug)]
pub struct VotingWindow {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub grace_period: Duration, // Optional buffer
}

impl VotingWindow {
    pub fn new(start_time: DateTime<Utc>, duration: Duration, grace_period: Duration) -> Self {
        let end_time = start_time + duration;
        Self { start_time, end_time, grace_period }
    }

    pub fn is_open(&self, now: DateTime<Utc>) -> bool {
        now >= self.start_time && now <= self.end_time
    }

    pub fn is_in_grace_period(&self, now: DateTime<Utc>) -> bool {
        now > self.end_time && now <= self.end_time + self.grace_period
    }

    pub fn is_expired(&self, now: DateTime<Utc>) -> bool {
        now > self.end_time + self.grace_period
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_is_open() {
        let now = Utc::now();
        let window = VotingWindow::new(now - Duration::seconds(5), Duration::seconds(10), Duration::seconds(5));
        assert!(window.is_open(now));
    }

    #[test]
    fn test_is_in_grace_period() {
        let now = Utc::now();
        let window = VotingWindow::new(now - Duration::seconds(15), Duration::seconds(10), Duration::seconds(10));
        assert!(window.is_in_grace_period(now));
    }

    #[test]
    fn test_is_expired() {
        let now = Utc::now();
        let window = VotingWindow::new(now - Duration::seconds(30), Duration::seconds(10), Duration::seconds(10));
        assert!(window.is_expired(now));
    }
}
