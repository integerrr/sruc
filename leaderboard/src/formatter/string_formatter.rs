use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Copy, Clone, Default, Error, PartialEq, Serialize, Deserialize)]
pub enum StringAlignment {
    Left,
    Centered,
    Right,
    #[default]
    None,
}

impl Display for StringAlignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn left_align(s: impl Into<String>, width: usize) -> String {
    format!("{:<width$}", s.into())
}

pub fn right_align(s: impl Into<String>, width: usize) -> String {
    format!("{:>width$}", s.into())
}

pub fn center_align(s: impl Into<String>, width: usize) -> String {
    format!("{:^width$}", s.into())
}

pub fn align(s: impl Into<String>, width: usize, alignment: StringAlignment) -> String {
    match alignment {
        StringAlignment::Left => left_align(s, width),
        StringAlignment::Centered => center_align(s, width),
        StringAlignment::Right => right_align(s, width),
        StringAlignment::None => s.into(),
    }
}
