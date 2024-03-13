pub mod discord_table;
pub mod discord_timestamp;
pub mod duration;
pub mod string_formatter;

pub trait ContractFormatter {
    fn format() -> String;
}
