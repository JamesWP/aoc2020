use itertools::Itertools;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::{collections::HashMap, fs};

type BagRules = HashMap<String, Vec<(i32, String)>>;

fn bag_content<'a>(mut line: impl Iterator<Item = &'a str>) -> Vec<(i32, String)> {
    let num: Option<i32> = line.next().and_then(|v| v.parse().ok());

    if num.is_some() {
        let (adj, col) = (line.next().unwrap(), line.next().unwrap());
        let bag = format!("{} {}", adj, col);

        line.next(); // ignore

        let mut recur = bag_content(line);
        recur.push((num.unwrap(), bag));
        recur
    } else {
        vec![]
    }
}

fn parse() -> BagRules {
    let contents =
        fs::read_to_string("day7.txt").expect("Something went wrong reading the file");

    let mut graph = HashMap::new();
    for line in contents.split("\n") {
        let mut line = line.split_whitespace();

        let (adj, col) = (line.next().unwrap(), line.next().unwrap());
        let bag = format!("{} {}", adj, col);

        line.next(); // ignore
        line.next(); // ignore

        graph.insert(bag.to_owned(), bag_content(line));
    }

    graph
}

fn can_contain(target_bag: &str, starting_bag: &str, rules: &BagRules) -> bool {
    rules
        .get(starting_bag)
        .unwrap()
        .iter()
        .filter(|(num, bag)| bag == target_bag || can_contain(target_bag, bag, rules))
        .take(1)
        .next()
        .is_some()
}

fn count_contents(target_bag: &str, rules: &BagRules) -> i32 {
    rules
        .get(target_bag)
        .unwrap()
        .iter()
        .map(|(count, bag)| count * count_contents(bag, rules))
        .fold(1, |acc, val| acc + val)
}

#[test]
fn test() {
    let rules = parse();
    println!("result: {:?}", rules);

    let num_bags = rules
        .iter()
        .filter(|(color, content)| can_contain("shiny gold", color, &rules))
        .count();

    println!("num_bags: {}", num_bags);

    let num_bags = count_contents("shiny gold", &rules)-1;

    println!("num_bags: {}", num_bags);
}
