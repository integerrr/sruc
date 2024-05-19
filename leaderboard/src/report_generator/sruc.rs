use std::ops::{Deref, DerefMut};

use crate::{
    contracts::coop::Coop,
    formatter::{
        discord_table::{DiscordTable, DiscordTableColumn},
        discord_timestamp::{DiscordTimestamp, DiscordTimestampDisplay},
        string_formatter::StringAlignment,
    },
};

#[derive(Default)]
pub struct SrucTable(DiscordTable<Coop>);

impl SrucTable {
    pub fn new() -> Self {
        let mut table = DiscordTable::new();

        let name_col = DiscordTableColumn::new(
            "Coop",
            |c: Coop| {
                format!(
                    "[\u{29c9}](<https://eicoop-carpet.netlify.app/{}/{}>) `{}",
                    c.contract_id(),
                    c.coop_id(),
                    c.stripped_coop_id()
                )
            },
            8,
            StringAlignment::Centered,
        );
        let boosted_col = DiscordTableColumn::new(
            "Boosted",
            |c: Coop| c.boosted_count().to_string(),
            9,
            StringAlignment::Centered,
        );
        let token_col = DiscordTableColumn::new(
            "Tokens",
            |c: Coop| c.total_tokens().to_string(),
            8,
            StringAlignment::Centered,
        );
        let dur_col = DiscordTableColumn::new(
            "Duration",
            |c: Coop| c.total_predicted_duration().format_too_long(),
            10,
            StringAlignment::Centered,
        );
        let finish_col = DiscordTableColumn::new(
            "Finish",
            |c: Coop| {
                format!(
                    "`{}",
                    c.finishing_time()
                        .display(DiscordTimestampDisplay::FullDateTime)
                )
            },
            20,
            StringAlignment::Centered,
        );

        table.add_column(name_col);
        table.add_column(boosted_col);
        table.add_column(token_col);
        table.add_column(dur_col);
        table.add_column(finish_col);

        Self(table)
    }

    pub fn generate(self) {
        println!(
            "Last updated: {}\n\
            \n\
            {}\n\
            {}\
            `Primary order based off of duration`\n\
            \n\
            *`!!sruc` to summon an update!*\n\
            *Note that this is NOT a Wonky command, and is still generated by WHAL-Int-rs*\n\
            \n\
            \n",
            DiscordTimestamp::new_from_now().display(DiscordTimestampDisplay::Relative),
            self.get_table_header(),
            self.get_table_body(),
        );
    }
}

impl Deref for SrucTable {
    type Target = DiscordTable<Coop>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SrucTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
