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
    fn touches_center(&self) -> bool {
        ////println!("({} | {})", self.x.abs(), self.y.abs());
        (self.x.abs() < 2) && (self.y.abs() < 2)
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

const CURRENT_DAY: u8 = 9;

type Data = Vec<Move>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test09.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(13), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.lines().map(Move::from).collect())
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut head = Position { x: 0, y: 0 };
        let mut tail = Position { x: 0, y: 0 };
        let mut visited = HashSet::from([Position { x: 0, y: 0 }]);

        for m in data {
            //println!("{:?} from {:?}", m, head);
            let goal = head + m.get_as_position();
            let step = m.get_direction();

            while head != goal {
                head += step;
                let distance = head - tail;
                let direction = distance.get_direction();
                //println!("  Distance:     {:?}", distance);
                //println!("    HEAD:       {:?}", head);
                //println!("    TAIL:       {:?}", tail);
                if !distance.touches_center() {
                    //println!("    --DON'T TOUCH");
                    tail += direction;
                    //println!("      MOVED TO: {:?}", tail);
                    visited.insert(tail);
                }
            }
            //println!();
        }

        Answer::Number(visited.len() as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(0)
    }
}
