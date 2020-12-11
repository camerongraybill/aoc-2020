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
                    Position::Seat(true) => if self.count_occupied_adj(pos) >= 4 { Position::Seat(false) } else { Position::Seat(true) }
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
        let x = pos.0 as i32;
        let y = pos.1 as i32;
        let indicies: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        indicies
            .iter()
            .map(|(iter_x, iter_y)| self.checked_get((x + iter_x, y + iter_y)))
            .filter(|pos| match pos {
                Ok(Position::Seat(true)) => true,
                _ => false
            })
            .count()
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
    fn test_example() -> Result<(), String> {
        let first = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
".parse::<FloorMap>()?;
        let second = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
".parse::<FloorMap>()?;
        let third = "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##
".parse::<FloorMap>()?;
        assert_eq!(first.step(), second);
        assert_eq!(second.step(), third);
        assert_eq!(first.step_until_complete().data.iter().filter(|pos| **pos == Position::Seat(true)).count(), 37);
        Ok(())
    }
}

fn main() {
    let args = App::new("Day 11 part 1 of AOC 2020!!")
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

