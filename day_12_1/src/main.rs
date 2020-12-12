use clap::{Arg, App};
use std::fs;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() -> Result<(), String> {
        Ok(())
    }

}

fn main() {
    let args = App::new("Day 12 part 1 of AOC 2020!!")
        .arg(Arg::with_name("input-file").takes_value(true))
        .get_matches();
    let input_data = fs::read_to_string(
        args.value_of("input-file").unwrap()
    )
        .unwrap();
    println!("{}", input_data)
}

