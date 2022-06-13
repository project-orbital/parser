use fancy_regex::Match;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::utils;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    date: String,
    description: String,
    amount: Decimal,
    balance: Option<Decimal>,
}

impl Transaction {
    pub(crate) fn from_strs(
        date: &str,
        desc1: &str,
        desc2: &str,
        amt: &str,
        bal: Option<Match>,
    ) -> Self {
        let desc = format!("{} {}", desc1, desc2);
        Self {
            date: date.to_string(),
            description: utils::truncate_whitespace(&desc),
            amount: Decimal::from_str(amt).unwrap(),
            balance: bal.map(|m| Decimal::from_str(m.as_str()).unwrap()),
        }
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_else(|_| "".to_string())
        )
    }
}
