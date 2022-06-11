use std::fmt::{Display, Formatter};

use fancy_regex::Regex;
use itertools::Itertools;
use lazy_static::lazy_static;
use rusty_money::{iso, Money};

use crate::dbs::transactions::Transaction;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(?s)(\d{2} \w{3})(.*?)(\d+\.\d{2})(\d+\.\d{2})?(.*?)(?=$|\d{2} \w{3})")
            .unwrap();
}

pub struct Page<'a> {
    balanced_brought_forward: Money<'a, iso::Currency>,
    balanced_carried_forward: Money<'a, iso::Currency>,
    transactions: Vec<Transaction<'a>>,
}

impl Page<'_> {
    pub(crate) fn from_strs(bbf: &str, bcf: &str, body: &str) -> Self {
        Self {
            balanced_brought_forward: Money::from_str(bbf, iso::SGD).unwrap(),
            balanced_carried_forward: Money::from_str(bcf, iso::SGD).unwrap(),
            transactions: RE
                .captures_iter(body)
                .filter_map(Result::ok)
                .map(|cap| Transaction::from_strs(&cap[1], &cap[2], &cap[5], &cap[3], cap.get(4)))
                .collect(),
        }
    }
}

impl Display for Page<'_> {
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
