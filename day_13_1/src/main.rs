use clap::{Arg, App};
use std::fs;
use std::str::FromStr;

struct InputData {
    arrive_at: usize,
    busses: Vec<usize>,
}

impl FromStr for InputData {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines_iter = s.lines();
        Ok(
            InputData {
                arrive_at: lines_iter.next().unwrap().parse().unwrap(),
                busses: lines_iter.next()
                    .unwrap()
                    .split(',')
                    .filter(|split| *split != "x")
                    .map(|num| num.parse().unwrap()).collect()
            }
        )
    }
}

impl InputData {
    fn first_departure(self) -> usize {
        for num in self.arrive_at.. {
            for bus in &self.busses {
                if num % bus == 0 {
                    return bus * (num - self.arrive_at);
                }
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<(), String> {
        let example_data: InputData = "939
7,13,x,x,59,x,31,19
".parse().unwrap();
        assert_eq!(example_data.first_departure(), 295);

        Ok(())
    }

}

fn main() {
    let args = App::new("Day 13 part 1 of AOC 2020!!")
        .arg(Arg::with_name("input-file").takes_value(true))
        .get_matches();
    let input_data: InputData = fs::read_to_string(
        args.value_of("input-file").unwrap()
    )
        .unwrap()
        .parse()
        .unwrap();
    println!("{}", input_data.first_departure())
}

