use std::fmt::Display;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, Default, Serialize, PartialEq, Eq, Error)]
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

impl<'de> Deserialize<'de> for CoopFlag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CoopFlagVisitor;

        impl<'de> serde::de::Visitor<'de> for CoopFlagVisitor {
            type Value = CoopFlag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("coop flag variant")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut any_grade = false;
                let mut carry = false;
                let mut fast_run = false;
                let mut speed_run = false;

                while let Some(key) = map.next_key::<&str>()? {
                    match key {
                        "anyGrade" => any_grade = map.next_value()?,
                        "carry" => carry = map.next_value()?,
                        "fastRun" => fast_run = map.next_value()?,
                        "speedRun" => speed_run = map.next_value()?,
                        _ => {}
                    }
                }

                if speed_run {
                    Ok(CoopFlag::Speedrun)
                } else if fast_run {
                    Ok(CoopFlag::Fastrun)
                } else if carry {
                    Ok(CoopFlag::Carry)
                } else if any_grade {
                    Ok(CoopFlag::AnyGrade)
                } else {
                    Ok(CoopFlag::NoFlags)
                }
            }
        }
        deserializer.deserialize_map(CoopFlagVisitor)
    }
}
