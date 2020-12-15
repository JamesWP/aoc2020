use std::{collections::HashMap, convert::TryFrom, convert::TryInto, fs};

#[derive(Debug)]
enum Op<'a> {
    SetMask(&'a str),
    SetMem(u64, u64),
}

struct State {
    mem: HashMap<u64, u64>,
    mask: String,
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

fn set_mem(mem: &mut HashMap<u64, u64>, mask: &mut Vec<char>, val: u64, idx: usize) {
    if idx == 36 {
        // done patching
        let s: String = mask.iter().collect();
        let addr = u64::from_str_radix(&s, 2).unwrap();
        //println!("setting: {} to val: {}. mask: {}", addr, val, s);
        mem.insert(addr, val);
        return;
    }

    let ch = mask.iter().cloned().nth(idx).unwrap();

    if ch == 'X' {
        mask[idx] = '0';
        set_mem(mem, mask, val, idx + 1);
        mask[idx] = '1';
        set_mem(mem, mask, val, idx + 1);

        // restore original value
        mask[idx] = 'X';
    } else {
        set_mem(mem, mask, val, idx + 1);
    }
}

fn apply<'a>(mut state: State, op: Op) -> State {
    println!("op: {:?}", op);
    match op {
        Op::SetMask(new_mask) => state.mask = new_mask.to_owned(),
        Op::SetMem(addr, val) => {
            let mut mask_vec: Vec<char> = state.mask.chars().collect();
            let addr = format!("{:036b}", addr);
            for (idx, v) in mask_vec.iter_mut().enumerate() {
                if *v == '0' {
                    // get idx'th bit in addr
                    let addr_char = addr.chars().nth(idx).unwrap();
                    *v = addr_char;
                }
            }
            set_mem(&mut state.mem, &mut mask_vec, val, 0)
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
            mask: "".to_owned(),
        },
        apply,
    );

    let sum: u64 = state.mem.iter().map(|(_k, v)| v).sum();

    println!("sum: {}", sum);
}
