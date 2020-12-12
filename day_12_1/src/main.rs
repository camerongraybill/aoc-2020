use clap::{Arg, App};
use std::fs;
use std::str::FromStr;

#[derive(Copy, Clone)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

fn direction_from_degrees(degrees: i32) -> Direction {
    match ((degrees + 360 * 2) % 360) / 90 {
        0 => Direction::North,
        1 => Direction::East,
        2 => Direction::South,
        3 => Direction::West,
        _ => {println!("{}", degrees); unreachable!()}
    }
}

fn turn_direction(direction: Direction, degrees: i32) -> Direction {
    direction_from_degrees((direction as i32) * 90 + degrees)
}

struct ShipState {
    x_pos: i32,
    y_pos: i32,
    facing: Direction
}

impl ShipState {

    fn new() -> ShipState {
        ShipState {
            x_pos: 0,
            y_pos: 0,
            facing: Direction::East,
        }
    }

    fn apply_action(&self, action: Action) -> ShipState {
        match action {
            Action::MoveDirection(direction, amount) => {
                ShipState {
                    x_pos: match direction {
                        Direction::North => self.x_pos + (amount as i32),
                        Direction::South => self.x_pos - (amount as i32),
                        _ => self.x_pos,
                    },
                    y_pos: match direction {
                        Direction::East => self.y_pos + (amount as i32),
                        Direction::West => self.y_pos - (amount as i32),
                        _ => self.y_pos,
                    },
                    facing: self.facing
                }
            }
            Action::Left(degrees) => {
                ShipState {
                    x_pos: self.x_pos,
                    y_pos: self.y_pos,
                    facing: turn_direction(self.facing, -1 * (degrees as i32))
                }
            }
            Action::Right(degrees) => {
                ShipState {
                    x_pos: self.x_pos,
                    y_pos: self.y_pos,
                    facing: turn_direction(self.facing, degrees as i32)
                }
            }
            Action::Forward(amount) => self.apply_action(Action::MoveDirection(self.facing, amount))
        }
    }

    fn manhattan_distance(&self) -> usize {
        (self.y_pos.abs() + self.x_pos.abs()) as usize
    }
}

enum Action {
    MoveDirection(Direction, u16),
    Left(u16),
    Right(u16),
    Forward(u16),
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s[1..].parse::<u16>().unwrap();
        match s.bytes().next().unwrap() {
            b'N' => Ok(Action::MoveDirection(Direction::North, val)),
            b'S' => Ok(Action::MoveDirection(Direction::South, val)),
            b'E' => Ok(Action::MoveDirection(Direction::East, val)),
            b'W' => Ok(Action::MoveDirection(Direction::West, val)),
            b'L' => Ok(Action::Left(val)),
            b'R' => Ok(Action::Right(val)),
            b'F' => Ok(Action::Forward(val)),
            _ => Err("Unexpected Character".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() -> Result<(), String> {
        Ok(())
    }

}

fn main() {
    let args = App::new("Day 12 part 1 of AOC 2020!!")
        .arg(Arg::with_name("input-file").takes_value(true))
        .get_matches();
    let ship_state = fs::read_to_string(
        args.value_of("input-file").unwrap()
    )
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .fold(ShipState::new(), |ship_state, action| ship_state.apply_action(action));
    println!("{}", ship_state.manhattan_distance())
}

