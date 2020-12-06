use clap::{Arg, App};
use std::str::FromStr;
use std::fs;
use std::collections::{HashSet};

struct DeclarationGroup {
    data: HashSet<u8>,
}

impl FromStr for DeclarationGroup {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DeclarationGroup {
            data: s
                .bytes()
                .filter(|b| *b != b'\n')
                .collect()
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_declaration_group_fromstr_1() -> Result<(), String> {
        let passport = "abc".parse::<DeclarationGroup>().unwrap();
        assert_eq!(passport.data.len(), 3);
        assert!(passport.data.contains(&b'a'));
        assert!(passport.data.contains(&b'b'));
        assert!(passport.data.contains(&b'c'));
        Ok(())
    }
    #[test]
    fn test_declaration_group_fromstr_2() -> Result<(), String> {
        let passport = "a\nb\nc".parse::<DeclarationGroup>().unwrap();
        assert_eq!(passport.data.len(), 3);
        assert!(passport.data.contains(&b'a'));
        assert!(passport.data.contains(&b'b'));
        assert!(passport.data.contains(&b'c'));
        Ok(())
    }
    #[test]
    fn test_declaration_group_fromstr_3() -> Result<(), String> {
        let passport = "ab\nac".parse::<DeclarationGroup>().unwrap();
        assert_eq!(passport.data.len(), 3);
        assert!(passport.data.contains(&b'a'));
        assert!(passport.data.contains(&b'b'));
        assert!(passport.data.contains(&b'c'));
        Ok(())
    }
    #[test]
    fn test_declaration_group_fromstr_4() -> Result<(), String> {
        let passport = "a\na\na\na".parse::<DeclarationGroup>().unwrap();
        assert_eq!(passport.data.len(), 1);
        assert!(passport.data.contains(&b'a'));
        Ok(())
    }
    #[test]
    fn test_declaration_group_fromstr_5() -> Result<(), String> {
        let passport = "b".parse::<DeclarationGroup>().unwrap();
        assert_eq!(passport.data.len(), 1);
        assert!(passport.data.contains(&b'b'));
        Ok(())
    }

}


fn main() {
    let args = App::new("Day six part two of AOC 2020!!")
        .arg(Arg::with_name("input-file").takes_value(true))
        .get_matches();
    let total_count: usize = fs::read_to_string(
        args.value_of("input-file").unwrap()
    )
        .unwrap()
        .split("\n\n")
        .map(|group| group.parse::<DeclarationGroup>().unwrap())
        .map(|group| group.data.len())
        .sum();
    println!("{}", total_count);
}

