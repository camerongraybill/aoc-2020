use clap::{Arg, App};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn read_file(filename: impl AsRef<Path>) -> Vec<i32> {
    BufReader::new(
        File::open(
            filename
        ).expect("File does not exist")
    )
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|l| l.parse::<i32>().unwrap())
        .collect()
}

fn main() {
    let args = App::new("Day one part two of AOC 2020!!")
        .arg(Arg::with_name("input-file").takes_value(true))
        .get_matches();
    let input_ints = read_file(args.value_of("input-file").unwrap());
    for first_int in input_ints.iter() {
        for second_int in input_ints.iter() {
            for third_int in input_ints.iter() {
                if first_int + second_int + third_int == 2020 {
                    println!("{}", first_int * second_int * third_int);
                    return;
                }
            }
        }
    }
}
