use std::fmt::Display;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ei::{Contract, ContractCoopStatusResponse};

pub mod builder;

#[derive(Debug, Error, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MajCoop {
    contract: Contract,
    coop: ContractCoopStatusResponse,
}

#[derive(Debug, Error, Clone, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct DiscordTimestamp {
    time: i64,
}

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContractGrade {
    #[default]
    Unset,
    AAA,
    AA,
    A,
    B,
    C,
}

impl MajCoop {
    pub fn new(contract: Contract, coop: ContractCoopStatusResponse) -> Self {
        Self { contract, coop }
    }

    pub fn build_table_row(&self) -> String {
        format!(
            "[⧉](<https://eicoop-carpet.netlify.app/{}/{}>)`{} |    {}    |   {}   |   {}   | {} `",
            self.contract_id(),
            self.coop_id(),
            self.stripped_coop_id(),
            self.boosted_count(),
            self.total_tokens(),
            self.coop_predicted_duration(),
            self.finishing_time()
                .display(DiscordTimestampDisplay::Relative),
        )
    }

    pub fn contract_id(&self) -> String {
        self.coop.contract_identifier().to_owned()
    }

    pub fn coop_id(&self) -> String {
        self.coop.coop_identifier().to_owned()
    }

    pub fn grade(&self) -> ContractGrade {
        match self.coop.grade {
            Some(0) => ContractGrade::Unset,
            Some(1) => ContractGrade::C,
            Some(2) => ContractGrade::B,
            Some(3) => ContractGrade::A,
            Some(4) => ContractGrade::AA,
            Some(5) => ContractGrade::AAA,
            _ => ContractGrade::default(),
        }
    }

    pub fn stripped_coop_id(&self) -> String {
        let coop_id = self.coop_id();
        coop_id[0..=5].to_owned()
    }

    pub fn boosted_count(&self) -> u32 {
        self.coop
            .contributors
            .iter()
            .filter(|player| player.boost_tokens_spent() >= 6)
            .count()
            .try_into()
            .expect("there's no way there can be more than 2^32 players in a coop")
    }

    pub fn total_tokens(&self) -> u32 {
        self.coop
            .contributors
            .iter()
            .map(|player| player.boost_tokens() + player.boost_tokens_spent())
            .sum()
    }

    pub fn coop_predicted_duration(&self) -> String {
        todo!()
        // "test".to_string()
    }
    pub fn finishing_time(&self) -> DiscordTimestamp {
        todo!()
        // DiscordTimestamp { time: 0 }
    }
}

impl Display for MajCoop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{} ({}): \n  boosted_count = {},\n  total_tokens = {},\n  finishing_time = {},\n finished = {}",
        self.contract_id(),
        self.coop_id(),
        self.grade(),
        self.boosted_count(),
        self.total_tokens(),
        self.coop_predicted_duration(),
        self.finishing_time()
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

impl Display for ContractGrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ContractGrade = {}",
            match self {
                Self::Unset => "Unset",
                Self::C => "C",
                Self::B => "B",
                Self::A => "A",
                Self::AA => "AA",
                Self::AAA => "AAA",
            }
        )
    }
}
