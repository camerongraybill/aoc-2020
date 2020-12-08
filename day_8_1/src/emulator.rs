use crate::emulator::instruction::Instruction;
use crate::emulator::instruction::operation::Operation;
use std::collections::HashSet;

pub mod instruction;

struct State {
    value: i32,
    instruction_pointer: u32,
}

impl State {
    fn new() -> State {
        State {
            value: 0,
            instruction_pointer: 0,
        }
    }
}

fn run_instruction(
    s: State,
    memory: &Vec<Instruction>,
) -> State {
    let next_ins = &memory[s.instruction_pointer as usize];
    match next_ins.op {
        Operation::Accumulate => State {
            value: s.value + next_ins.arg,
            instruction_pointer: s.instruction_pointer + 1,
        },
        Operation::Jump => State {
            value: s.value,
            instruction_pointer: s.instruction_pointer.wrapping_add(next_ins.arg as u32),
        },
        Operation::NoOperation => State {
            value: s.value,
            instruction_pointer: s.instruction_pointer + 1,
        },
    }
}

pub fn run_until_repeat(
    memory: &Vec<Instruction>,
) -> i32 {
    let mut instructions_run: HashSet<u32> = HashSet::new();
    let mut state = State::new();
    loop {
        if instructions_run.contains(&state.instruction_pointer) {
            return state.value;
        }
        instructions_run.insert(state.instruction_pointer);
        state = run_instruction(state, &memory);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<(), String> {
        let instructions: Vec<Instruction> = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6".lines().map(|line| line.parse().unwrap()).collect();
        assert_eq!(run_until_repeat(&instructions), 5);
        Ok(())
    }
}