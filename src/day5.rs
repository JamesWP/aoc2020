use std::{fs};

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
}
