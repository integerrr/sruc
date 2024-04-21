use std::fmt::Display;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq, Eq, Error)]
pub enum CoopFlag {
    #[default]
    NoFlags,
    AnyGrade,
    Carry,
    Fastrun,
    Speedrun,
}

impl Display for CoopFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
