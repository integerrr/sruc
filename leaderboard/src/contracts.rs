use std::fmt::Display;

use ei::ei::{Contract, ContractCoopStatusResponse};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::formatter::discord_formatter::{DiscordTimestamp, DiscordTimestampDisplay};

const SECONDS_IN_A_MINUTE: f64 = 60f64;
const SECONDS_IN_AN_HOUR: f64 = SECONDS_IN_A_MINUTE * 60f64;
const SECONDS_IN_A_DAY: f64 = SECONDS_IN_AN_HOUR * 24f64;

#[derive(Debug, Error, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ActiveContract {
    contract: Contract,
    coop: ContractCoopStatusResponse,
}

impl ActiveContract {
    pub fn new(contract: Contract, coop: ContractCoopStatusResponse) -> Self {
        Self { contract, coop }
    }

    pub fn build_table_row(&self) -> String {
        // let timestamp = self.coop.contributors.clone().first().unwrap().clone().farm_info.unwrap().clone().timestamp();
        // dbg!(timestamp);

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

        let coop_total_shipping_rate = self.total_shipping_rate();
        let eggs_remaining = self.get_contract_egg_goal() - self.coop.total_amount();
        let predicted_remaining_time_to_finish = eggs_remaining / coop_total_shipping_rate;

        let coop_valid_time_remaining = self.coop.seconds_remaining();

        let total_prediction_coop_duration =
            contract_time_limit - coop_valid_time_remaining + predicted_remaining_time_to_finish;

        let day = (total_prediction_coop_duration / SECONDS_IN_A_DAY).floor();
        let hour = ((total_prediction_coop_duration - day * SECONDS_IN_A_DAY) / SECONDS_IN_AN_HOUR)
            .floor();
        let minute =
            ((total_prediction_coop_duration - day * SECONDS_IN_A_DAY - hour * SECONDS_IN_AN_HOUR)
                / SECONDS_IN_A_MINUTE)
                .floor();

        format!("{}d{}h{}m", day, hour, minute)
    }

    pub fn finishing_time(&self) -> DiscordTimestamp {
        let coop_total_shipping_rate = self.total_shipping_rate();
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

    /// Returns the **per second** shipping rate of the entire coop.
    pub fn total_shipping_rate(&self) -> f64 {
        self.coop
            .contributors
            .iter()
            .map(|player| player.contribution_rate())
            .sum()
    }
}

impl Display for ActiveContract {
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
