use std::fmt::Display;

use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ei::{self, Contract, ContractCoopStatusResponse};

pub mod builder;

const SECONDS_IN_A_MINUTE: f64 = 60f64;
const SECONDS_IN_AN_HOUR: f64 = SECONDS_IN_A_MINUTE * 60f64;
const SECONDS_IN_A_DAY: f64 = SECONDS_IN_AN_HOUR * 24f64;

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

impl MajCoop {
    pub fn new(contract: Contract, coop: ContractCoopStatusResponse) -> Self {
        Self { contract, coop }
    }

    pub fn build_table_row(&self) -> String {
        format!(
            "[⧉](<https://eicoop-carpet.netlify.app/{}/{}>)`{} |   {}   |  {}  |   {}   | {} `",
            self.contract_id(),
            self.coop_id(),
            self.stripped_coop_id(),
            self.boosted_count(),
            self.total_tokens(),
            self.total_duration(),
            self.finishing_time()
                .display(DiscordTimestampDisplay::FullDateTime),
        )
    }

    pub fn contract_id(&self) -> String {
        self.coop.contract_identifier().to_owned()
    }

    pub fn coop_id(&self) -> String {
        self.coop.coop_identifier().to_owned()
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

    pub fn total_duration(&self) -> String {
        let contract_time_limit = self.contract.length_seconds();

        let coop_total_shipping_rate = self.coop.total_shipping_rate();
        let eggs_remaining = self.get_contract_egg_goal() - self.coop.total_amount();
        let predicted_remaining_time_to_finish = eggs_remaining / coop_total_shipping_rate;
        
        let coop_valid_time_remaining = self.coop.seconds_remaining();

        let total_prediction_coop_duration = contract_time_limit - coop_valid_time_remaining + predicted_remaining_time_to_finish;

        let day = (total_prediction_coop_duration / SECONDS_IN_A_DAY).floor();
        let hour = ((total_prediction_coop_duration - day * SECONDS_IN_A_DAY) / SECONDS_IN_AN_HOUR).floor();
        let minute = ((total_prediction_coop_duration - day * SECONDS_IN_A_DAY - hour * SECONDS_IN_AN_HOUR)
            / SECONDS_IN_A_MINUTE)
            .floor();

        format!("{}d{}h{}m", day, hour, minute)
    }

    pub fn finishing_time(&self) -> DiscordTimestamp {
        let coop_total_shipping_rate = self.coop.total_shipping_rate();
        let eggs_remaining = self.get_contract_egg_goal() - self.coop.total_amount();
        let time_remaining = eggs_remaining / coop_total_shipping_rate;

        DiscordTimestamp::finish_time_from_secs_remaining(time_remaining)
    }

    pub fn get_contract_egg_goal(&self) -> f64 {
        let contract_grade = self.coop.grade();
        let contract_spec = self
            .contract
            .grade_specs
            .iter()
            .find(|grade| grade.grade() == contract_grade)
            .expect("This grade must exist")
            .to_owned();

        let egg_goal = contract_spec
            .goals
            .iter()
            .max_by_key(|goal| OrderedFloat(goal.target_amount()))
            .expect("a largest goal `Goal` must exist")
            .target_amount();

        egg_goal
    }
}

impl Display for MajCoop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{} ({}): \n  boosted_count = {},\n  total_tokens = {},\n  finishing_time = {},\n finished = {}",
        self.contract_id(),
        self.coop_id(),
        self.coop.grade(),
        self.boosted_count(),
        self.total_tokens(),
        self.total_duration(),
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

impl Display for ei::contract::PlayerGrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ContractGrade = {}",
            match self {
                Self::GradeUnset => "Unset",
                Self::GradeC => "C",
                Self::GradeB => "B",
                Self::GradeA => "A",
                Self::GradeAa => "AA",
                Self::GradeAaa => "AAA",
            }
        )
    }
}
