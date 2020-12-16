use itertools::Itertools;
use regex::Regex;
use std::{
    collections::HashMap, collections::HashSet, convert::TryFrom, convert::TryInto, fs,
    ops::RangeInclusive,
};

fn parse(
    filename: &str,
) -> (
    Vec<(String, RangeInclusive<i32>, RangeInclusive<i32>)>,
    Vec<i32>,
    Vec<Vec<i32>>,
) {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let (rules, my_ticket, other_tickets) = contents.split("\n\n").next_tuple().unwrap();

    let re: Regex = Regex::new(r"^([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();

    let rules: Vec<_> = rules
        .split("\n")
        .map(|rule| {
            let captures = re.captures(rule).unwrap();
            let name = &captures[1];
            let r1b = &captures[2].parse::<i32>().unwrap();
            let r1e = &captures[3].parse::<i32>().unwrap();
            let r2b = &captures[4].parse::<i32>().unwrap();
            let r2e = &captures[5].parse::<i32>().unwrap();

            let r1 = *r1b..=*r1e;
            let r2 = *r2b..=*r2e;
            //println!("rule: {}. {:?} or {:?}", name, r1,r2);

            (name.to_owned(), r1, r2)
        })
        .collect();

    let my_ticket: Vec<_> = my_ticket
        .split("\n")
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect();

    let other_tickets: Vec<Vec<_>> = other_tickets
        .split("\n")
        .skip(1)
        .map(|ticket| {
            ticket
                .split(",")
                .map(str::parse::<i32>)
                .map(Result::unwrap)
                .collect()
        })
        .collect();

    //println!("rules: {:?}", rules);
    //println!("my_ticket: {:?}", my_ticket);
    //println!("other_tickets: {:?}", other_tickets);
    (rules, my_ticket, other_tickets)
}

#[test]
fn test_parse() {
    let (rules, _, others) = parse("day16test.txt");

    let errors: i32 = others
        .iter()
        .map(|ticket| -> i32 {
            let ticket_bad_values: Vec<_> = ticket
                .iter()
                .cloned()
                .filter(|num| {
                    rules
                        .iter()
                        .filter(|(_name, r1, r2)| r1.contains(num) || r2.contains(num))
                        .next()
                        .is_none()
                })
                .collect();
            // println!("bad_values: {:?}", ticket_bad_values);
            ticket_bad_values.iter().sum()
        })
        .sum();

    println!("errors: {:?}", errors);

    let (rules, _, others) = parse("day16.txt");

    let valid_tickets: Vec<Vec<_>> = others
        .iter()
        .cloned()
        .filter(|ticket| {
            ticket
                .iter()
                .cloned()
                .filter(|num| {
                    rules
                        .iter()
                        .filter(|(name, r1, r2)| r1.contains(num) || r2.contains(num))
                        .next()
                        .is_none()
                })
                .next()
                .is_none()
        })
        .collect();

    println!("valid_tickets: {:?}", valid_tickets);

    let num_fields = rules.len();

    for rule_idx in 0..num_fields {
        let rule = &rules[rule_idx];
        let (name, r1, r2) = rule;

        println!("rule: {:?}", name);

        let possibilities = (0..num_fields)
            .map(|field_idx| {
                //println!("field idx: {}", field_idx);
                let values = valid_tickets
                    .iter()
                    .map(|ticket| ticket[field_idx])
                    .collect::<HashSet<_>>();
                let num_invalid = values
                    .iter()
                    .filter(|val| !r1.contains(val) && !r2.contains(val))
                    .count();
                //println!("values: {:?}", values);
                //println!("num invalid values: {:?}", num_invalid);
                if num_invalid == 0 {
                    0
                } else {
                    1
                }
            })
            .collect::<Vec<_>>();

        println!("possibilities: {:?}", possibilities);
    }
}
