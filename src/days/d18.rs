use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 18;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: i8,
    y: i8,
    z: i8,
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let (x, s) = value.split_once(',').unwrap();
        let (y, z) = s.split_once(',').unwrap();
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            z: z.parse().unwrap(),
        }
    }
}

struct FaceNeighbors {
    pos: Position,
    n: u8,
}

impl FaceNeighbors {
    fn new(pos: Position) -> Self {
        Self { pos, n: 0 }
    }
}

impl Iterator for FaceNeighbors {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        match self.n {
            0 => {
                self.n += 1;
                Some(self.pos + Position { x: 1, y: 0, z: 0 })
            }
            1 => {
                self.n += 1;
                Some(self.pos + Position { x: 0, y: 1, z: 0 })
            }
            2 => {
                self.n += 1;
                Some(self.pos + Position { x: -1, y: 0, z: 0 })
            }
            3 => {
                self.n += 1;
                Some(self.pos + Position { x: 0, y: -1, z: 0 })
            }
            4 => {
                self.n += 1;
                Some(self.pos + Position { x: 0, y: 0, z: 1 })
            }
            5 => {
                self.n += 1;
                Some(self.pos + Position { x: 0, y: 0, z: -1 })
            }
            _ => None,
        }
    }
}

fn check_outside_connection(
    pos: Position,
    lava: &HashSet<Position>,
    boundaries: Position,
    visited: &mut HashSet<Position>,
) -> bool {
    visited.insert(pos);

    if lava.contains(&pos) {
        return false;
    }

    if pos.x < 0
        || pos.y < 0
        || pos.z < 0
        || pos.x > boundaries.x
        || pos.y > boundaries.y
        || pos.z > boundaries.z
    {
        return true;
    }
    for n in FaceNeighbors::new(pos) {
        if !visited.contains(&n) && check_outside_connection(n, lava, boundaries, visited) {
            return true;
        }
    }
    false
}

type Data = HashSet<Position>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test18.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(64), Answer::Number(58))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.lines().map(Position::from).collect())
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut n = 0;

        for p in data.iter() {
            for neighbor in FaceNeighbors::new(*p) {
                if !data.contains(&neighbor) {
                    n += 1;
                }
            }
        }

        Answer::Number(n)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut n = 0;
        let mut max = Position { x: 0, y: 0, z: 0 };

        for p in data.iter() {
            if p.x > max.x {
                max.x = p.x;
            }
            if p.y > max.y {
                max.y = p.y;
            }
            if p.z > max.z {
                max.z = p.z;
            }
        }

        for p in data.iter() {
            for neighbor in FaceNeighbors::new(*p) {
                if !data.contains(&neighbor)
                    && check_outside_connection(neighbor, data, max, &mut HashSet::new())
                {
                    n += 1;
                }
            }
        }

        Answer::Number(n)
    }
}
