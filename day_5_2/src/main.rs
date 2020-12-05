use clap::{Arg, App};
use std::str::FromStr;
use std::fs;

struct Seat {
    row: u8,
    column: u8,
}

impl FromStr for Seat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            Seat{
                row: s[0..7]
                    .bytes()
                    .map(|b| b == b'B')
                    .fold(0, |accumulated, b| (accumulated << 1) | u8::from(b)),
                column: s[7..10]
                    .bytes()
                    .map(|b| b == b'R')
                    .fold(0, |accumulated, b| (accumulated << 1) | u8::from(b)),
            }
        )
    }
}

fn seat_id(s: &Seat) -> usize {
    (s.row as usize) * 8 + (s.column) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_1() -> Result<(), String> {
        let seat = "BFFFBBFRRR".parse::<Seat>()?;
        assert_eq!(seat.row, 70);
        assert_eq!(seat.column, 7);
        assert_eq!(seat_id(&seat), 567);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<(), String> {
        let seat = "FFFBBBFRRR".parse::<Seat>()?;
        assert_eq!(seat.row, 14);
        assert_eq!(seat.column, 7);
        assert_eq!(seat_id(&seat), 119);
        Ok(())
    }

    #[test]
    fn test_example_3() -> Result<(), String> {
        let seat = "BBFFBBFRLL".parse::<Seat>()?;
        assert_eq!(seat.row, 102);
        assert_eq!(seat.column, 4);
        assert_eq!(seat_id(&seat), 820);
        Ok(())
    }
}


fn main() {
    let args = App::new("Day five part two of AOC 2020!!")
        .arg(Arg::with_name("input-file").takes_value(true))
        .get_matches();
    let mut all_seat_ids = fs::read_to_string(
        args.value_of("input-file").unwrap()
    )
        .unwrap()
        .lines()
        .map(|raw_seat| raw_seat.parse::<Seat>().unwrap())
        .map(|s| seat_id(&s))
        .collect::<Vec<usize>>();
    all_seat_ids.sort();
    let my_seat = all_seat_ids
        .iter()
        .zip(all_seat_ids[0]..)
        .find(|(a, b)| *a != b)
        .unwrap()
        .1;
    println!("{}", my_seat);
}

