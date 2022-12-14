use std::{cmp::Ordering, collections::BTreeMap};

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 14;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Tile {
    Stone,
    Sand,
}

/*
    Y-
  X-  X+
    Y+
*/
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Position(usize, usize);

impl Position {
    fn get_direction_to(&self, other: &Self) -> Option<Direction> {
        match (other.0.cmp(&self.0), other.1.cmp(&self.1)) {
            (Ordering::Equal, Ordering::Greater) => Some(Direction::Down), // =x y+ DOWN
            (Ordering::Equal, Ordering::Less) => Some(Direction::Up),      // =x y- UP
            (Ordering::Greater, Ordering::Equal) => Some(Direction::Right), //x+ =y RIGHT
            (Ordering::Less, Ordering::Equal) => Some(Direction::Left),    // x- =y LEFT
            _ => None,
        }
    }

    fn move_in_dir(&self, dir: Direction, steps: usize) -> Position {
        match dir {
            Direction::Up => Position(self.0, self.1 - steps),
            Direction::Down => Position(self.0, self.1 + steps),
            Direction::Left => Position(self.0 - steps, self.1),
            Direction::Right => Position(self.0 + steps, self.1),
        }
    }

    fn get_block_distance(&self, other: &Self) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let s = value.split_once(',').unwrap();
        Self(s.0.parse().unwrap(), s.1.parse().unwrap())
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Line {
    a: Position,
    index: usize,
    max_index: usize,
    dir: Direction,
}

impl Line {
    fn new(a: Position, b: Position) -> Self {
        Self {
            a,
            index: 0,
            dir: a.get_direction_to(&b).unwrap(),
            max_index: a.get_block_distance(&b) + 1,
        }
    }
}

impl Iterator for Line {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.max_index {
            None
        } else {
            let pos = self.a.move_in_dir(self.dir, self.index);
            self.index += 1;
            Some(pos)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    tiles: BTreeMap<Position, Tile>,
    deepest_stone_y: usize,
    floor_enabled: bool,
}

impl Map {
    /// Spawns and simulates a sand tile.
    /// Returns true when sand rests and false when sand falls into infinity.
    fn spawn_sand(&mut self) -> bool {
        let mut pos = Position(500, 0);
        let limit = if self.floor_enabled {
            self.deepest_stone_y + 2
        } else {
            self.deepest_stone_y
        };

        while pos.1 < limit {
            match (
                self.is_free(&Position(pos.0 - 1, pos.1 + 1)),
                self.is_free(&Position(pos.0, pos.1 + 1)),
                self.is_free(&Position(pos.0 + 1, pos.1 + 1)),
            ) {
                (_, true, _) => {
                    //println!("Straight down.");
                    pos.1 += 1;
                }

                (true, false, _) => {
                    //println!("Diagonally left");
                    pos.1 += 1;
                    pos.0 -= 1;
                }
                (false, false, true) => {
                    //println!("Diagonally right");
                    pos.1 += 1;
                    pos.0 += 1;
                }
                _ => {
                    //println!("Came to rest at {:?}", pos);
                    self.tiles.insert(pos, Tile::Sand);
                    if self.floor_enabled && pos == Position(500, 0) {
                        return false;
                    }
                    return true;
                }
            }
        }

        false
    }

    fn is_free(&self, pos: &Position) -> bool {
        !(self.tiles.contains_key(pos) || (self.floor_enabled && pos.1 == self.deepest_stone_y + 2))
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut tiles = BTreeMap::new();
        let mut deepest_stone_y = 0;

        for l in value.lines() {
            let mut prev = None;
            for p in l.split(" -> ") {
                let pos = Position::from(p);

                if let Some(prev_pos) = prev {
                    let l = Line::new(prev_pos, pos);
                    for p in l {
                        tiles.insert(p, Tile::Stone);
                    }
                }

                if pos.1 > deepest_stone_y {
                    deepest_stone_y = pos.1;
                }
                prev = Some(pos);
            }
        }

        Self {
            tiles,
            deepest_stone_y,
            floor_enabled: false,
        }
    }
}

type Data = Map;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test14.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(24), Answer::Number(93))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, Map::from(input))
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut i = 0;
        while data.spawn_sand() {
            i += 1;
        }
        Answer::Number(i)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut i = 1;

        data.floor_enabled = true;
        while data.spawn_sand() {
            i += 1;
        }
        Answer::Number(i)
    }
}
