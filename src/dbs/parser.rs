use itertools::Itertools;
use std::str::FromStr;

use crate::dbs::document::Document;

pub fn parse(texts: Vec<String>) -> String {
    let documents = texts
        .into_iter()
        .map(|text| Document::from_str(&text))
        .filter_map(Result::ok)
        .collect_vec();
    let transactions = documents
        .iter()
        .flat_map(|doc| doc.transactions().into_iter())
        .collect_vec();
    serde_json::to_string_pretty(&transactions).unwrap_or_else(|_| "".to_string())
}

#[test]
fn test() {
    let s = include_str!("tmp.txt");
    let p = parse(vec![s.to_string()]);
    println!("{p}");
    assert!(!p.is_empty())
}
