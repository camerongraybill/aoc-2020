use clap::{Arg, App};
use std::fs;
use std::str::FromStr;
use counter::Counter;


fn main() {
    let args = App::new("Day ten part two of AOC 2020!!")
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

