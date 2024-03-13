use anyhow::Result;
use leaderboard::contracts::active_contract::ActiveContractBuilder;
use leaderboard::contracts::coop::Coop;
use leaderboard::formatter::discord_table::{DiscordTable, DiscordTableColumn};
use leaderboard::formatter::string_formatter::StringAlignment;

#[tokio::main]
async fn main() -> Result<()> {
    let contract_id = "heavy-regulation-2024";
    let coop_ids = vec!["anyape849"];

    let mut contr = ActiveContractBuilder::new()
        .contract_id(contract_id)
        .build()
        .await?;

    for id in coop_ids {
        contr.add_coop(id).await?;
    }

    let mut table: DiscordTable<Coop> = DiscordTable::new();
    let col1 = DiscordTableColumn::new(
        "Coop name",
        |coop: Coop| coop.coop_id().to_string(),
        13,
        StringAlignment::Centered,
    );
    let col2 = DiscordTableColumn::new(
        "Duration",
        |coop: Coop| coop.total_predicted_duration().format(),
        30,
        StringAlignment::Centered,
    );

    table.add_column(col1);
    table.add_column(col2);
    for coop in contr.coops() {
        table.add_data_row(coop.clone());
    }

    println!("{}", table.get_table_header());
    println!("{}", table.get_table_body());

    Ok(())
}
