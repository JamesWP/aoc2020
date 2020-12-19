use std::{collections::HashMap, fs};
use itertools::Itertools;
use regex::Regex;


fn compute_regex<'a>(rule: i32, rule_map: &HashMap<i32, &'a str>) -> String {
    if rule == 8 {
        let rule = compute_regex(42, rule_map);
        return format!("(({})+)", rule);
    } else if rule==11 {
        let rule_42 = compute_regex(42, rule_map);
        let rule_31 = compute_regex(31, rule_map);
        let combined = (1..=5).map(|num|{
            format!("{0}{{{2}}}{1}{{{2}}}", rule_42, rule_31, num)
        }).join("|");
        return format!("({})", combined);
    }

    let rule = rule_map.get(&rule).unwrap();
    if rule.contains("\"") {
        rule.trim_matches('"').to_owned()
    } else {
        let alternation = rule.split("|").map(|part| {
            let concat = part.trim().split(" ").map(str::parse::<i32>).map(Result::unwrap).map(|sub_rule| compute_regex(sub_rule, rule_map)).join("");
            concat
        }).join("|");

        if alternation.contains("|") {
            format!("({})", alternation)
        } else {
            alternation
        }
    }
}

fn create_regex(rules: &str, start: i32) -> Regex {
    let mut rule_map: HashMap<i32, &str> = rules.split("\n").map(|rule| {
        let (rule_number, rule_str) = rule.split(":").collect_tuple().unwrap();
        let rule_number:i32 = rule_number.parse().unwrap();
        (rule_number, rule_str.trim())
    }).collect();
    let regex = compute_regex(start, &rule_map);
    let regex = format!("^{}$", regex);
    Regex::new(&regex).unwrap()
}

#[test]
fn test_2() {
    let contents = fs::read_to_string("day19.txt").expect("Something went wrong reading the file");

    let (rules, input) = contents.split("\n\n").collect_tuple().unwrap();

    let regex = create_regex(rules, 8);
    let regex = create_regex(rules, 11);
}
// 360 high
#[test]
fn test() {
    let contents = fs::read_to_string("day19.txt").expect("Something went wrong reading the file");

    let (rules, input) = contents.split("\n\n").collect_tuple().unwrap();

    let regex = create_regex(rules, 0);

    let match_count = input.split("\n").filter(|line|regex.is_match(line)).map(|mtch| {println!("match: {}", mtch); mtch}).count();

    println!("count: {}", match_count);
}