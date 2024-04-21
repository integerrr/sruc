use std::fmt::{Display, Formatter};
use std::slice::Iter;

use anyhow::{Context, Result};
use log::error;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgQueryResult;
use sqlx::types::time::OffsetDateTime;
use sqlx::PgPool;
use thiserror::Error;

use ei::ei::Contract;

use crate::egg_inc_api::{get_backup_contracts, get_periodicals};

use super::coop::{Coop, CoopBuilder};
use super::coop_flag::CoopFlag;

#[derive(Debug, Error, Clone)]
pub struct ActiveContract {
    contract: Contract,
    coop_flag: CoopFlag,
    coops: Vec<Coop>,
    pg_pool: PgPool,
}

impl ActiveContract {
    fn new(contract: Contract, coop_flag: CoopFlag, pg_pool: PgPool) -> Self {
        Self {
            contract,
            coop_flag,
            coops: vec![],
            pg_pool,
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
            self.coops.push(new);
        }
        self.coops.sort();
        Ok(())
    }

    pub async fn update_all_coop_statuses(&mut self) {
        for coop in &mut self.coops {
            coop.update_coop_status().await;
        }
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
#[derive(Debug, Clone, Default)]
pub struct CoopFlagNotSpecified;
#[derive(Debug, Clone, Default)]
pub struct CoopFlagSpecified(CoopFlag);
#[derive(Debug, Clone, Default)]
pub struct NoPgPool;
#[derive(Debug, Clone)]
pub struct WithPgPool(PgPool);
// endregion:   --- Builder States

#[derive(Debug, Error, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ActiveContractBuilder<I, F, P> {
    contract_id: I,
    coop_flag: F,
    pg_pool: P,
}

impl ActiveContractBuilder<NoContractId, CoopFlagNotSpecified, NoPgPool> {
    pub fn new() -> Self {
        ActiveContractBuilder::default()
    }
}

impl<I, F, P> ActiveContractBuilder<I, F, P> {
    pub fn with_contract_id(
        self,
        contract_id: impl Into<String>,
    ) -> ActiveContractBuilder<ContractId, F, P> {
        ActiveContractBuilder {
            contract_id: ContractId(contract_id.into()),
            coop_flag: self.coop_flag,
            pg_pool: self.pg_pool,
        }
    }

    pub fn with_coop_flag(
        self,
        coop_flag: CoopFlag,
    ) -> ActiveContractBuilder<I, CoopFlagSpecified, P> {
        ActiveContractBuilder {
            contract_id: self.contract_id,
            coop_flag: CoopFlagSpecified(coop_flag),
            pg_pool: self.pg_pool,
        }
    }

    pub fn with_pg_pool(self, pool: PgPool) -> ActiveContractBuilder<I, F, WithPgPool> {
        ActiveContractBuilder {
            contract_id: self.contract_id,
            coop_flag: self.coop_flag,
            pg_pool: WithPgPool(pool),
        }
    }
}

impl ActiveContractBuilder<ContractId, CoopFlagSpecified, WithPgPool> {
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
            if let Err(e) = self.update_db(contract).await {
                error!("Unable to insert details of this contract into db: {}", e);
            }

            return Ok(ActiveContract::new(
                contract.clone(),
                self.coop_flag.0,
                self.pg_pool.0,
            ));
        }

        match get_backup_contracts(&self.contract_id.0).await {
            Ok(c) => {
                if let Err(e) = self.update_db(&c).await {
                    error!("Unable to insert details of this contract into db: {}", e);
                }

                Ok(ActiveContract::new(
                    c.clone(),
                    self.coop_flag.0,
                    self.pg_pool.0,
                ))
            }
            Err(e) => Err(e),
        }
    }

    async fn update_db(&self, contract: &Contract) -> Result<PgQueryResult> {
        Ok(sqlx::query!("INSERT INTO contracts(kev_id, release_date) VALUES ($1, $2) ON CONFLICT (kev_id, release_date) DO NOTHING;",
            self.contract_id.0,
            // `start_time()` is the unix timestamp for the contract's start time in `f64` for some reason,
            // should be valid unless Kev decides to blow up,
            // and the conversion wouldn't lose precision for the next 31 million years so it's fine
            OffsetDateTime::from_unix_timestamp(contract.start_time() as i64).unwrap()
        )
        .execute(&self.pg_pool.0)
        .await?)
    }
}
