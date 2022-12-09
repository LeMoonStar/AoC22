use std::ops::{Add, AddAssign, Sub};

use super::{Answer, Day, DayImpl};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Move {
    Right(i64),
    Left(i64),
    Up(i64),
    Down(i64),
}

impl Move {
    fn get_as_position(&self) -> Position {
        match self {
            Self::Right(d) => Position { x: *d, y: 0 },
            Self::Left(d) => Position { x: -d, y: 0 },
            Self::Up(d) => Position { x: 0, y: *d },
            Self::Down(d) => Position { x: 0, y: -*d },
        }
    }

    fn get_direction(&self) -> Position {
        match self {
            Self::Right(_) => Position { x: 1, y: 0 },
            Self::Left(_) => Position { x: -1, y: 0 },
            Self::Up(_) => Position { x: 0, y: 1 },
            Self::Down(_) => Position { x: 0, y: -1 },
        }
    }
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        let mut s = value.split_ascii_whitespace();
        match s.next().unwrap() {
            "R" => Self::Right(s.next().unwrap().parse().unwrap()),
            "L" => Self::Left(s.next().unwrap().parse().unwrap()),
            "U" => Self::Up(s.next().unwrap().parse().unwrap()),
            "D" => Self::Down(s.next().unwrap().parse().unwrap()),
            _ => panic!("unexpected Move"),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new() -> Self {
        Position { x: 0, y: 0 }
    }

    fn is_touching(&self, other: &Self) -> bool {
        let d = *self - *other;

        (d.x.abs() < 2) && (d.y.abs() < 2)
    }

    fn get_direction(&self) -> Position {
        Self {
            x: self.x.clamp(-1, 1),
            y: self.y.clamp(-1, 1),
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

#[derive(Debug, Clone, Copy)]
struct Knot {
    pos: Position,
}

impl Knot {
    fn new() -> Self {
        Self {
            pos: Position::new(),
        }
    }

    fn follow(&mut self, other: &Self, visited: Option<&mut HashSet<Position>>) {
        while !self.pos.is_touching(&other.pos) {
            self.pos += (other.pos - self.pos).get_direction();
        }
        if let Some(s) = visited {
            s.insert(self.pos);
        }
    }
}

#[derive(Debug, Clone)]
struct Chain {
    knots: Vec<Knot>,
    visited: HashSet<Position>,
}

impl Chain {
    fn new(len: usize) -> Self {
        Self {
            knots: vec![Knot::new(); len],
            visited: HashSet::from([Position::new()]),
        }
    }

    fn execute_move(&mut self, m: &Move) {
        let goal = m.get_as_position() + self.knots[0].pos;
        let step = m.get_direction();

        while goal != self.knots[0].pos {
            self.knots[0].pos += step;

            let n = self.knots.len();
            let mut prev = self.knots[0];
            for i in 1..n {
                self.knots[i].follow(
                    &prev,
                    // And this is where I messed up Part 2... I thought every knot of the tail was supposed to count towards the set... but noooo, just the last one, thank you.
                    if i == n - 1 {
                        Some(&mut self.visited)
                    } else {
                        None
                    },
                );
                prev = self.knots[i];
            }
        }
    }

    fn get_visited_count(&self) -> usize {
        self.visited.len()
    }
}

const CURRENT_DAY: u8 = 9;

type Data = Vec<Move>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test09.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(88), Answer::Number(36))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.lines().map(Move::from).collect())
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut chain = Chain::new(2);

        for m in data {
            chain.execute_move(m);
        }

        Answer::Number(chain.get_visited_count() as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut chain = Chain::new(10);

        for m in data {
            chain.execute_move(m);
        }

        Answer::Number(chain.get_visited_count() as u64)
    }
}
