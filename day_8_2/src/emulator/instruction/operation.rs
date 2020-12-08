use std::str::FromStr;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Operation {
    Accumulate,
    Jump,
    NoOperation,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "acc" => Ok(Operation::Accumulate),
            "jmp" => Ok(Operation::Jump),
            "nop" => Ok(Operation::NoOperation),
            _ => Err("Unknown Operation".to_string()),
        }
    }
}
