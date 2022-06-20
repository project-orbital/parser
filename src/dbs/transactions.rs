use fancy_regex::Match;
use std::fmt::{Display, Formatter};

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
            description: utils::redact_card_numbers(utils::truncate_whitespace(&desc).as_str()),
            amount: utils::parse_monetary_value(amt),
            balance: bal.map(|m| utils::parse_monetary_value(m.as_str())),
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
