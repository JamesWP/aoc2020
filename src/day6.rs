use std::{collections::HashSet, fs};

#[test]
fn parse() {
    let contents = fs::read_to_string("day6.txt").expect("Something went wrong reading the file");

    let count_sum: usize = contents
        .split("\n\n")
        .map(|group| {
            let answer_set: HashSet<char> = group.chars().filter(|c| !c.is_whitespace()).collect();

            answer_set.len()
        })
        .sum();

    println!("count sum {}", count_sum);

    let count_sum: usize = contents
        .split("\n\n")
        .map(|group| {
            group
                .split_whitespace()
                .map(|person| person.chars().collect())
                .fold(('a'..='z').collect::<HashSet<char>>(), |acc, val| {
                    acc.intersection(&val).copied().collect()
                })
                .len()
        })
        .sum();

    println!("count sum {}", count_sum);
}
