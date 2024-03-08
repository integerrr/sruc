use anyhow::Result;

use leaderboard::table_builder;

#[tokio::main]
async fn main() -> Result<()> {
    let contract_id = "heavy-regulation-2024";
    let coop_ids = vec!["anyape849"];

    table_builder::build(contract_id, &coop_ids).await?;

    Ok(())
}
