use clap::{Arg, App};
use std::str::FromStr;
use std::fs;
use std::collections::{HashMap, HashSet};

type Color = String;

struct ColorRule {
    bag_color: Color,
    contents: BagContents,
}

struct BagContents {
    data: Vec<Color>
}

struct AllRules {
    data: HashMap<Color, BagContents>
}

fn bag_contains(rules: &AllRules, bag: &Color, contains: &Color) -> bool {
    let contained_colors: HashSet<&String> = rules.data[bag].data.iter().collect();
    contained_colors.contains(contains) || contained_colors.iter().any(|contained_color| bag_contains(rules, contained_color, contains))
}

fn bag_full_depth(rules: &AllRules, bag: &Color) -> usize {
    rules.data[bag].data.iter()
        .map(|bag| 1 + bag_full_depth(&rules, &bag))
        .sum()
}

impl FromStr for ColorRule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let space_pos: Vec<usize> = s
            .match_indices(' ')
            .map(|(idx, _)| idx)
            .collect();

        Ok(ColorRule {
            bag_color: s[0..space_pos[1]].to_string(),
            contents: BagContents {
                data: space_pos[3..]
                    .iter()
                    .zip(1..)
                    .map(|(_, idx)| idx)
                    .filter(|idx| idx % 4 == 0 )
                    // idx is now the index of the space before the number
                    .map(|idx| (s[(space_pos[idx - 1] + 1)..space_pos[idx]].parse::<usize>().unwrap(), &s[(space_pos[idx] + 1)..space_pos[idx + 2]]) )
                    .flat_map(|(count, color)| (0..count).map(move |_| color.to_string()))
                    .collect()
            }

        })
    }
}

impl FromStr for AllRules {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            AllRules {
                data: s
                    .lines()
                    .map(|line| line.parse::<ColorRule>().unwrap())
                    .map(|rule| (rule.bag_color, rule.contents))
                    .collect()
            }
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_1() -> Result<(), String> {
        let rule = "light red bags contain 1 bright white bag, 2 muted yellow bags.".parse::<ColorRule>()?;
        assert_eq!(rule.bag_color, "light red");
        assert_eq!(rule.contents.data.len(), 3);
        Ok(())
    }
    #[test]
    fn test_example_2() -> Result<(), String> {
        let rule = "faded blue bags contain no other bags.".parse::<ColorRule>()?;
        assert_eq!(rule.bag_color, "faded blue");
        assert_eq!(rule.contents.data.len(), 0);
        Ok(())
    }
}


fn main() {
    let args = App::new("Day eight part one of AOC 2020!!")
        .arg(Arg::with_name("input-file").takes_value(true))
        .get_matches();
    let all_rules = fs::read_to_string(
        args.value_of("input-file").unwrap()
    )
        .unwrap()
        .parse::<AllRules>().unwrap();
    let target_color = "shiny gold".to_string();
    println!("{}", bag_full_depth(&all_rules, &target_color))
}

