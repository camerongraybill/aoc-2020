mod emulator;

use clap::{Arg, App};
use std::fs;
use emulator::instruction::Instruction;
use crate::emulator::run_until_repeat;


fn main() {
    let args = App::new("Day eight part one of AOC 2020!!")
        .arg(Arg::with_name("input-file").takes_value(true))
        .get_matches();
    let all_instructions: Vec<Instruction> = fs::read_to_string(
        args.value_of("input-file").unwrap()
    )
        .unwrap()
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect();
    println!("{}", run_until_repeat(&all_instructions));
}

