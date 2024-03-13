use anyhow::Result;

use ei::ei::custom_traits::EiApiRequest;
use ei::ei::{
    ContractCoopStatusRequest, ContractCoopStatusResponse, EggIncFirstContactRequest,
    EggIncFirstContactResponse, GetPeriodicalsRequest, PeriodicalsResponse,
};

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
