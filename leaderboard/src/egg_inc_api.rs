use anyhow::{Context, Error, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};

use ei::ei::custom_traits::EiApiRequest;
use ei::ei::{
    Contract, ContractCoopStatusRequest, ContractCoopStatusResponse, EggIncFirstContactRequest,
    EggIncFirstContactResponse, GetPeriodicalsRequest, PeriodicalsResponse,
};
use prost::Message;
use serde::{Deserialize, Serialize};

use crate::custom_errors::InvalidContractId;

pub async fn get_coop_status(
    contract_id: &str,
    coop_code: &str,
) -> Result<ContractCoopStatusResponse> {
    let status_request = ContractCoopStatusRequest::new(contract_id, coop_code);
    status_request.make_ei_api_request().await
}

pub async fn get_periodicals() -> Result<PeriodicalsResponse> {
    let periodicals_request = GetPeriodicalsRequest::new();
    periodicals_request.make_ei_api_request().await
}

pub async fn get_first_contact() -> Result<EggIncFirstContactResponse> {
    let first_contact_req = EggIncFirstContactRequest::new();
    first_contact_req.make_ei_api_request().await
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CarpetBackedUpContract {
    pub id: String,
    pub proto: String,
}

pub async fn get_backup_contracts(contract_id: impl Into<String>) -> Result<Contract> {
    let id = contract_id.into();
    let carpet_json = reqwest::Client::new()
        .get(
            "https://raw.githubusercontent.com/carpetsage/egg/main/periodicals/data/contracts.json",
        )
        .send()
        .await?
        .text()
        .await?;

    let deserialised_json: Vec<CarpetBackedUpContract> = serde_json::from_str(&carpet_json)?;
    let proto = deserialised_json
        .iter()
        .rev()
        .find(|&c| c.id == id)
        .ok_or_else(|| Error::from(InvalidContractId))
        .context(format!(": \"{}\"", id))?
        .clone()
        .proto;
    let decoded = BASE64
        .decode(proto)
        .context("Cannot base64 decode into byte stream")?;

    let contract = Contract::decode(decoded.as_slice()).context("Cannot decode into `Contract`")?;
    Ok(contract)
}
