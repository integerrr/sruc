use std::fmt::{Display, Formatter};
use std::slice::Iter;

use anyhow::{Context, Result};
use log::error;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use ei::ei::Contract;

use crate::contracts::coop::{Coop, CoopBuilder};
use crate::egg_inc_api::{get_backup_contracts, get_periodicals};

#[derive(Debug, Error, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ActiveContract {
    contract: Contract,
    coops: Vec<Coop>,
}

impl ActiveContract {
    fn new(contract: Contract) -> Self {
        Self {
            contract,
            coops: vec![],
        }
    }

    pub async fn add_coops(&mut self, coop_codes: &[impl Into<String> + Clone]) -> Result<()> {
        for code in coop_codes {
            let new = match CoopBuilder::new()
                .with(self.contract.clone(), code.clone())
                .build()
                .await
            {
                Ok(coop) => coop,
                Err(_) => {
                    error!("Invalid coop code: \"{}\"", code.clone().into());
                    continue;
                }
            };
            // .context(format!("Invalid coop code: \"{}\"", code.clone().into()))?;
            self.coops.push(new);
        }
        self.coops.sort();
        Ok(())
    }

    pub fn coops(&self) -> Iter<'_, Coop> {
        self.coops.iter()
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
// endregion:   --- Builder States

#[derive(Debug, Error, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ActiveContractBuilder<I> {
    contract_id: I,
}

impl ActiveContractBuilder<NoContractId> {
    pub fn new() -> Self {
        ActiveContractBuilder::default()
    }
}

impl<I> ActiveContractBuilder<I> {
    pub fn with_contract_id(
        self,
        contract_id: impl Into<String>,
    ) -> ActiveContractBuilder<ContractId> {
        ActiveContractBuilder {
            contract_id: ContractId(contract_id.into()),
        }
    }
}

impl ActiveContractBuilder<ContractId> {
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
            return Ok(ActiveContract::new(contract.clone()));
        }

        match get_backup_contracts(&self.contract_id.0).await {
            Ok(c) => Ok(ActiveContract::new(c.clone())),
            Err(e) => Err(e),
        }
    }
}
