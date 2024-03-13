use std::cmp::Ordering;

const SECONDS_IN_A_MINUTE: i64 = 60;
const SECONDS_IN_AN_HOUR: i64 = SECONDS_IN_A_MINUTE * 60;
const SECONDS_IN_A_DAY: i64 = SECONDS_IN_AN_HOUR * 24;

#[derive(Debug)]
pub struct Duration {
    pub duration_in_seconds: i64,
}

impl Duration {
    pub fn new(duration: i64) -> Self {
        Self {
            duration_in_seconds: duration,
        }
    }

    pub fn format(&self) -> String {
        let (day, hour, min) = (self.calc_days(), self.calc_hours(), self.calc_minutes());

        let day_str = if day > 0 {
            format!("{day}d")
        } else {
            "".to_string()
        };
        let hr_str = if hour > 0 {
            format!("{hour}h")
        } else {
            "".to_string()
        };

        format!("{day_str}{hr_str}{min}m")
    }

    pub fn format_too_long(&self) -> String {
        if self.calc_days() >= 100 {
            return "Too long".to_string();
        }

        self.format()
    }

    fn calc_days(&self) -> i64 {
        self.duration_in_seconds / SECONDS_IN_A_DAY
    }

    fn calc_hours(&self) -> i64 {
        (self.duration_in_seconds - self.calc_days() * SECONDS_IN_A_DAY) / SECONDS_IN_AN_HOUR
    }

    fn calc_minutes(&self) -> i64 {
        (self.duration_in_seconds
            - self.calc_days() * SECONDS_IN_A_DAY
            - self.calc_hours() * SECONDS_IN_AN_HOUR)
            / SECONDS_IN_A_MINUTE
    }
}

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Duration {
    fn cmp(&self, other: &Self) -> Ordering {
        self.duration_in_seconds.cmp(&other.duration_in_seconds)
    }
}

impl PartialEq for Duration {
    fn eq(&self, other: &Self) -> bool {
        self.duration_in_seconds == other.duration_in_seconds
    }
}

impl Eq for Duration {}
