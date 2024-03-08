use crate::contracts::ActiveContract;

pub mod discord_formatter;
pub mod sruc_table_formatter;

pub trait ContractFormatter {
    fn format(contract: &ActiveContract) -> String;
}
