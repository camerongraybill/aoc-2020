use clap::{Arg, App};
use std::str::FromStr;
use std::num::ParseIntError;
use std::fs;
use std::collections::{HashMap};

struct Passport {
    data: HashMap<String, String>,
}

impl FromStr for Passport {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Passport {
            data: s
                .split_whitespace()
                .map(
                    |pair|pair.split(':')
                        .map(|s|s.to_string())
                )
                .map(|mut pair| (pair.next().unwrap(), pair.next().unwrap()))
                .collect()
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_passport_fromstr() -> Result<(), String> {
        let passport = "a:b c:d\ne:f".parse::<Passport>().unwrap();
        assert_eq!(passport.data.keys().count(), 3);
        assert!(passport.data.contains_key("a"));
        assert!(passport.data.contains_key("c"));
        assert!(passport.data.contains_key("e"));
        Ok(())
    }

}


fn main() {
    let args = App::new("Day six part one of AOC 2020!!")
        .arg(Arg::with_name("input-file").takes_value(true))
        .get_matches();
    let input_data = fs::read_to_string(
        args.value_of("input-file").unwrap()
    )
        .unwrap();
    println!("{}", input_data);
}

