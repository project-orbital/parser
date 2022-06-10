use std::fmt::{Display, Formatter};
use std::str::FromStr;

use fancy_regex::Regex;
use itertools::Itertools;
use lazy_static::lazy_static;

lazy_static! {
    static ref FILTERING_REGEX: Regex = Regex::new(
        r"(?s)Balance Brought Forward\s(\d+\.\d{2})(.*?)Balance Carried Forward\s(\d+\.\d{2})",
    )
    .unwrap();
    static ref TRANSACTIONS_REGEX: Regex =
        Regex::new(r"(?s)(\d{2} \w{3})(.*?)(\d+\.\d{2})(\d+\.\d{2})?(.*?)(?=$|\d{2} \w{3})")
            .unwrap();
}

struct Transaction {
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
    fn parse_all(s: &str) -> Vec<Transaction> {
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

struct Page {
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

struct Document {
    pages: Vec<Page>,
}

impl FromStr for Document {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            pages: FILTERING_REGEX
                .captures_iter(s)
                .map(|m| m.unwrap().get(0).unwrap().as_str())
                .map(Page::from_str)
                .map(Result::unwrap)
                .collect(),
        })
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pages.iter().map(Page::to_string).join("\n\n"))
    }
}

pub fn parse(text: &str) -> String {
    Document::from_str(text).unwrap().to_string()
}

#[test]
fn test() {
    let string = parse(include!("./tmp.txt"));
    println!("{string}");
    assert!(!string.is_empty())
}
