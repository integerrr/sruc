use anyhow::Result;

use sruc::table_builder;

#[tokio::main]
async fn main() -> Result<()> {
    let contract_id = "fday-ouch";
    let coop_ids = vec!["voyage483", "winter159", "bottom835"];

    table_builder::build(contract_id, &coop_ids).await?;

    Ok(())
}
