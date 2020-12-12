use clap::{Arg, App};
use std::fs;
use std::str::FromStr;

type CoordInt = i128;

#[derive(Copy, Clone)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

#[derive(Debug)]
struct ShipState {
    x_pos: CoordInt,
    y_pos: CoordInt,
    waypoint_x: CoordInt,
    waypoint_y: CoordInt,
}

impl ShipState {

    fn new() -> ShipState {
        ShipState {
            x_pos: 0,
            y_pos: 0,
            waypoint_x: 10,
            waypoint_y: 1,
        }
    }

    fn rotate_waypoint_by(self, degrees: CoordInt) -> ShipState {
        (0..(((degrees + 360 * 2) % 360) / 90))
            .fold(self, |state, _| state.rotate_waypoint_by_90())
    }

    fn rotate_waypoint_by_90(self) -> ShipState {
        ShipState {
            x_pos: self.x_pos,
            y_pos: self.y_pos,
            waypoint_x: self.waypoint_y,
            waypoint_y: self.waypoint_x * -1,
        }
    }


    fn apply_action(self, action: Action) -> ShipState {
        match action {
            Action::MoveWaypoint(direction, amount) => {
                ShipState {
                    x_pos: self.x_pos,
                    y_pos: self.y_pos,
                    waypoint_x: match direction {
                        Direction::East => self.waypoint_x + (amount as CoordInt),
                        Direction::West => self.waypoint_x - (amount as CoordInt),
                        _ => self.waypoint_x,
                    },
                    waypoint_y: match direction {
                        Direction::North => self.waypoint_y + (amount as CoordInt),
                        Direction::South => self.waypoint_y - (amount as CoordInt),
                        _ => self.waypoint_y,
                    },
                }
            }
            Action::Left(degrees) => self.rotate_waypoint_by(-1 * (degrees as CoordInt)),
            Action::Right(degrees) => self.rotate_waypoint_by(degrees as CoordInt),
            Action::Forward(amount) => {
                ShipState {
                    x_pos: self.x_pos + (amount as CoordInt) * self.waypoint_x,
                    y_pos: self.y_pos + (amount as CoordInt) * self.waypoint_y,
                    waypoint_y: self.waypoint_y,
                    waypoint_x: self.waypoint_x,
                }
            }
        }
    }

    fn manhattan_distance(self) -> usize {
        (self.y_pos.abs() + self.x_pos.abs()) as usize
    }
}

enum Action {
    MoveWaypoint(Direction, u16),
    Left(u16),
    Right(u16),
    Forward(u16),
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s[1..].parse::<u16>().unwrap();
        match s.bytes().next().unwrap() {
            b'N' => Ok(Action::MoveWaypoint(Direction::North, val)),
            b'S' => Ok(Action::MoveWaypoint(Direction::South, val)),
            b'E' => Ok(Action::MoveWaypoint(Direction::East, val)),
            b'W' => Ok(Action::MoveWaypoint(Direction::West, val)),
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
    fn test_example() -> Result<(), String> {
        let mut ship_state = ShipState::new();
        assert_eq!(ship_state.y_pos, 0);
        assert_eq!(ship_state.x_pos, 0);
        assert_eq!(ship_state.waypoint_x, 10);
        assert_eq!(ship_state.waypoint_y, 1);
        ship_state = ship_state.apply_action(Action::Forward(10));
        assert_eq!(ship_state.y_pos, 10);
        assert_eq!(ship_state.x_pos, 100);
        assert_eq!(ship_state.waypoint_x, 10);
        assert_eq!(ship_state.waypoint_y, 1);
        ship_state = ship_state.apply_action(Action::MoveWaypoint(Direction::North, 3));
        assert_eq!(ship_state.y_pos, 10);
        assert_eq!(ship_state.x_pos, 100);
        assert_eq!(ship_state.waypoint_x, 10);
        assert_eq!(ship_state.waypoint_y, 4);
        ship_state = ship_state.apply_action(Action::Forward(7));
        assert_eq!(ship_state.y_pos, 38);
        assert_eq!(ship_state.x_pos, 170);
        assert_eq!(ship_state.waypoint_x, 10);
        assert_eq!(ship_state.waypoint_y, 4);
        ship_state = ship_state.apply_action(Action::Right(90));
        assert_eq!(ship_state.y_pos, 38);
        assert_eq!(ship_state.x_pos, 170);
        assert_eq!(ship_state.waypoint_x, 4);
        assert_eq!(ship_state.waypoint_y, -10);
        ship_state = ship_state.apply_action(Action::Forward(11));
        assert_eq!(ship_state.y_pos, -72);
        assert_eq!(ship_state.x_pos, 214);
        assert_eq!(ship_state.waypoint_x, 4);
        assert_eq!(ship_state.waypoint_y, -10);
        ship_state = ship_state.apply_action(Action::Left(180));
        assert_eq!(ship_state.y_pos, -72);
        assert_eq!(ship_state.x_pos, 214);
        assert_eq!(ship_state.waypoint_x, -4);
        assert_eq!(ship_state.waypoint_y, 10);


        assert_eq!(ship_state.manhattan_distance(), 286);
        Ok(())
    }

}

fn main() {
    let args = App::new("Day 12 part 2 of AOC 2020!!")
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

