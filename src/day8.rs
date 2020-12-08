use std::fmt;
use std::{collections::HashSet, convert::TryInto, fs};

#[derive(Clone, Debug)]
enum Instruction {
    Jmp(i32),
    Acc(i32),
    Nop(i32),
}

fn instruction_parse(input: &str) -> Option<Instruction> {
    let mut iter = input.split_whitespace();

    let op_str = iter.next()?;

    let val: i32 = iter.next()?.parse().ok()?;

    match op_str {
        "jmp" => Some(Instruction::Jmp(val)),
        "acc" => Some(Instruction::Acc(val)),
        "nop" => Some(Instruction::Nop(val)),
        _ => None,
    }
}

struct HGC {
    pc: usize,
    acc: i32,
    memory: Vec<Instruction>,
}

impl HGC {
    fn step(&mut self) -> Option<()> {
        let instruction = self.memory.get(self.pc)?;

        self.pc += 1;

        match instruction {
            Instruction::Acc(val) => {
                self.acc += val;
                Some(())
            }
            Instruction::Jmp(val) => {
                let mut new_pc: i32 = self.pc.try_into().ok()?;
                new_pc += val - 1;
                self.pc = new_pc.try_into().ok()?;
                Some(())
            }
            Instruction::Nop(_) => Some(()),
        }
    }
}

impl fmt::Debug for HGC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HGC")
            .field("pc", &self.pc)
            .field("acc", &self.acc)
            .field("isn", &self.memory.get(self.pc))
            .finish()
    }
}

#[test]
fn first_repeating() {
    let contents = fs::read_to_string("day8.txt").expect("Something went wrong reading the file");

    let instructions: Vec<Instruction> = contents
        .split("\n")
        .map(instruction_parse)
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect();

    let candidates: Vec<usize> = instructions
        .iter()
        .enumerate()
        .filter(|(_idx, op)| match op {
            Instruction::Jmp(_) => true,
            Instruction::Acc(_) => true,
            Instruction::Nop(_) => false,
        })
        .map(|(idx, _op)| idx)
        .collect();

    for candidate in candidates {
        let mut modified_instructions = instructions.clone();

        let bad_isn = modified_instructions.get(candidate).unwrap();

        modified_instructions[candidate] = match bad_isn.clone() {
            Instruction::Jmp(x) => Instruction::Nop(x),
            Instruction::Acc(x) => Instruction::Acc(x),
            Instruction::Nop(x) => Instruction::Jmp(x),
        };

        let mut console = HGC {
            memory: modified_instructions,
            pc: 0,
            acc: 0,
        };

        let mut executed_instructions = HashSet::new();

        loop {
            if executed_instructions.contains(&console.pc) {
                break;
            }

            if console.pc == console.memory.len() {
                println!("Step was last; terminating");
                break;
            }

            executed_instructions.insert(console.pc);

            // println!("console: {:?}", console);

            if console.step().is_none() {
                break;
            }
        }

        println!("candidate: {} console: {:?}", candidate, console);
    }
}
