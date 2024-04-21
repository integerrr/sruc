use anyhow::Result;

use leaderboard::contracts::active_contract::ActiveContractBuilder;
use leaderboard::contracts::coop_flag::CoopFlag;

use leaderboard::report_generator::sruc::SrucTable;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env("RUST_LOG");
    let pgpool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://localhost:5432/whal-int-test")
        .await?;

    let contract_id = "Luncheggbles-2022";
    let coop_ids = vec!["regime813", "father747", "degree267", "holyshit"];

    let mut fr = ActiveContractBuilder::new()
        .with_contract_id(contract_id)
        .with_coop_flag(CoopFlag::Fastrun)
        .with_pg_pool(pgpool.clone())
        .build()
        .await?;

    let _ = fr.add_coops(&coop_ids).await;

    let mut sr_table = SrucTable::new();
    sr_table.add_data_rows(fr.coops().as_slice());

    Ok(())
}
