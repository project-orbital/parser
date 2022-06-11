use std::fmt::{Display, Formatter};
use std::str::FromStr;

use fancy_regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref TRANSACTIONS_REGEX: Regex =
        Regex::new(r"(?s)(\d{2} \w{3})(.*?)(\d+\.\d{2})(\d+\.\d{2})?(.*?)(?=$|\d{2} \w{3})")
            .unwrap();
}

pub struct Transaction {
    date: String,
    description: String,
    amount: String,
    balance: Option<String>,
}

impl FromStr for Transaction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = TRANSACTIONS_REGEX.captures(s).unwrap().unwrap();
        let description =
            captures.get(1).unwrap().as_str().to_string() + captures.get(5).unwrap().as_str();
        let description = Regex::new(r"\s+")
            .unwrap()
            .replace_all(description.as_str(), " ");
        Ok(Self {
            date: captures.get(1).unwrap().as_str().to_string(),
            description: description.to_string(),
            amount: captures.get(3).unwrap().as_str().to_string(),
            balance: captures.get(4).map(|m| m.as_str().to_string()),
        })
    }
}

impl Transaction {
    pub fn parse_all(s: &str) -> Vec<Transaction> {
        TRANSACTIONS_REGEX
            .captures_iter(s)
            .map(|m| m.unwrap().get(0).unwrap().as_str())
            .map(Transaction::from_str)
            .map(Result::unwrap)
            .collect()
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "date: {}\ndescription: {}\namount: {}\nbalance: {}",
            self.date,
            self.description,
            self.amount,
            self.balance.as_ref().unwrap_or(&"?".to_string())
        )
    }
}
