use clap::{Arg, App};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::num::ParseIntError;

struct Line {
    min_ct: usize,
    max_ct: usize,
    target: char,
    str: String,
}

fn line_valid(l: &Line) -> bool {
    let ct = l.str.matches(l.target).count();
    ct <= l.max_ct && ct >= l.min_ct
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_by_space: Vec<&str> = s.split(' ').collect();
        let min_max_split: Vec<&str> = split_by_space[0].split('-').collect();
        Ok(Line {
            min_ct: min_max_split[0].parse::<usize>().unwrap(),
            max_ct: min_max_split[1].parse::<usize>().unwrap(),
            target: split_by_space[1].chars().next().unwrap(),
            str: split_by_space[2].parse().unwrap(),
        })
    }
}

fn main() {
    let args = App::new("Day two part one of AOC 2020!!")
        .arg(Arg::with_name("input-file").takes_value(true))
        .get_matches();
    let ct = BufReader::new(
        File::open(
            args.value_of("input-file").unwrap()
        ).expect("File does not exist")
    )
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|l| l.parse::<Line>().unwrap())
        .filter(line_valid)
        .count();
    println!("{}", ct);
}
