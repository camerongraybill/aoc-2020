pub mod operation;

use std::str::FromStr;
use crate::emulator::instruction::operation::Operation;


pub struct Instruction {
    pub op: Operation,
    pub arg: i32,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(' ').collect();
        Ok(
            Instruction {
                op: split[0].parse()?,
                arg: split[1].parse().unwrap(),
            }
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<(), String> {
        let instruction = "nop +0".parse::<Instruction>()?;
        assert_eq!(instruction.arg, 0);
        assert_eq!(instruction.op, Operation::NoOperation);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<(), String> {
        let instruction = "acc +1".parse::<Instruction>()?;
        assert_eq!(instruction.arg, 1);
        assert_eq!(instruction.op, Operation::Accumulate);
        Ok(())
    }

    #[test]
    fn test_example_3() -> Result<(), String> {
        let instruction = "jmp +4".parse::<Instruction>()?;
        assert_eq!(instruction.arg, 4);
        assert_eq!(instruction.op, Operation::Jump);
        Ok(())
    }

    #[test]
    fn test_example_4() -> Result<(), String> {
        let instruction = "jmp -4".parse::<Instruction>()?;
        assert_eq!(instruction.arg, -4);
        assert_eq!(instruction.op, Operation::Jump);
        Ok(())
    }
}