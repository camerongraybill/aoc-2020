mod emulator;

use clap::{Arg, App};
use std::fs;
use emulator::instruction::Instruction;
use crate::emulator::{run_until_repeat_or_halt};
use crate::emulator::instruction::operation::Operation;

fn swap_op(s: &Instruction) -> Instruction {
    Instruction {
        op: match s.op {
            Operation::Accumulate => Operation::Accumulate,
            Operation::Jump => Operation::NoOperation,
            Operation::NoOperation => Operation::Jump,
        },
        arg: s.arg
    }
}

fn find_correct_swap(mut all_instructions: Vec<Instruction>) -> i32 {
    for index in 0..all_instructions.len() {
        if all_instructions[index].op == Operation::Accumulate {
            continue;
        }
        all_instructions[index] = swap_op(&all_instructions[index]);
        let state = run_until_repeat_or_halt(&all_instructions);
        if state.halted {
            return state.value;
        }
        all_instructions[index] = swap_op(&all_instructions[index]);
    }
    return 0;
}

fn main() {
    let args = App::new("Day eight part two of AOC 2020!!")
        .arg(Arg::with_name("input-file").takes_value(true))
        .get_matches();
    let all_instructions: Vec<Instruction> = fs::read_to_string(
        args.value_of("input-file").unwrap()
    )
        .unwrap()
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect();
    println!("{}", find_correct_swap(all_instructions));
}

