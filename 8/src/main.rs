use std::io::{BufReader, BufRead};
use std::{io, fmt};
use std::fs::File;
use std::collections::{HashMap, HashSet};

#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fmt::{Debug, Formatter};

lazy_static! {
    static ref RE_INSTRUCTION: Regex = Regex::new(r"(\w+)\s(-?\+?\d+)").unwrap();
}

#[derive(Debug, Copy, Clone)]
enum OpCode {
    NOP(i32),
    ACC(i32),
    JMP(i32),
}

#[derive(Copy, Clone)]
struct Instruction {
    command: OpCode,
}

impl Instruction {
    fn new(line: &str) -> Instruction {
        let cap = RE_INSTRUCTION.captures(line).unwrap();
        let instruction = cap.get(1).unwrap().as_str();
        let argument: i32 = cap.get(2).unwrap().as_str().parse().unwrap();

        let opcode = match instruction {
            "nop" => { OpCode::NOP(argument) }
            "acc" => { OpCode::ACC(argument) }
            "jmp" => { OpCode::JMP(argument) }
            _ => { panic!("Unknown OpCode"); }
        };

        Instruction {
            command: opcode,
        }
    }
}

impl From<OpCode> for Instruction {
    fn from(item: OpCode) -> Self {
        Instruction {
            command: item
        }
    }
}

struct VirtualMachineTerminated;

impl fmt::Display for VirtualMachineTerminated {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Virtual Machine terminated")
    }
}

impl fmt::Debug for VirtualMachineTerminated {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Virtual Machine terminated! {{ file: {}, line: {} }}", file!(), line!())
    }
}

struct VirtualMachine {
    eip: i32,
    accumulator: i32,
    instructions: HashMap<i32, Instruction>,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{Command:{:?}}}", self.command)
    }
}

impl VirtualMachine {
    fn new(instruction_data: Vec<String>) -> VirtualMachine {
        let instructions: HashMap<i32, Instruction> = instruction_data.into_iter().enumerate()
            .map(|(idx, value)| (idx as i32, Instruction::new(&value)))
            .collect();
        VirtualMachine {
            eip: 0,
            accumulator: 0,
            instructions,
        }
    }

    fn step(&mut self) -> Result<(), VirtualMachineTerminated> {
        match self.instructions.get(&self.eip).ok_or(VirtualMachineTerminated)?.command {
            OpCode::NOP(argument) => { self.instruction_nop(argument) }
            OpCode::ACC(argument) => { self.instruction_acc(argument) }
            OpCode::JMP(argument) => { self.instruction_jmp(argument) }
        };

        Ok(())
    }

    fn overwrite_instruction(&mut self, idx: i32, opcode: OpCode) {
        (*self.instructions.get_mut(&idx).unwrap()).command = opcode;
    }

    fn inc_eip(&mut self) {
        self.eip += 1;
    }

    fn instruction_nop(&mut self, argument: i32) {
        self.inc_eip();
    }

    fn instruction_acc(&mut self, argument: i32) {
        self.accumulator += argument;
        self.inc_eip();
    }

    fn instruction_jmp(&mut self, argument: i32) {
        self.eip += argument;
    }
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().filter_map(io::Result::ok).collect();

    let mut vm = VirtualMachine::new(input.clone());

    let instructions: Vec<Instruction> = input.iter().map(|x| Instruction::new(x)).collect();

    // let mut instruction_tracker: HashMap<i32, bool> = HashMap::new();
    //
    // // Part 1
    // loop {
    //     let eip = vm.eip;
    //     if instruction_tracker.contains_key(&eip) {
    //         println!("Part 1: {}", vm.accumulator);
    //         break;
    //     }
    //     instruction_tracker.insert(eip, true);
    //
    //     vm.step();
    // }

    // Part 2

    // Find fix

    let mut come_from = HashMap::<i32, Vec<i32>>::new();
    for (idx, instruction) in instructions.iter().enumerate() {
        let jumps_to = match instruction.command {
            OpCode::NOP(arg) => { idx as i32 + 1 }
            OpCode::ACC(arg) => { idx as i32 + 1 }
            OpCode::JMP(arg) => { idx as i32 + arg }
        };
        come_from.entry(jumps_to as i32).or_default().push(idx as i32);
    }


    let mut stack = vec![instructions.len() as i32];
    let mut leads_to_end = HashSet::new();
    while let Some(idx) = stack.pop() {
        if !leads_to_end.contains(&idx) {
            leads_to_end.insert(idx);
            if let Some(froms) = come_from.get(&idx) {
                for &from in froms {
                    stack.push(from);
                }
            }
        }
    }

    let mut patched = false;
    loop {
        if !patched && vm.eip < instructions.len() as i32 {
            let current_instruction = vm.instructions.get(&vm.eip).unwrap();
            let uncorrupt = match current_instruction.command {
                OpCode::NOP(arg) => { OpCode::JMP(arg) }
                OpCode::ACC(arg) => { current_instruction.command }
                OpCode::JMP(arg) => { OpCode::NOP(arg) }
            };
            let jumps_to = match uncorrupt {
                OpCode::NOP(arg) => { vm.eip as i32 + 1 }
                OpCode::ACC(arg) => { vm.eip as i32 + 1 }
                OpCode::JMP(arg) => { vm.eip as i32 + arg }
            };

            if leads_to_end.contains(&jumps_to) {
                vm.overwrite_instruction(vm.eip, uncorrupt);
                patched = true;
            }
        }


        match vm.step() {
            Ok(_) => {}
            Err(_) => {
                println!("Part 2: {}", vm.accumulator);
                break;
            }
        }
    };

    Ok(())
}
