use clap::{Arg, App};
use std::fs;
use std::str::FromStr;

enum Bus {
    Unscheduled,
    Scheduled(usize),
}

struct InputData {
    busses: Vec<Bus>,
}

impl FromStr for Bus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Bus::Unscheduled),
            other => Ok(Bus::Scheduled(other.parse().unwrap()))
        }
    }
}

impl FromStr for InputData {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines_iter = s.lines();
        lines_iter.next();
        Ok(
            InputData {
                busses: lines_iter.next()
                    .unwrap()
                    .split(',')
                    .map(|num| num.parse().unwrap()).collect()
            }
        )
    }
}

impl InputData {
    fn find_consecutive_timestamp(self) -> usize {
        0
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<(), String> {
        let example_data: InputData = "
7,13,x,x,59,x,31,19
".parse().unwrap();
        assert_eq!(example_data.find_consecutive_timestamp(), 1068781);

        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<(), String> {
        let example_data: InputData = "
17,x,13,19
".parse().unwrap();
        assert_eq!(example_data.find_consecutive_timestamp(), 3417);

        Ok(())
    }

    #[test]
    fn test_example_3() -> Result<(), String> {
        let example_data: InputData = "
67,7,59,61
".parse().unwrap();
        assert_eq!(example_data.find_consecutive_timestamp(), 754018);

        Ok(())
    }

    #[test]
    fn test_example_4() -> Result<(), String> {
        let example_data: InputData = "
67,x,7,59,61
".parse().unwrap();
        assert_eq!(example_data.find_consecutive_timestamp(), 779210);

        Ok(())
    }

    #[test]
    fn test_example_5() -> Result<(), String> {
        let example_data: InputData = "
67,7,x,59,61
".parse().unwrap();
        assert_eq!(example_data.find_consecutive_timestamp(), 1261476);

        Ok(())
    }

    #[test]
    fn test_example_6() -> Result<(), String> {
        let example_data: InputData = "
1789,37,47,1889
".parse().unwrap();
        assert_eq!(example_data.find_consecutive_timestamp(), 1202161486);

        Ok(())
    }

}

fn main() {
    let args = App::new("Day 13 part 2 of AOC 2020!!")
        .arg(Arg::with_name("input-file").takes_value(true))
        .get_matches();
    let input_data: InputData = fs::read_to_string(
        args.value_of("input-file").unwrap()
    )
        .unwrap()
        .parse()
        .unwrap();
    println!("{}", input_data.find_consecutive_timestamp())
}

