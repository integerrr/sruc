use dotenvy_macro::dotenv;
use ei::{contract, BasicRequestInfo, ContractCoopStatusRequest, GetPeriodicalsRequest};

pub mod ei {
    include!(concat!(env!("OUT_DIR"), "/ei.rs"));
}

const CURRENT_CLIENT_VERSION: u32 = 999;
const CLIENT_VERSION: u32 = 62;
const VERSION: &str = "1.29.1";
const BUILD: &str = "111279";
const PLATFORM: &str = "IOS";
const EID: &'static str = dotenv!("EID");

impl BasicRequestInfo {
    pub fn new() -> Self {
        Self {
            ei_user_id: Some(EID.into()),
            client_version: Some(CLIENT_VERSION),
            version: Some(VERSION.into()),
            build: Some(BUILD.into()),
            platform: Some(PLATFORM.into()),
            ..Default::default()
        }
    }
}

impl ContractCoopStatusRequest {
    pub fn new(contract_id: impl Into<String>, coop_id: impl Into<String>) -> Self {
        let rinfo = BasicRequestInfo::new();

        Self {
            rinfo: Some(rinfo.clone()),
            contract_identifier: Some(contract_id.into()),
            coop_identifier: Some(coop_id.into()),
            user_id: Some(rinfo.ei_user_id().into()),
            client_version: Some(rinfo.client_version()),
        }
    }
}

impl GetPeriodicalsRequest {
    pub fn new() -> Self {
        let rinfo = BasicRequestInfo::new();

        Self {
            rinfo: Some(rinfo.clone()),
            user_id: Some(rinfo.ei_user_id().to_string()),
            current_client_version: Some(CURRENT_CLIENT_VERSION),
            ..Default::default()
        }
    }
}

impl std::fmt::Display for contract::PlayerGrade {
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
