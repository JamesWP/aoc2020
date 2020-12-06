use std::{collections::BTreeMap, fs};

fn seating(pass: &str) -> i32 {
    pass.chars()
        .map(|ch| match ch {
            'F' => 0,
            'B' => 1,
            'L' => 0,
            'R' => 1,
            _ => panic!("bad input '{}'", ch),
        })
        .fold(0, |acc, val| (acc << 1) | val)
}

#[test]
fn examples() {
    assert_eq!(0b1000110111, 567);
    assert_eq!(seating("BFFFBBFRRR"), 567);
    assert_eq!(seating("FFFBBBFRRR"), 119);
    assert_eq!(seating("BBFFBBFRLL"), 820);
}

#[test]
fn day5() {
    let contents = fs::read_to_string("day5.txt").expect("Something went wrong reading the file");

    let max_id = contents.split_whitespace().map(seating).max().unwrap();

    println!("max seat: {}", max_id);

    let mut rows = BTreeMap::new();

    contents
        .split_whitespace()
        .map(seating)
        .map(|loc| (loc >> 3, loc & 0x7))
        .for_each(|(row, col)| {
            let val = rows.entry(row).or_insert(0);
            *val |= 1 << col;
        });

    let santas_ticket = "BFBFFFBLRR";

    println!(
        "santas ticket: {tkt} {tkt:010b}",
        tkt = seating(santas_ticket)
    );

    for (key, val) in rows {
        if val == 0b11111111 {
            continue;
        }
        println!("row {:07b}={}:{:08b}", key, val, val);
    }
}
