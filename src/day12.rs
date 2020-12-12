use std::{collections::HashMap, convert::TryFrom, convert::TryInto, fs};

type Instruction = (char, i32);

fn parse_instruction(ins: &str) -> Instruction {
    let ch = ins.chars().next().unwrap();
    let number = &ins[1..];
    let number: i32 = number.parse().unwrap();

    (ch, number)
}
fn add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    let (ax, ay) = a;
    let (bx, by) = b;
    (ax + bx, ay + by)
}

fn rot(a:(i32, i32), deg: i32) -> (i32, i32){
    let (ax, ay) = a;
    match deg {
        0 => a,
        90 => (ay, -ax),
        180 => (-ax, -ay),
        270 => (-ay, ax),
        _ => panic!("unknown deg: {:}", deg),
    }
}

fn mul(a:(i32, i32), mul:i32) -> (i32,i32) {
    let (ax, ay) = a;
    (ax*mul, ay*mul)
}

fn update(pos: ((i32, i32), (i32, i32)), ins: Instruction) -> ((i32, i32), (i32, i32)) {
    let (loc, heading) = pos;
    println!("before update: loc: {:?} hdg: {:?}", loc, heading);
    match ins {
        ('N', len) => (loc, add(heading, (0, len))),
        ('S', len) => (loc, add(heading, (0, -len))),
        ('E', len) => (loc, add(heading, (len, 0))),
        ('W', len) => (loc, add(heading, (-len, 0))),

        ('L', deg) => (loc, rot(heading, 360-deg)),
        ('R', deg) => (loc, rot(heading, deg)),

        ('F', num) => (add(loc, mul(heading, num)), heading),

        _ => panic!("unknown ins: {:?}, pos: {:?}", ins, pos),
    }
}

#[test]
fn test() {
    let contents = fs::read_to_string("day12.txt").expect("Something went wrong reading the file");
    let instructions: Vec<Instruction> = contents.split("\n").map(parse_instruction).collect();

    let (pos, heading) = instructions.iter().cloned().fold(((0, 0), (10, 1)), update);
    let (ew, ns)= pos;
    println!("pos: {:?}", (ew, ns));
    println!("manhat: {}", ew.abs() + ns.abs());
}
