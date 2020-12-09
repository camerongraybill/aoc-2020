use clap::{Arg, App};
use std::fs;

fn chunk_is_satisfied(chunk: &[usize], target: usize) -> bool {
    for idx_a in 0..chunk.len() {
        for idx_b in 0..chunk.len() {
            if chunk[idx_a] + chunk[idx_b] == target {
                return true;
            }
        }
    }
    false
}


fn find_first_broken_idx(data: &Vec<usize>, preamble_size: usize) -> usize {
    (preamble_size..data.len())
        .map(|idx_in_data| (&data[idx_in_data - preamble_size..idx_in_data], idx_in_data))
        .find(|(chunk, idx_in_data)| !chunk_is_satisfied(chunk, data[*idx_in_data]))
        .unwrap()
        .1
}

fn main() {
    let args = App::new("Day nine part one of AOC 2020!!")
        .arg(Arg::with_name("input-file").takes_value(true))
        .arg(Arg::with_name("preamble-size").takes_value(true))
        .get_matches();
    let input_data: Vec<usize> = fs::read_to_string(
        args.value_of("input-file").unwrap()
    )
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let preamble_size: usize = args.value_of("preamble-size").unwrap().parse().unwrap();
    println!("{}", input_data[find_first_broken_idx(&input_data, preamble_size)]);
}

