use crate::ei;

const CURRENT_CLIENT_VERSION: u32 = 999;
const CLIENT_VERSION: u32 = 62;
const VERSION: &str = "1.29.1";
const BUILD: &str = "111279";
const PLATFORM: &str = "IOS";
const EID: &str = "";

fn basic_request_info_builder() -> ei::BasicRequestInfo {
    ei::BasicRequestInfo {
        ei_user_id: Some(EID.into()),
        client_version: Some(CLIENT_VERSION),
        version: Some(VERSION.into()),
        build: Some(BUILD.into()),
        platform: Some(PLATFORM.into()),
        ..Default::default()
    }
}

pub fn contract_coop_status_request_builder(
    contract_id: impl Into<String>,
    coop_id: impl Into<String>,
) -> ei::ContractCoopStatusRequest {
    let rinfo = basic_request_info_builder();

    ei::ContractCoopStatusRequest {
        rinfo: Some(rinfo.clone()),
        contract_identifier: Some(contract_id.into()),
        coop_identifier: Some(coop_id.into()),
        user_id: Some(rinfo.ei_user_id().into()),
        client_version: Some(rinfo.client_version()),
    }
}

pub fn get_periodicals_request_builder() -> ei::GetPeriodicalsRequest {
    let rinfo = basic_request_info_builder();

    ei::GetPeriodicalsRequest {
        rinfo: Some(rinfo.clone()),
        user_id: Some(rinfo.ei_user_id().to_string()),
        current_client_version: Some(CURRENT_CLIENT_VERSION),
        ..Default::default()
    }
}
