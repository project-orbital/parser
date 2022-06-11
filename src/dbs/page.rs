use std::fmt::{Display, Formatter};
use std::str::FromStr;

use fancy_regex::Regex;
use lazy_static::lazy_static;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::dbs::transactions::Transaction;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(?s)(\d{2} \w{3})(.*?)(\d+\.\d{2})(\d+\.\d{2})?(.*?)(?=$|\d{2} \w{3})")
            .unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    balanced_brought_forward: Decimal,
    balanced_carried_forward: Decimal,
    transactions: Vec<Transaction>,
}

impl Page {
    pub(crate) fn from_strs(bbf: &str, bcf: &str, body: &str) -> Self {
        Self {
            balanced_brought_forward: Decimal::from_str(bbf).unwrap(),
            balanced_carried_forward: Decimal::from_str(bcf).unwrap(),
            transactions: RE
                .captures_iter(body)
                .filter_map(Result::ok)
                .map(|cap| Transaction::from_strs(&cap[1], &cap[2], &cap[5], &cap[3], cap.get(4)))
                .collect(),
        }
    }

    pub(crate) fn transactions(&self) -> Vec<&Transaction> {
        self.transactions.iter().collect()
    }
}

impl Display for Page {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_else(|_| "".to_string())
        )
    }
}
