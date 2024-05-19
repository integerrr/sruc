use std::fmt::{Display, Formatter};
use std::slice::Iter;

use anyhow::{Context, Result};
use log::error;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use ei::ei::Contract;

use crate::api::{get_backup_contracts, get_periodicals, maj_api};

use super::coop::{Coop, CoopBuilder};
use super::coop_flag::CoopFlag;

#[derive(Debug, Error, Clone)]
pub struct ActiveContract {
    contract: Contract,
    coop_flag: CoopFlag,
    coops: Vec<Coop>,
}

impl ActiveContract {
    fn new(contract: Contract, coop_flag: CoopFlag) -> Self {
        Self {
            contract,
            coop_flag,
            coops: vec![],
        }
    }

    pub async fn fill_coops(&mut self) -> Result<()> {
        let coop_codes =
            maj_api::get_maj_active_coop_codes(self.contract.identifier(), self.coop_flag).await?;
        for code in coop_codes {
            let new = match CoopBuilder::new()
                .with_contract(self.contract.clone())
                .with_coop_code(code.clone())
                .build()
                .await
            {
                Ok(coop) => coop,
                Err(_) => {
                    error!("Invalid coop code: \"{}\"", code.clone());
                    continue;
                }
            };
            self.coops.push(new);
        }
        self.coops.sort();
        Ok(())
    }

    pub fn contract_name(&self) -> &str {
        self.contract.identifier()
    }

    pub fn coops(&self) -> Iter<'_, Coop> {
        self.coops.iter()
    }

    pub fn all_coops_green_scrolled(&self) -> bool {
        self.coops().all(|c| c.green_scrolled())
    }
}

impl Display for ActiveContract {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// region:      --- Builder States
#[derive(Debug, Clone, Default)]
pub struct NoContractId;
#[derive(Debug, Clone, Default)]
pub struct ContractId(String);
#[derive(Debug, Clone, Default)]
pub struct CoopFlagNotSpecified;
#[derive(Debug, Clone, Default)]
pub struct CoopFlagSpecified(CoopFlag);
// endregion:   --- Builder States

#[derive(Debug, Error, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ActiveContractBuilder<I, F> {
    contract_id: I,
    coop_flag: F,
}

impl ActiveContractBuilder<NoContractId, CoopFlagNotSpecified> {
    pub fn new() -> Self {
        ActiveContractBuilder::default()
    }
}

impl<I, F> ActiveContractBuilder<I, F> {
    pub fn with_contract_id(
        self,
        contract_id: impl Into<String>,
    ) -> ActiveContractBuilder<ContractId, F> {
        ActiveContractBuilder {
            contract_id: ContractId(contract_id.into()),
            coop_flag: self.coop_flag,
        }
    }

    pub fn with_coop_flag(
        self,
        coop_flag: CoopFlag,
    ) -> ActiveContractBuilder<I, CoopFlagSpecified> {
        ActiveContractBuilder {
            contract_id: self.contract_id,
            coop_flag: CoopFlagSpecified(coop_flag),
        }
    }
}

impl ActiveContractBuilder<ContractId, CoopFlagSpecified> {
    pub async fn build(self) -> Result<ActiveContract> {
        let periodicals_response = get_periodicals().await?;
        let contracts_response = periodicals_response
            .contracts
            .context("No ContractsResponse found")?;
        if let Some(contract) = contracts_response
            .contracts
            .iter()
            .find(|&c| c.identifier() == self.contract_id.0)
        {
            return Ok(ActiveContract::new(contract.clone(), self.coop_flag.0));
        }

        match get_backup_contracts(&self.contract_id.0).await {
            Ok(c) => Ok(ActiveContract::new(c.clone(), self.coop_flag.0)),
            Err(e) => Err(e),
        }
    }
}
