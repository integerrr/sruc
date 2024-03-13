use dotenvy_macro::dotenv;

pub mod ei;

const CURRENT_CLIENT_VERSION: u32 = 999;
const CLIENT_VERSION: u32 = 64;
const VERSION: &str = "1.31";
const BUILD: &str = "111284";
const PLATFORM: &str = "IOS";
const EID: &str = dotenv!("EID");
