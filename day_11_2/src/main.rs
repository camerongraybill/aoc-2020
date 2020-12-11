use clap::{Arg, App};
use std::fs;
use std::str::FromStr;
use std::ops::Index;
use itertools::Itertools;

#[derive(PartialEq)]
#[derive(Debug)]
struct FloorMap {
    data: Vec<Position>,
    width: usize,
    height: usize,
}

impl Clone for FloorMap {
    fn clone(&self) -> Self {
        FloorMap {
            data: self.data.iter().copied().collect(),
            width: self.width,
            height: self.height,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.width = source.width;
        self.height = source.height;
        self.data = source.data.iter().copied().collect();
    }
}

impl FloorMap {
    fn step(&self) -> FloorMap {
        FloorMap {
            data: (0..self.height)
                .cartesian_product(0..self.width)
                .map(|(y, x)| (x, y))
                .map(|pos| match self[pos] {
                    Position::Seat(true) => if self.count_occupied_adj(pos) >= 5 { Position::Seat(false) } else { Position::Seat(true) }
                    Position::Seat(false) => if self.count_occupied_adj(pos) == 0 { Position::Seat(true) } else { Position::Seat(false) }
                    Position::Floor => Position::Floor
                })
                .collect(),
            width: self.width,
            height: self.height,
        }
    }

    fn step_until_complete(&self) -> FloorMap {
        let mut last = self.clone();
        let mut next = self.step();
        while last != next {
            let tmp = next;
            next = tmp.step();
            last = tmp;
        }
        next
    }

    fn count_occupied_adj(&self, pos: (usize, usize)) -> usize {
        let direction_vectors: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        direction_vectors
            .iter()
            .map(|direction| self.find_first_seat_in_direction_from(pos, *direction))
            .filter(|pos| match pos {
                Ok(Position::Seat(true)) => true,
                _ => false
            })
            .count()
    }

    fn find_first_seat_in_direction_from(&self, starting_pos: (usize, usize), direction_vector: (i32, i32)) -> Result<&Position, ()> {
        let mut x = (starting_pos.0 as i32) + direction_vector.0;
        let mut y = (starting_pos.1 as i32) + direction_vector.1;
        loop {
            let val =self.checked_get((x, y));
            match val {
                Ok(Position::Seat(_)) => return val,
                Err(_) => return Err(()),
                _ => (),
            }
            x += direction_vector.0;
            y += direction_vector.1;
        }
    }

    fn checked_get(&self, pos: (i32, i32)) -> Result<&Position, ()> {
        let (x, y) = pos;
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            Err(())
        } else {
            Ok(&self[(x as usize, y as usize)])
        }
    }
}

impl Index<(usize, usize)> for FloorMap {
    type Output = Position;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.data[y * self.width + x]
    }
}


impl FromStr for FloorMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            FloorMap {
                data: s.bytes().filter(|b| *b != b'\n').map(|b| b.into()).collect(),
                width: s.find('\n').expect("There has to be a newline in the input."),
                height: s.bytes().filter(|b| *b == b'\n').count(),
            }
        )
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Copy, Clone)]
enum Position {
    Seat(bool),
    Floor,
}

impl From<u8> for Position {
    fn from(byte: u8) -> Self {
        match byte {
            b'#' => Self::Seat(true),
            b'L' => Self::Seat(false),
            b'.' => Self::Floor,
            _ => unreachable!()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() -> Result<(), String> {
        let data = "#L.\n.L#\n".parse::<FloorMap>()?;
        assert_eq!(data.height, 2);
        assert_eq!(data.width, 3);
        assert_eq!(data[(0, 0)], Position::Seat(true));
        assert_eq!(data[(1, 0)], Position::Seat(false));
        assert_eq!(data[(2, 0)], Position::Floor);
        assert_eq!(data[(0, 1)], Position::Floor);
        assert_eq!(data[(1, 1)], Position::Seat(false));
        assert_eq!(data[(2, 1)], Position::Seat(true));
        Ok(())
    }

    #[test]
    fn test_example_1() -> Result<(), String> {
        let first = ".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....
".parse::<FloorMap>()?;
        assert_eq!(first.count_occupied_adj((3,4)), 8);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<(), String> {
        let first = ".............
.L.L.#.#.#.#.
.............".parse::<FloorMap>()?;
        assert_eq!(first.count_occupied_adj((1,1)), 0);
        Ok(())
    }
    #[test]
    fn test_example_3() -> Result<(), String> {
        let first = ".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.
".parse::<FloorMap>()?;
        assert_eq!(first.count_occupied_adj((3,3)), 0);
        Ok(())
    }
}

fn main() {
    let args = App::new("Day 11 part 2 of AOC 2020!!")
        .arg(Arg::with_name("input-file").takes_value(true))
        .get_matches();
    let input_data: FloorMap = fs::read_to_string(
        args.value_of("input-file").unwrap()
    )
        .unwrap()
        .parse()
        .unwrap();
    println!("{}", input_data.step_until_complete().data.iter().filter(|pos| **pos == Position::Seat(true)).count())
}

