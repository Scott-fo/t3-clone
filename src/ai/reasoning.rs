use anyhow::{Error, Result, bail};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EffortLevel {
    Low,
    Medium,
    High,
}

impl ToString for EffortLevel {
    fn to_string(&self) -> String {
        match self {
            EffortLevel::Low => "low".to_string(),
            EffortLevel::Medium => "medium".to_string(),
            EffortLevel::High => "high".to_string(),
        }
    }
}

impl FromStr for EffortLevel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "high" => Ok(EffortLevel::High),
            "low" => Ok(EffortLevel::Low),
            "medium" => Ok(EffortLevel::Medium),
            _ => bail!("Invalid effort level: '{}'", s),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Reasoning {
    pub effort: EffortLevel,
    pub summary: String,
}

impl Reasoning {
    pub fn new(effort: EffortLevel) -> Reasoning {
        Reasoning {
            effort,
            summary: "detailed".to_string(),
        }
    }
}
