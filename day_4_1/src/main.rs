#[macro_use]
extern crate lazy_static;


use clap::{Arg, App};
use std::str::FromStr;
use std::num::ParseIntError;
use std::fs;
use std::collections::{HashMap, HashSet};

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
lazy_static! {

    static ref REQUIRED_KEYS: HashSet<String> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .map(<&str>::to_string)
        .collect();

    static ref ALLOWED_OTHER_KEYS: HashSet<String> = ["cid"]
        .iter()
        .map(<&str>::to_string)
        .collect();

    static ref ALL_ALLOWED_KEYS: HashSet<String> = REQUIRED_KEYS
        .union(&ALLOWED_OTHER_KEYS)
        .cloned()
        .collect();
}

fn passport_valid(pass: &Passport) -> bool {
    let pass_keys_set: HashSet<String> = pass.data.keys().cloned().collect();
    let extra_keys = pass_keys_set.difference(&ALL_ALLOWED_KEYS);
    let missing_keys = REQUIRED_KEYS.difference(&pass_keys_set);
    missing_keys.count() == 0 && extra_keys.count() == 0
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
    let args = App::new("Day four part one of AOC 2020!!")
        .arg(Arg::with_name("input-file").takes_value(true))
        .get_matches();
    let valid_passport_count = fs::read_to_string(
        args.value_of("input-file").unwrap()
    )
        .unwrap()
        .split("\n\n")
        .map(|raw_passport| raw_passport.parse::<Passport>().unwrap())
        .filter(passport_valid)
        .count();
    println!("{}", valid_passport_count);
}

