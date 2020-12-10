use clap::{Arg, App};
use std::fs;
use std::cmp::min;
use std::collections::HashMap;


fn count_charger_chains_cached(data: &[usize], cache_table: &mut HashMap<usize, usize>) -> usize {
    let last_placed_charger = &data[0];

    if cache_table.contains_key(last_placed_charger) {
        cache_table[last_placed_charger]
    } else {
        let val =
            if data.len() == 1 {
                1
            } else {
                (1..min(4, data.len()))
                    .map(
                        |idx| if data[idx] <= last_placed_charger + 3 { count_charger_chains_cached(&data[idx..data.len()], cache_table) } else { 0 }
                    ).sum()
            };
        cache_table.insert(*last_placed_charger, val);
        val
    }
}

fn count_charger_chains(data: &[usize], cache: bool) -> usize {
    if cache {
        let mut cache_map: HashMap<usize, usize> = HashMap::new();
        count_charger_chains_cached(&data, &mut cache_map)
    } else {
        count_charger_chains_impl(&data)
    }
}


fn count_charger_chains_impl(data: &[usize]) -> usize {
    let last_placed_charger = &data[0];

    if data.len() == 1 {
        1
    } else {
        (1..min(4, data.len()))
            .map(
                |idx| if data[idx] <= last_placed_charger + 3 { count_charger_chains_impl(&data[idx..data.len()]) } else { 0 }
            ).sum()
    }
}

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

    println!("{}", count_charger_chains(&input_data, true));
    println!("{}", count_charger_chains(&input_data, false));
}

