use std::collections::HashMap;

use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use prost::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RequestEndpoint {
    CoopStatus,
    FirstContact,
    GetPeriodicals,
}

pub async fn ei_post(req: impl Message, endpoint: RequestEndpoint) -> Result<Vec<u8>> {
    let end_point = match endpoint {
        RequestEndpoint::CoopStatus => "coop_status",
        RequestEndpoint::FirstContact => "first_contact",
        RequestEndpoint::GetPeriodicals => "get_periodicals",
    };
    let url = format!("https://www.auxbrain.com/ei/{}", end_point);

    let mut req_body_byte_arr = vec![];
    req.encode(&mut req_body_byte_arr)
        .context("Buffer insufficient capacity?")?;
    let req_body_base64 = BASE64.encode(req_body_byte_arr.clone());
    let mut params = HashMap::new();
    params.insert("data", req_body_base64);

    let client = reqwest::Client::new();

    let res = client.post(&url).form(&params).send().await?;

    let res_body = res.text().await?;
    let decoded_byte_arr = BASE64
        .decode(res_body)
        .context("failed to decode base64 string into a Vec<u8>")?;

    Ok(decoded_byte_arr)
}
