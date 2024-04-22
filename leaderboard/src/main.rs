use anyhow::Result;

use leaderboard::contracts::active_contract::ActiveContractBuilder;
use leaderboard::contracts::coop_flag::CoopFlag;

use leaderboard::report_generator::sruc::SrucTable;
use log::info;
use sqlx::postgres::PgPoolOptions;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .init();
    let pgpool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://localhost:5432/whal-int-test")
        .await?;

    let contract_id = "eggene-2024";
    let sr_coop_ids = vec!["benson376", "better858", "morgan225"];

    let mut sr = ActiveContractBuilder::new()
        .with_contract_id(contract_id)
        .with_coop_flag(CoopFlag::Speedrun)
        .with_pg_pool(pgpool.clone())
        .build()
        .await?;

    let _ = sr.add_coops(&sr_coop_ids).await;

    let mut sr_table = SrucTable::new();
    sr_table.add_data_rows(sr.coops().as_slice());

    while !sr.all_coops_green_scrolled() {
        sr.update_all_coop_statuses().await;
        info!("Sleeping for 5 minutes...");
        sleep(Duration::from_secs_f32(300_f32)).await;
    }

    Ok(())
}
