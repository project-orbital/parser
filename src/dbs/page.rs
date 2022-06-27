use std::fmt::{Display, Formatter};

use fancy_regex::Regex;
use lazy_static::lazy_static;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::dbs::transactions::Transaction;
use crate::utils;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(?sm)^(?-m)(\d{2}) (\w{3})(.*?)(\d{1,3}(?:\,\d{3})*\.\d{2})(\d{1,3}(?:\,\d{3})*\.\d{2})?(.*?)(?=$|\d{2} \w{3})")
            .unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    balanced_brought_forward: Decimal,
    balanced_carried_forward: Decimal,
    transactions: Vec<Transaction>,
}

impl Page {
    pub(crate) fn from_strs(
        start: (&str, &str),
        end: (&str, &str),
        bbf: &str,
        bcf: &str,
        body: &str,
    ) -> Self {
        Self {
            balanced_brought_forward: utils::parse_monetary_value(bbf),
            balanced_carried_forward: utils::parse_monetary_value(bcf),
            transactions: RE
                .captures_iter(body)
                .filter_map(Result::ok)
                .map(|cap| {
                    Transaction::from_strs(
                        start,
                        end,
                        &cap[1],
                        &cap[2],
                        &cap[3],
                        &cap[6],
                        &cap[4],
                        cap.get(5),
                    )
                })
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
