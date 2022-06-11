use std::fmt::{Display, Formatter};

use fancy_regex::{Match, Regex};
use lazy_static::lazy_static;
use rusty_money::{iso, Money};

lazy_static! {
    static ref RE: Regex = Regex::new(r"\s+",).unwrap();
}

pub struct Transaction<'a> {
    date: String,
    description: String,
    amount: Money<'a, iso::Currency>,
    balance: Option<Money<'a, iso::Currency>>,
}

impl Transaction<'_> {
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
            description: RE.replace_all(&desc, " ").trim().to_string(),
            amount: Money::from_str(amt, iso::SGD).unwrap(),
            balance: bal.map(|m| Money::from_str(m.as_str(), iso::SGD).unwrap()),
        }
    }
}

impl Display for Transaction<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "date: {}\ndescription: {}\namount: {}\nbalance: {}",
            self.date,
            self.description,
            self.amount,
            self.balance
                .as_ref()
                .map(|bal| bal.to_string())
                .unwrap_or_else(|| "?".to_string())
        )
    }
}
