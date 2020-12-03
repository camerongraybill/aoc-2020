use clap::{Arg, App};
use std::str::FromStr;
use std::num::ParseIntError;
use std::ops::Index;
use std::fs;

struct Map {
    data: Vec<bool>,
    width: usize,
    height: usize,
}

struct MapIter {
    slope: (usize, usize),
    curr_pos: (usize, usize),
    map: Map,
}

impl Iterator for MapIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr_pos = (self.curr_pos.0 + self.slope.0, self.curr_pos.1 + self.slope.1);
        if self.curr_pos.1 >= self.map.height {
            None
        } else {
            Some(self.map[self.curr_pos])
        }
    }
}

impl Index<(usize, usize)> for Map {
    type Output = bool;
    // True value means there IS a tree there
    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        let x = idx.0 % self.width;
        let y = idx.1;
        &self.data[x + self.width * y]
    }
}

impl FromStr for Map {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.find('\n').unwrap();
        let data: Vec<bool> = s.chars().filter(|c| *c != '\n').map(|c| c == '#').collect();
        let height = data.len() / width;
        Ok(Map {
            data,
            width,
            height,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map_fromstr() -> Result<(), String> {
        let map = ".#.#\n#.##\n.#.#\n".parse::<Map>().unwrap();
        assert_eq!(map.height, 3);
        assert_eq!(map.width, 4);
        assert_eq!(map.data.len(), 12);
        Ok(())
    }
    #[test]
    fn test_map_index() -> Result<(), String> {
        let map = ".#\n#.\n".parse::<Map>().unwrap();
        assert_eq!(map[(0,0)], false);
        assert_eq!(map[(1,0)], true);
        assert_eq!(map[(0,1)], true);
        assert_eq!(map[(1,1)], false);
        assert_eq!(map[(2,0)], false);
        assert_eq!(map[(2,1)], true);
        assert_eq!(map[(3,1)], false);
        assert_eq!(map[(3,0)], true);
        Ok(())
    }
    #[test]
    fn test_map_iter() -> Result<(), String> {
        let mut iter = MapIter {
            slope: (0,1),
            curr_pos: (0,0),
            map: ".#\n#.\n".parse::<Map>().unwrap()
        };
        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next(), None);
        Ok(())
    }
}

fn count_collisions(
    map: Map,
    slope: (usize, usize),
) -> usize {
    MapIter {
        slope,
        curr_pos: (0, 0),
        map,
    }
        .filter(|b| *b)
        .count()
}


fn main() {
    let args = App::new("Day three part one of AOC 2020!!")
        .arg(Arg::with_name("input-file").takes_value(true))
        .arg(Arg::with_name("x-slope").takes_value(true))
        .arg(Arg::with_name("y-slope").takes_value(true))
        .get_matches();
    let map = fs::read_to_string(args.value_of("input-file").unwrap()).unwrap().parse::<Map>().unwrap();
    println!("{}", count_collisions(
        map,
        (
            args.value_of("x-slope").unwrap().parse::<usize>().unwrap(),
            args.value_of("y-slope").unwrap().parse::<usize>().unwrap(),
        ),
    )
    );
}

