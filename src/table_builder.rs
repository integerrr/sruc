use anyhow::{Context, Ok, Result};
use prost::Message;
use zune_inflate::DeflateDecoder;

use crate::{
    ei::{
        AuthenticatedMessage, Contract, ContractCoopStatusRequest,
        ContractCoopStatusResponse, GetPeriodicalsRequest, PeriodicalsResponse,
    },
    ei_request,
    ei_struct::MajCoop,
};

pub async fn build(contract_id: &str, coop_ids: &[&str]) -> Result<()> {
    let mut sr_coops = vec![];

    let contract_obj = get_contract_struct(contract_id).await?;

    for coop_id in coop_ids {
        let req = ContractCoopStatusRequest::new(contract_id, *coop_id);
        let decoded_byte_arr =
            ei_request::ei_post(req, ei_request::RequestEndpoint::CoopStatus).await?;
        let auth_msg = AuthenticatedMessage::decode(decoded_byte_arr.as_slice())
            .context("Cannot decode into an `AuthenticatedMessage`")?;
        let coop_obj = ContractCoopStatusResponse::decode(auth_msg.message())
            .context("Cannot decode into a `ContractCoopStatusResponse`")?;

        let coop = MajCoop::new(contract_obj.clone(), coop_obj);
        sr_coops.push(coop);
    }

    print_sruc(sr_coops)?;
    Ok(())
}

async fn get_contract_struct(contract_id: &str) -> Result<Contract> {
    let req = GetPeriodicalsRequest::new();
    let decoded_byte_arr =
        ei_request::ei_post(req, ei_request::RequestEndpoint::GetPeriodicals).await?;
    let auth_msg = AuthenticatedMessage::decode(decoded_byte_arr.as_slice())
        .context("Cannot decode into an `AuthenticatedMessage`")?;
    let mut decoder = DeflateDecoder::new(auth_msg.message());
    let uncomped = decoder
        .decode_zlib()
        .context("Error inflating `message` field of the auth_msg")?;
    let periodicals_resp = PeriodicalsResponse::decode(uncomped.as_slice())
        .context("Cannot decode into `PeriodicalResponse`")?;
    let contract_resp = periodicals_resp.contracts.unwrap_or_default();
    let contract = contract_resp
        .contracts
        .iter()
        .find(|contract| contract.identifier() == contract_id)
        .expect("This contract should exist")
        .to_owned();

    Ok(contract)
}

fn print_sruc(coops: Vec<MajCoop>) -> Result<()> {
    println!("`  Coop   | Boosted | Tokens | Duration  | Finish`");

    for coop in coops {
        println!("{}", coop.build_table_row());
    }

    Ok(())
}
