use std::collections::HashMap;

use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use prost::Message;
use zune_inflate::DeflateDecoder;

use crate::ei::{
    AuthenticatedMessage, ContractCoopStatusRequest, ContractCoopStatusResponse,
    EggIncFirstContactRequest, EggIncFirstContactResponse, GetPeriodicalsRequest,
    PeriodicalsResponse,
};

pub trait EiApiRequest: Message + Default {
    type Response: Message + Default;
    const IS_AUTH_MSG: bool;
    const END_POINT: &'static str;

    fn make_ei_api_request(
        &self,
    ) -> impl std::future::Future<Output = Result<Self::Response>> + Send {
        async {
            let url = format!("https://www.auxbrain.com/ei/{}", Self::END_POINT);

            let mut req_body_byte_arr = vec![];
            self.encode(&mut req_body_byte_arr)?;
            let req_body_base64 = BASE64.encode(req_body_byte_arr);

            let mut request_params = HashMap::new();
            request_params.insert("data", req_body_base64);

            let client = reqwest::Client::new();
            let res = client.post(&url).form(&request_params).send().await?;

            let res_body = res.text().await?;
            let decoded_byte_arr = BASE64.decode(res_body)?;

            if !Self::IS_AUTH_MSG {
                return Ok(Self::Response::decode(decoded_byte_arr.as_slice())?);
            }

            let auth_msg = AuthenticatedMessage::decode(decoded_byte_arr.as_slice())
                .context("cannot decode into `AuthenticatedMessage`")?;
            Ok(Self::Response::decode(
                Self::parse_auth_msg(auth_msg)?.as_slice(),
            )?)
        }
    }

    fn parse_auth_msg(auth_msg: AuthenticatedMessage) -> Result<Vec<u8>> {
        if !auth_msg.compressed() {
            return Ok(auth_msg.message().to_vec());
        }
        let mut decoder = DeflateDecoder::new(auth_msg.message());
        let uncompressed = decoder
            .decode_zlib()
            .context("Error inflating `message` field of auth_msg")?;
        Ok(uncompressed)
    }
}

impl EiApiRequest for ContractCoopStatusRequest {
    type Response = ContractCoopStatusResponse;
    const IS_AUTH_MSG: bool = true;
    const END_POINT: &'static str = "coop_status";
}

impl EiApiRequest for GetPeriodicalsRequest {
    type Response = PeriodicalsResponse;
    const IS_AUTH_MSG: bool = true;
    const END_POINT: &'static str = "get_periodicals";
}

impl EiApiRequest for EggIncFirstContactRequest {
    type Response = EggIncFirstContactResponse;
    const IS_AUTH_MSG: bool = false;
    const END_POINT: &'static str = "first_contact";
}
