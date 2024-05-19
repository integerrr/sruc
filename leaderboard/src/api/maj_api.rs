use crate::{contracts::coop_flag::CoopFlag, error};
use anyhow::Result;
use dotenvy_macro::dotenv;
use serde::Deserialize;

const MAJ_COOPS_API_URL: &str = dotenv!("MAJ_API");

#[derive(Debug, Clone, Deserialize)]
pub struct MajCoopsResponse {
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "activeCoops")]
    pub active_coops: bool,
    pub coops: Vec<MajCoops>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MajCoops {
    #[serde(rename = "coopFlags")]
    pub coop_flags: CoopFlag,
    pub code: String,
}

pub async fn get_maj_api_response(contract_id: impl Into<String>) -> Result<MajCoopsResponse> {
    let url = format!("{}?contract={}", MAJ_COOPS_API_URL, contract_id.into());
    let maj_coop_json = reqwest::Client::new()
        .get(url)
        .header(
            "User-Agent",
            "Rust backend test bot by @integerrrr on discord",
        )
        .send()
        .await?
        .text()
        .await?;

    let maj_res: Vec<MajCoopsResponse> = serde_json::from_str(&maj_coop_json)?;
    let maj_res = maj_res
        // because we're only ever supplying 1 contract ID at a time
        // trying to keep things simple here
        .first()
        .cloned()
        .ok_or(error::EmptyMajResponse)?;
    Ok(maj_res)
}

pub async fn get_maj_active_coop_codes(
    contract_id: impl Into<String>,
    coop_flag: CoopFlag,
) -> Result<Vec<String>> {
    let res = get_maj_api_response(contract_id).await?;
    if !res.active_coops {
        return Ok(Vec::new());
    }
    Ok(res
        .coops
        .iter()
        .filter(|&c| c.coop_flags == coop_flag)
        .map(|c| c.code.clone())
        .collect())
}
