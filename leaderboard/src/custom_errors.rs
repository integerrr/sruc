use std::fmt::{Display, Formatter};

use thiserror::Error;

#[derive(Debug, Copy, Clone, Error)]
pub struct InvalidCoopCode;

impl Display for InvalidCoopCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid coop code used")
    }
}

#[derive(Debug, Clone, Copy, Error)]
pub struct InvalidContractId;

impl Display for InvalidContractId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid contract ID used")
    }
}
