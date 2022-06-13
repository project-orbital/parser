use std::str::FromStr;

use itertools::Itertools;

use crate::dbs::document::Document;
use crate::utils;

pub fn parse(texts: Vec<String>) -> String {
    let documents = texts
        .into_iter()
        .map(|text| Document::from_str(utils::redact_card_numbers(&text).as_str()))
        .filter_map(Result::ok)
        .collect_vec();
    let transactions = documents
        .iter()
        .flat_map(|doc| doc.transactions().into_iter())
        .collect_vec();
    serde_json::to_string_pretty(&transactions).unwrap_or_else(|_| "".to_string())
}

#[cfg(test)]
mod tests {
    use crate::dbs::document::Document;
    use crate::parser::parse;
    use crate::utils;
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case("src/dbs/test1.txt"; "test1.txt")]
    #[test_case("src/dbs/test2.txt"; "test2.txt")]
    fn test_parses_document(path: &str) {
        let data = utils::read_to_string(path);
        let document = Document::from_str(data.as_str());
        assert!(document.is_ok());
    }

    #[test_case("src/dbs/test1.txt", 27; "test1.txt")]
    #[test_case("src/dbs/test2.txt", 18; "test2.txt")]
    fn test_parses_all_transactions(path: &str, expected: usize) {
        let data = utils::read_to_string(path);
        let document = Document::from_str(data.as_str()).unwrap();
        let transactions = document.transactions();
        assert_eq!(transactions.len(), expected);
    }

    #[test_case("src/dbs/test1.txt"; "test1.txt")]
    #[test_case("src/dbs/test2.txt"; "test2.txt")]
    fn test_parses_one_to_json(path: &str) {
        let data = utils::read_to_string(path);
        let json = parse(vec![data]);
        assert!(!json.is_empty());
        assert_ne!(json, "[]");
    }
}
