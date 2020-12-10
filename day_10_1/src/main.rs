use clap::{Arg, App};
use std::fs;
use std::str::FromStr;
use counter::Counter;


type Joltage = u32;

struct Adapter {
    output_joltage: Joltage,
}

impl Adapter {
    fn can_pull_from(&self, source_joltage: Joltage) -> bool {
        if self.output_joltage == 0 {
            source_joltage == 0
        } else if self.output_joltage < 3 {
            source_joltage <= (self.output_joltage - 1)
        } else {
            source_joltage >= (self.output_joltage - 3) && source_joltage <= (self.output_joltage - 1)
        }
    }
}

impl FromStr for Adapter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            Adapter {
                output_joltage: s.parse().unwrap()
            }
        )
    }
}

fn main() {
    let args = App::new("Day ten part one of AOC 2020!!")
        .arg(Arg::with_name("input-file").takes_value(true))
        .get_matches();
    let mut input_data: Vec<usize> = fs::read_to_string(
        args.value_of("input-file").unwrap()
    )
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    input_data.sort();
    input_data.insert(0, 0);
    input_data.push(input_data.last().unwrap() + 3);
    let counted: Counter<_> = input_data[0..input_data.len() - 1]
        .iter()
        .zip(input_data[1..input_data.len()].iter())
        .map(|(lower_index, higher_index)| higher_index - lower_index)
        .collect();



    println!("{}", counted[&1] * counted[&3]);
}

