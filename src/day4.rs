use std::{collections::HashMap, fs};

fn has_required_fields(a: &HashMap<&str, &str>) -> bool {
    let missing = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .cloned()
        .filter(|field| !a.contains_key(field));

    missing.count() == 0
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
fn has_valid_byr(a: &HashMap<&str, &str>) -> bool {
    let field = a.get("byr").unwrap();
    let number = field.parse::<i32>();

    if let Ok(number) = number {
        field.len() == 4 && number >= 1920 && number <= 2002
    } else {
        false
    }
}

// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
fn has_valid_iyr(a: &HashMap<&str, &str>) -> bool {
    let field = a.get("iyr").unwrap();
    let number: i32 = field.parse().unwrap();

    field.len() == 4 && number >= 2010 && number <= 2020
}

// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
fn has_valid_eyr(a: &HashMap<&str, &str>) -> bool {
    let field = a.get("eyr").unwrap();
    let number: i32 = field.parse().unwrap();

    field.len() == 4 && number >= 2020 && number <= 2030
}

// hgt (Height) - a number followed by either cm or in:
// If cm, the number must be at least 150 and at most 193.
// If in, the number must be at least 59 and at most 76.
fn has_valid_hgt(a: &HashMap<&str, &str>) -> bool {
    let field = a.get("hgt").unwrap();

    if field.len() <= 2 {
        return false;
    }

    let bounds = if field.ends_with("in") {
        59..=76
    } else if field.ends_with("cm"){
        150..=193
    } else {
        0..=0
    };

    let num = field[0..field.len() - 2].parse::<i32>();
    num.is_ok() && bounds.contains(&(num.unwrap()))
}

fn is_hex(c: char) -> bool {
    match &c {
        '0'..='9' => true,
        'a'..='f' => true,
        _ => false,
    }
}

// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
fn has_valid_hcl(a: &HashMap<&str, &str>) -> bool {
    let field = a.get("hcl").unwrap();

    if field.len() != 7 {
        return false;
    }

    if &field[0..1] != "#" {
        return false;
    }

    // we shouldnt find any invalid chars
    field[1..].find(|c| !is_hex(c)).is_none()
}

// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
fn has_valid_ecl(a: &HashMap<&str, &str>) -> bool {
    let field = a.get("ecl").unwrap();

    match field.clone() {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false,
    }
}

// pid (Passport ID) - a nine-digit number, including leading zeroes.
fn has_valid_pid(a: &HashMap<&str, &str>) -> bool {
    let field = a.get("pid").unwrap();

    if field.len() != 9 {
        return false;
    }

    field.parse::<i32>().is_ok()
}

#[test]
fn test() {
    let contents = fs::read_to_string("day4.txt").expect("Something went wrong reading the file");

    let passports = contents
        .split("\n\n")
        .map(|pprt| {
            pprt.split_whitespace()
                .map(|record| {
                    let pos = record.find(":").unwrap();
                    let (key, val) = record.split_at(pos);
                    (key, &val[1..])
                })
                .collect::<HashMap<&str, &str>>()
        })
        .collect::<Vec<HashMap<&str, &str>>>();

    let count = passports
        .iter()
        .cloned()
        .filter(has_required_fields)
        .filter(has_valid_byr)
        .filter(has_valid_iyr)
        .filter(has_valid_eyr)
        .filter(has_valid_hgt)
        .filter(has_valid_hcl)
        .filter(has_valid_ecl)
        .filter(has_valid_pid)
        .count();

    println!("Valid passports {}", count);
}
