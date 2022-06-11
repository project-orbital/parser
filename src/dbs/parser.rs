use std::str::FromStr;

use crate::dbs::document::Document;

pub fn parse(text: &str) -> String {
    Document::from_str(text).unwrap().to_string()
}

#[test]
fn test() {
    let s = include_str!("tmp.txt");
    let p = parse(s);
    println!("{p}");
    assert!(!p.is_empty())
}
