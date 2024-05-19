use anyhow::Result;

use leaderboard::api::get_periodicals;
use leaderboard::contracts::active_contract::ActiveContractBuilder;
use leaderboard::contracts::coop_flag::CoopFlag;
use leaderboard::error;
use leaderboard::report_generator::sruc::SrucTable;
use time::OffsetDateTime;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .init();

    let kev_open_contracts = get_periodicals()
        .await?
        .contracts
        .ok_or(error::EmptyContractsResponse)?
        .contracts;

    let most_recent_contract_codes: Vec<_> = kev_open_contracts
        .iter()
        .filter(|&c| {
            OffsetDateTime::now_utc().unix_timestamp() - c.start_time() as i64 <= 259_200i64
        })
        .map(|c| c.identifier())
        .collect();

    for contract_id in most_recent_contract_codes {
        let mut sr = ActiveContractBuilder::new()
            .with_contract_id(contract_id)
            .with_coop_flag(CoopFlag::Speedrun)
            .build()
            .await?;

        let _ = sr.fill_coops().await;

        let mut sr_table = SrucTable::new();
        sr_table.add_data_rows(sr.coops().as_slice());
        println!("# {} | Speedrun Leaderboard", sr.contract_name());
        sr_table.generate();
    }

    Ok(())
}
