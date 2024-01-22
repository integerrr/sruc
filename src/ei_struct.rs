use std::fmt::Display;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ei::ContractCoopStatusResponse;

pub mod builder;

#[derive(Debug, Error, Clone, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct MajCoop {
    contract_id: String,
    coop_id: String,
    boosted_count: u32,
    total_tokens: u32,
    finishing_time: DiscordTimestamp,
    finished: bool,
}

#[derive(Debug, Error, Clone, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct DiscordTimestamp {
    time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiscordTimestampDisplay {
    ShortDate,
    FullDate,
    HourMinuteTime,
    HourMinuteSecondTime,
    FullDateTime,
    FullDateTimeDayOfWeek,
    Relative,
}

impl TryFrom<ContractCoopStatusResponse> for MajCoop {
    type Error = &'static str;

    fn try_from(resp: ContractCoopStatusResponse) -> Result<Self, Self::Error> {
        let boosted_count: u32 = resp
            .contributors
            .iter()
            .filter(|player| player.boost_tokens_spent() >= 6)
            .count()
            .try_into()
            .expect("there's no way there can be more than 2^32 players in a coop");

        let total_tokens: u32 = resp
            .contributors
            .iter()
            .map(|player| player.boost_tokens() + player.boost_tokens_spent())
            .sum();

        Ok(MajCoop {
            contract_id: resp.contract_identifier().to_string(),
            coop_id: resp.coop_identifier().to_string(),
            boosted_count,
            total_tokens,
            finishing_time: DiscordTimestamp { time: 0 },
            finished: resp.all_goals_achieved(),
        })
    }
}

impl MajCoop {
    pub fn build_table_row(&self) -> String {
        format!(
            "[⧉](<https://eicoop-carpet.netlify.app/{}/{}>)`{} |    {}    |   {}   |{}|{}`",
            self.contract_id,
            self.coop_id,
            self.stripped_coop_id(),
            self.boosted_count,
            self.total_tokens,
            self.finishing_time
                .display(DiscordTimestampDisplay::Relative),
            self.finished
        )
    }

    pub fn stripped_coop_id(&self) -> String {
        self.coop_id[0..=5].to_owned()
    }
}

impl Display for MajCoop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}: \n  boosted_count = {},\n  total_tokens = {},\n  finishing_time = {},\n finished = {}",
        self.contract_id,
        self.coop_id,
        self.boosted_count,
        self.total_tokens,
        self.finishing_time,
        self.finished
        )
    }
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

        format!("`<t:{}:{}>`", self.time, identifier)
    }

    pub fn finish_time_from_secs_remaining(secs_remaining: f64) -> DiscordTimestamp {
        let current_time = chrono::Utc::now().timestamp();
        let secs_remaining_as_i64 = secs_remaining as i64;
        DiscordTimestamp {
            time: secs_remaining_as_i64 + current_time,
        }
    }
}

impl Display for DiscordTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DiscordTimestamp( time = {} )", self.time)
    }
}

impl ContractCoopStatusResponse {
    /// Returns the **per second** shipping rate of the entire coop.
    pub fn total_shipping_rate(&self) -> f64 {
        self.contributors
            .iter()
            .map(|player| player.contribution_rate())
            .sum()
    }
}
