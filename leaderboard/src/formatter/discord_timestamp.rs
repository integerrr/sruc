use std::cmp::Ordering;
use std::fmt::Display;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Clone, Serialize, Deserialize, Default)]
pub struct DiscordTimestamp {
    time: i64,
}

impl DiscordTimestamp {
    pub fn display(&self, display_mode: DiscordTimestampDisplay) -> String {
        let identifier = match display_mode {
            DiscordTimestampDisplay::ShortDate => "d",
            DiscordTimestampDisplay::FullDate => "D",
            DiscordTimestampDisplay::HourMinuteTime => "t",
            DiscordTimestampDisplay::HourMinuteSecondTime => "T",
            DiscordTimestampDisplay::FullDateTime => "f",
            DiscordTimestampDisplay::FullDateTimeDayOfWeek => "F",
            DiscordTimestampDisplay::Relative => "R",
        };

        format!("<t:{}:{}>", self.time, identifier)
    }

    pub fn new_from_secs_remaining(secs_remaining: i64) -> Self {
        let current_time = chrono::Utc::now().timestamp();
        Self {
            time: secs_remaining + current_time,
        }
    }

    pub fn new(time: i64) -> Self {
        Self { time }
    }
}

impl Display for DiscordTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DiscordTimestamp( time = {} )", self.time)
    }
}

impl Ord for DiscordTimestamp {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for DiscordTimestamp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for DiscordTimestamp {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for DiscordTimestamp {}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiscordTimestampDisplay {
    ShortDate,
    FullDate,
    HourMinuteTime,
    HourMinuteSecondTime,
    #[default]
    FullDateTime,
    FullDateTimeDayOfWeek,
    Relative,
}
