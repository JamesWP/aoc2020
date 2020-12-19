use std::{collections::HashMap, fs};
use itertools::Itertools;
use regex::Regex;

fn create_regex(rules: &str) -> Regex {
    let rule_map: HashMap<i32, &str> = rules.split("\n").map(|rule| {
        let (rule_number, rule_str) = rule.split(":").collect_tuple().unwrap();
        let rule_number:i32 = rule_number.parse().unwrap();
        (rule_number, rule_str.trim())
    }).collect();
    let compute_regex = |rule| {
        "abc".to_owned()
    };
    let regex = compute_regex(0);
    println!("rule_map: {:?}", rule_map);
    Regex::new(&regex).unwrap()
}
#[test]
fn test() {
    let contents = fs::read_to_string("day19test.txt").expect("Something went wrong reading the file");

    let (rules, input) = contents.split("\n\n").collect_tuple().unwrap();

    let regex = create_regex(rules);
}