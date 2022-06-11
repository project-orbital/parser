use std::fmt::{Display, Formatter};
use std::str::FromStr;

use fancy_regex::Regex;
use itertools::Itertools;
use lazy_static::lazy_static;

use crate::dbs::transactions::Transaction;

lazy_static! {
    static ref FILTERING_REGEX: Regex = Regex::new(
        r"(?s)Balance Brought Forward\s(\d+\.\d{2})(.*?)Balance Carried Forward\s(\d+\.\d{2})",
    )
    .unwrap();
}

pub struct Page {
    balanced_brought_forward: String,
    balanced_carried_forward: String,
    transactions: Vec<Transaction>,
}

impl FromStr for Page {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = FILTERING_REGEX.captures(s).unwrap().unwrap();
        Ok(Self {
            balanced_brought_forward: captures.get(1).unwrap().as_str().to_string(),
            balanced_carried_forward: captures.get(3).unwrap().as_str().to_string(),
            transactions: Transaction::parse_all(captures.get(2).unwrap().as_str()),
        })
    }
}

impl Display for Page {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "bbf: {}\n\n{}\n\nbcf: {}",
            self.balanced_brought_forward,
            self.transactions
                .iter()
                .map(Transaction::to_string)
                .join("\n\n"),
            self.balanced_carried_forward
        )
    }
}
