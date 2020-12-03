use std::{ops::Range, convert::TryInto};
use std::str::FromStr;
use std::{collections::HashSet, fs};
use itertools::Itertools;
use scan_fmt::scan_fmt;

#[derive(Debug, PartialEq)]
struct Require {
    occur: Range<usize>,
    char: char,
}

impl FromStr for Require {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let (l, u, c) = scan_fmt!(s, "{d}-{d} {[a-z]}", usize, usize, char).unwrap();

        Ok(Require {
            occur: l..u+1,
            char: c
        })
    }
}

impl Require {
    fn matches_pt1(&self, p: &str) -> bool {
        let count = p.chars().filter(|c| *c == self.char).count();
        self.occur.contains(&count)
    }

    fn matches_pt2(&self, p: &str) -> bool {
        let (a, b) = (p.chars().nth(self.occur.start-1), p.chars().nth(self.occur.end-2));
        if let (Some(a), Some(b)) = (a,b) {
            (a == self.char && b != self.char) || (b == self.char && a != self.char)
        } else  {
            false
        }
    }
}

#[test]
fn test_parse() {
    let r = Require::from_str("1-4 a").unwrap();

    assert_eq!(r, Require{occur: 1..5, char: 'a'});

    assert!(r.matches_pt1("a"));
    assert!(r.matches_pt1("aaaa"));
    assert!(r.matches_pt1("aabaa"));
    assert!(!r.matches_pt1("aaaaa"));
    assert!(!r.matches_pt1("b"));
}

#[test]
fn test_parse2() {
    assert!(Require::from_str("1-3 a").unwrap().matches_pt2("abcde"));
    assert!(!Require::from_str("1-3 b").unwrap().matches_pt2("cdefg"));
    assert!(!Require::from_str("2-9 c").unwrap().matches_pt2("ccccccccc"));
}

#[test]
fn test() {
    let contents = fs::read_to_string("day2.txt")
        .expect("Something went wrong reading the file");

    let passwords: Vec<(Require, String)> = contents
        .split("\n")
        .map(|line| {
            let ps: [&str; 2] = line.split(":").collect_vec().try_into().expect("two parts");
            (Require::from_str(ps[0]).unwrap(),ps[1].trim().to_owned())
        })
        .collect();

    let matches = passwords.iter().filter(|(r,p)| r.matches_pt1(p)).count();

    println!("num matches pt1: {}", matches);


    let matches = passwords.iter().filter(|(r,p)| r.matches_pt2(p)).count();

    println!("num matches pt2: {}", matches);
}