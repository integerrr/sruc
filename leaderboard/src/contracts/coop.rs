use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use ei::ei::contract::{Goal, GradeSpec};
use ei::ei::{contract_coop_status_response::ResponseStatus, Contract, ContractCoopStatusResponse};

use crate::api::get_coop_status;
use crate::error::InvalidCoopCode;
use crate::formatter;
use crate::formatter::discord_timestamp::DiscordTimestamp;
use crate::formatter::duration::Duration;

#[derive(Debug, Error, Clone)]
pub struct Coop {
    coop_status: ContractCoopStatusResponse,

    // These fields are stored because you have to extract these from the contract
    // they belong in, which would be a pain in the ass to find after the fact.
    grade_spec: GradeSpec,
    contract_farm_max_secs_allowed: f64,
}

impl Coop {
    fn new(coop_status: ContractCoopStatusResponse, contract: Contract) -> Self {
        Self {
            coop_status: coop_status.clone(),
            grade_spec: contract
                .grade_specs
                .iter()
                .find(|&g| g.grade() == coop_status.grade())
                .unwrap_or(&GradeSpec::default())
                .clone(),
            contract_farm_max_secs_allowed: contract.length_seconds(),
        }
    }

    pub fn coop_id(&self) -> &str {
        self.coop_status.coop_identifier()
    }

    pub fn contract_id(&self) -> &str {
        self.coop_status.contract_identifier()
    }

    pub fn stripped_coop_id(&self) -> String {
        match self.coop_id().get(0..6) {
            Some(slice) => slice.to_string(),
            None => formatter::string_formatter::left_align(self.coop_id(), 6),
        }
    }

    pub fn boosted_count(&self) -> u32 {
        self.coop_status
            .contributors
            .iter()
            .filter(|player| player.boost_tokens_spent() >= 4)
            .count()
            .try_into()
            .expect("there's no way there can be more than 2^32 players in a coop")
    }

    pub fn total_tokens(&self) -> u32 {
        self.coop_status
            .contributors
            .iter()
            .map(|player| player.boost_tokens() + player.boost_tokens_spent())
            .sum()
    }

    pub fn finishing_time(&self) -> DiscordTimestamp {
        DiscordTimestamp::new_from_secs_remaining(
            self.predicted_seconds_remaining()
                - self.coop_status.seconds_since_all_goals_achieved() as i64,
        )
    }

    pub fn total_predicted_duration(&self) -> Duration {
        let total = self.contract_farm_max_secs_allowed as i64
            - self.coop_allowable_seconds_remaining() as i64
            + self.predicted_seconds_remaining()
            - self.coop_status.seconds_since_all_goals_achieved() as i64;

        Duration::new(total)
    }

    pub fn green_scrolled(&self) -> bool {
        self.coop_status.cleared_for_exit()
    }

    fn coop_allowable_seconds_remaining(&self) -> f64 {
        self.coop_status.seconds_remaining()
    }

    fn egg_goal(&self) -> f64 {
        self.grade_spec
            .goals
            .iter()
            .max_by(|&g1, &g2| {
                g1.target_amount()
                    .partial_cmp(&g2.target_amount())
                    .unwrap_or(Ordering::Less)
            })
            .unwrap_or(&Goal::default())
            .target_amount()
    }

    fn shipped_eggs(&self) -> f64 {
        self.coop_status.total_amount()
    }

    /// Returns the **per second** shipping rate of the entire coop.
    fn total_shipping_rate(&self) -> f64 {
        self.coop_status
            .contributors
            .iter()
            .map(|p| p.contribution_rate())
            .sum()
    }

    /// Returns the total offline eggs laid by players.
    ///
    /// # Details
    ///
    /// `farm_info.timestamp()` is basically `LastFarmSyncTimeUnix - CurrentTimeUnix` in seconds,
    /// so the negative is required in the maths. Credits to WHALE for figuring that shit out.
    /// `farm_info` can also be `None` if the player is marked as `[departed]` or has a private farm.
    fn total_offline_eggs(&self) -> f64 {
        self.coop_status
            .contributors
            .iter()
            .map(|p| match &p.farm_info {
                None => 0f64,
                Some(farm) => p.contribution_rate() * -farm.timestamp(),
            })
            .sum()
    }

    fn eggs_remaining(&self) -> f64 {
        let zero = 0f64;
        let calc_eggs = self.egg_goal() - self.shipped_eggs() - self.total_offline_eggs();
        zero.max(calc_eggs)
    }

    fn predicted_seconds_remaining(&self) -> i64 {
        (self.eggs_remaining() / self.total_shipping_rate()) as i64
    }
}

impl Display for Coop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PartialOrd for Coop {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Coop {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut ord = self
            .total_predicted_duration()
            .cmp(&other.total_predicted_duration());
        if ord == Ordering::Equal {
            ord = other.boosted_count().cmp(&self.boosted_count());
        }
        if ord == Ordering::Equal {
            ord = other.total_tokens().cmp(&self.total_tokens());
        }
        if ord == Ordering::Equal {
            ord = self.finishing_time().cmp(&other.finishing_time());
        }
        ord
    }
}

impl PartialEq for Coop {
    fn eq(&self, other: &Self) -> bool {
        self.coop_id() == other.coop_id() && self.contract_id() == other.contract_id()
    }
}

impl Eq for Coop {}

// region:      --- Builder States
#[derive(Debug, Clone, Default)]
pub struct NoContract;
#[derive(Debug, Clone, Default)]
pub struct WithContract(Contract);

#[derive(Debug, Clone, Default)]
pub struct NoCoopCode;
#[derive(Debug, Clone, Default)]
pub struct WithCoopCode(String);
// endregion:   --- Builder States

#[derive(Debug, Error, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CoopBuilder<K, C> {
    contract: K,
    coop_code: C,
}

impl CoopBuilder<NoContract, NoCoopCode> {
    pub fn new() -> Self {
        CoopBuilder::default()
    }
}

impl<C> CoopBuilder<NoContract, C> {
    pub fn with_contract(self, contract: Contract) -> CoopBuilder<WithContract, C> {
        CoopBuilder {
            contract: WithContract(contract),
            coop_code: self.coop_code,
        }
    }
}

impl<K> CoopBuilder<K, NoCoopCode> {
    pub fn with_coop_code(self, coop_code: impl Into<String>) -> CoopBuilder<K, WithCoopCode> {
        CoopBuilder {
            contract: self.contract,
            coop_code: WithCoopCode(coop_code.into()),
        }
    }
}

impl CoopBuilder<WithContract, WithCoopCode> {
    pub async fn build(self) -> Result<Coop> {
        let coop = get_coop_status(self.contract.0.identifier(), &self.coop_code.0).await?;
        match &coop.response_status() {
            ResponseStatus::NoError => Ok(Coop::new(coop, self.contract.0)),
            _ => Err(Error::from(InvalidCoopCode)),
        }
    }
}
