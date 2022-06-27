use chrono::DateTime;
use std::fmt::{Display, Formatter};

use fancy_regex::Match;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    date: String,
    description: String,
    amount: Decimal,
    balance: Option<Decimal>,
}

impl Transaction {
    pub(crate) fn from_strs(
        start: (&str, &str),
        end: (&str, &str),
        day: &str,
        month: &str,
        desc1: &str,
        desc2: &str,
        amt: &str,
        bal: Option<Match>,
    ) -> Self {
        let desc = format!("{} {}", desc1, desc2);
        let (start_month, start_year) = start;
        let (_, end_year) = end;
        let year = if month == start_month {
            start_year
        } else {
            end_year
        };
        let date_str = format!("{}-{}-{} 00:00:00 +08:00", day, month, year);
        let date = DateTime::parse_from_str(date_str.as_str(), "%d-%b-%Y %H:%M:%S %z");
        Self {
            date: date.unwrap().to_rfc3339(),
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
