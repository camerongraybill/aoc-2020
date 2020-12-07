use clap::{Arg, App};
use std::str::FromStr;
use std::fs;
use std::collections::{HashSet};


struct DeclarationForm {
    data: HashSet<u8>,
}


struct DeclarationGroup {
    data: Vec<DeclarationForm>,
}

impl FromStr for DeclarationGroup {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DeclarationGroup {
            data: s
                .lines()
                .map(|line| line.parse::<DeclarationForm>().unwrap())
                .collect()
        })
    }
}

impl FromStr for DeclarationForm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DeclarationForm {
            data: s
                .bytes()
                .collect()
        })
    }

}

fn extract_all_items(group: &DeclarationGroup) -> HashSet<u8> {
    group
        .data[1..]
        .iter()
        .fold(
            group.data[0].data.clone(),
            |a, b| a.intersection(&b.data).copied().collect()
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_declaration_group_fromstr_1() -> Result<(), String> {
        let group = "abc".parse::<DeclarationGroup>().unwrap();
        let all_items = extract_all_items(&group);
        assert_eq!(all_items.len(), 3);
        assert!(all_items.contains(&b'a'));
        assert!(all_items.contains(&b'b'));
        assert!(all_items.contains(&b'c'));
        Ok(())
    }

}


fn main() {
    let args = App::new("Day seven part one of AOC 2020!!")
        .arg(Arg::with_name("input-file").takes_value(true))
        .get_matches();
    let total_count: usize = fs::read_to_string(
        args.value_of("input-file").unwrap()
    )
        .unwrap()
        .split("\n\n")
        .map(|group| group.parse::<DeclarationGroup>().unwrap())
        .map(|group| extract_all_items(&group).len())
        .sum();
    println!("{}", total_count);
}

