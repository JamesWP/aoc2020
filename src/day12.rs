
use std::{collections::HashMap, convert::TryFrom, convert::TryInto, fs};

type Instruction = (char, i32);

fn parse_instruction(ins: &str) -> Instruction {
    let ch = ins.chars().next().unwrap();
    let number = &ins[1..];
    let number:i32 = number.parse().unwrap();

    (ch, number)
}

fn update(pos:(i32,i32, i32), ins: Instruction) -> (i32, i32, i32) {
    let (ew,ns,heading) = pos;
    let heading = (heading+360) % 360;
    match (ins, heading) {
        (('N', len), heading) => (ew, ns + len, heading),
        (('S', len), heading) => (ew, ns - len, heading),
        (('E', len), heading) => (ew+len, ns, heading),
        (('W', len), heading) => (ew-len, ns, heading),
        (('L', len), heading) => (ew, ns, heading-len),
        (('R', len), heading) => (ew, ns, heading+len),
        (('F', len), 0) => update(pos, ('N', len)),
        (('F', len), 90) => update(pos, ('E', len)),
        (('F', len), 180) => update(pos, ('S', len)),
        (('F', len), 270) => update(pos, ('W', len)),
        _=> panic!("unknown ins: {:?}, heading: {}", ins, heading)
    }
}

#[test]
fn test() {
    let contents = fs::read_to_string("day12.txt").expect("Something went wrong reading the file");
    let instructions: Vec<Instruction> = contents.split("\n").map(parse_instruction).collect();

    let (ew, ns, _heading) = instructions.iter().cloned().fold((0,0, 90), update);

    println!("pos: {:?}", (ew, ns));
    println!("manhat: {}", ew.abs() + ns.abs());
}