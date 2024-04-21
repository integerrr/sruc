use anyhow::Result;

use leaderboard::contracts::active_contract::ActiveContractBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    let contract_id = "st-patty-2019";
    let coop_ids = vec![
        "accent470",
        "quartz938",
        "stress420",
        "relate379",
        "oyster582",
        "soccer772",
        "switch958",
        "priest958",
        "willie631",
        "holyshit",
    ];

    let mut contr = ActiveContractBuilder::new()
        .with_contract_id(contract_id)
        .build()
        .await?;






    Ok(())
}
