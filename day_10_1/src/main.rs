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

fn find_contiguous_addition(data: &Vec<usize>, target: usize) -> &[usize] {
    for starting_idx in 0..data.len() {
        for end_idx in starting_idx..data.len() {
            let range = &data[starting_idx..=end_idx];
            let sum: usize = range.iter().sum();
            if sum > target {
                break;
            }
            if sum == target {
                return range;
            }
        }
    }
    unreachable!()
}

fn main() {
    let args = App::new("Day ten part one of AOC 2020!!")
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
    let part_one_answer = input_data[find_first_broken_idx(&input_data, preamble_size)];
    let part_two_range = find_contiguous_addition(&input_data, part_one_answer);


    println!("{}", part_two_range.iter().min().unwrap() + part_two_range.iter().max().unwrap());
}

