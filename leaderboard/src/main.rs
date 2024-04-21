use anyhow::Result;

use leaderboard::contracts::active_contract::ActiveContractBuilder;
use leaderboard::contracts::coop_flag::CoopFlag;

use leaderboard::report_generator::sruc::SrucTable;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env("RUST_LOG").init();
    let pgpool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://localhost:5432/whal-int-test")
        .await?;

    let ultra_contract_id = "Luncheggbles-2022";
    let fr_coop_ids = vec!["regime813", "father747", "degree267", "holyshit"];
    let usr_coop_ids = vec!["manila784"];

    let mut fr = ActiveContractBuilder::new()
        .with_contract_id(ultra_contract_id)
        .with_coop_flag(CoopFlag::Fastrun)
        .with_pg_pool(pgpool.clone())
        .build()
        .await?;

    let mut usr = ActiveContractBuilder::new()
        .with_contract_id(ultra_contract_id)
        .with_coop_flag(CoopFlag::Speedrun)
        .with_pg_pool(pgpool.clone())
        .build()
        .await?;

    let _ = fr.add_coops(&fr_coop_ids).await;
    let _ = usr.add_coops(&usr_coop_ids).await;

    let mut fr_table = SrucTable::new();
    fr_table.add_data_rows(fr.coops().as_slice());

    let mut usr_table = SrucTable::new();
    usr_table.add_data_rows(usr.coops().as_slice());

    Ok(())
}
