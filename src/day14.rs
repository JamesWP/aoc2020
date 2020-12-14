use std::{collections::HashMap, convert::TryFrom, convert::TryInto, fs};

fn calc_masks(m: &str) -> (u64, u64) {
    let mut SB = 0;
    let mut UB = u64::MAX;
    for (idx, b) in m.chars().enumerate() {
        if b == '1' {
            // set this bit
            SB = SB ^ (1 << 35 - idx);
        } else if b == '0' {
            // unset this bit
            UB = UB ^ (1 << 35 - idx);
        }
    }

    (SB, UB)
}

fn apply_mask(v: u64, m: (u64, u64)) -> u64 {
    let (SB, UB) = m;
    (v | SB) & UB
}

#[test]
fn test_masks() {
    let (SB, UB) = calc_masks("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");

    assert_eq!(SB, 0b1000000u64);
    assert_eq!(UB, u64::MAX ^ 0b10u64);

    assert_eq!(
        apply_mask(11, calc_masks("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")),
        73
    );
    assert_eq!(
        apply_mask(101, calc_masks("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")),
        101
    );
    assert_eq!(
        apply_mask(0, calc_masks("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")),
        64
    );
}

#[derive(Debug)]
enum Op<'a> {
    SetMask(&'a str),
    SetMem(u64, u64),
}

struct State {
    mem: HashMap<u64, u64>,
    mask: (u64, u64),
}

fn parse<'a>(input: &'a str) -> Result<Op<'a>, ()> {
    if input.starts_with("mask") {
        Ok(Op::SetMask(input.strip_prefix("mask = ").unwrap()))
    } else if input.starts_with("mem") {
        let addr: u64 = input
            .split("[")
            .skip(1)
            .next()
            .unwrap()
            .split("]")
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let val: u64 = input
            .split("=")
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        Ok(Op::SetMem(addr, val))
    } else {
        Err(())
    }
}

fn apply<'a>(mut state: State, op: Op) -> State {
    println!("op: {:?}", op);
    match op {
        Op::SetMask(new_mask) => state.mask = calc_masks(new_mask),
        Op::SetMem(addr, val) => {
            state.mem.insert(addr, apply_mask(val, state.mask));
        }
    };
    state
}

#[test]
fn dothing() {
    let contents =
        fs::read_to_string("day14.txt").expect("Something went wrong reading the file");

    let state = contents.split("\n").map(parse).map(Result::unwrap).fold(
        State {
            mem: HashMap::new(),
            mask: (0, 0),
        },
        apply,
    );

    let sum: u64 = state.mem.iter().map(|(_k, v)| v).sum();

    println!("sum: {}", sum);
}
