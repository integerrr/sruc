use anyhow::Result;

use leaderboard::table_builder;

#[tokio::main]
async fn main() -> Result<()> {
    let contract_id = "panama-canal-2024";
    let coop_ids = vec!["watery-poop-chute"];

    table_builder::build(contract_id, &coop_ids).await?;

    Ok(())
}
