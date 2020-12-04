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

struct Year {
    data: u32,
}

impl FromStr for Year {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            Err("Failed to parse".to_string())
        } else {
            match s.parse::<u32>() {
                Ok(val) => Ok(Year{data:val}),
                Err(_) => Err("Failed to parse".to_string())
            }
        }
    }
}

fn byr_valid(data: &String) -> bool {
    match data.parse::<Year>() {
        Ok(val) => val.data >= 1920 && val.data <=2002,
        Err(_) => false
    }
}

fn iyr_valid(data: &String) -> bool {
    match data.parse::<Year>() {
        Ok(val) => val.data >= 2010 && val.data <= 2020,
        Err(_) => false
    }
}

fn eyr_valid(data: &String) -> bool {
    match data.parse::<Year>() {
        Ok(val) => val.data >= 2020 && val.data <= 2030,
        Err(_) => false
    }
}

fn hgt_valid(data: &String) -> bool {
    match data.strip_suffix("cm") {
        Some(data) => {
            match data.parse::<u32>() {
                Ok(val) => val >= 150 && val <= 193,
                Err(_) => false,
            }
        }
        None => match data.strip_suffix("in") {
            Some(data) => {
                match data.parse::<u32>() {
                    Ok(val) => val >= 59 && val <= 76,
                    Err(_) => false,
                }
            }
            None => false
        }
    }
}

fn hcl_valid(data: &String) -> bool {
    data.as_bytes()[0] == b'#' && data[1..].bytes().all(|byte| (byte >= b'0' && byte <= b'9') || (byte >= b'a' && byte <= b'f'))
}

const VALID_EYE_COLORS: [&str; 7] = ["abm", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn ecl_valid(data: &String) -> bool {
    VALID_EYE_COLORS.contains(&&**data)
}

fn pid_valid(data: &String) -> bool {
    if data.len() != 9 {
        false
    } else {
        match data.parse::<u32>() {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

fn passport_valid(pass: &Passport) -> bool {
    match pass.data.get("byr") {
        None => return false,
        Some(data) => if !byr_valid(data) { return false },
    };
    match pass.data.get("iyr") {
        None => return false,
        Some(data) => if !iyr_valid(data) { return false },
    };
    match pass.data.get("eyr") {
        None => return false,
        Some(data) => if !eyr_valid(data) { return false },
    };
    match pass.data.get("hgt") {
        None => return false,
        Some(data) => if !hgt_valid(data) { return false },
    };
    match pass.data.get("hcl") {
        None => return false,
        Some(data) => if !hcl_valid(data) { return false },
    };
    match pass.data.get("ecl") {
        None => return false,
        Some(data) => if !ecl_valid(data) { return false },
    };
    match pass.data.get("pid") {
        None => return false,
        Some(data) => if !pid_valid(data) { return false },
    };
    if pass.data.contains_key("cid") {
        pass.data.len() == 8
    } else {
        pass.data.len() == 7
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

    #[test]
    fn test_examples_valid() -> Result<(), String> {
        let data = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719".split("\n\n")
            .map(|raw_passport| raw_passport.parse::<Passport>().unwrap())
            .all(|p| passport_valid(&p));
        assert!(data);
        Ok(())
    }
    #[test]
    fn test_examples_invalid() -> Result<(), String> {
        let data = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007".split("\n\n")
            .map(|raw_passport| raw_passport.parse::<Passport>().unwrap())
            .all(|p| !passport_valid(&p));
        assert!(data);
        Ok(())
    }

    #[test]
    fn test_byr() -> Result<(), String> {
        assert!(byr_valid(&"2002".to_string()));
        assert!(!byr_valid(&"2003".to_string()));
        Ok(())
    }

    #[test]
    fn test_hgt() -> Result<(), String> {
        assert!(hgt_valid(&"60in".to_string()));
        assert!(hgt_valid(&"190cm".to_string()));
        assert!(!hgt_valid(&"190in".to_string()));
        assert!(!hgt_valid(&"190".to_string()));
        Ok(())
    }

    #[test]
    fn test_hcl() -> Result<(), String> {
        assert!(hcl_valid(&"#123abc".to_string()));
        assert!(!hcl_valid(&"#123abz".to_string()));
        assert!(!hcl_valid(&"123abc".to_string()));
        Ok(())
    }

    #[test]
    fn test_ecl() -> Result<(), String> {
        assert!(ecl_valid(&"brn".to_string()));
        assert!(!ecl_valid(&"wat".to_string()));
        Ok(())
    }

    #[test]
    fn test_pid() -> Result<(), String> {
        assert!(pid_valid(&"000000001".to_string()));
        assert!(!pid_valid(&"0123456789".to_string()));
        Ok(())
    }
}


fn main() {
    let args = App::new("Day four part two of AOC 2020!!")
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

