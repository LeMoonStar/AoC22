use std::{
    collections::{BTreeMap, BTreeSet},
    hash::Hash,
    ops::{Add, Sub},
};

use super::{Answer, Day, DayImpl};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Left,
    Right,
    Down,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '<' => Self::Left,
            '>' => Self::Right,
            _ => unreachable!("Unexpected input value."),
        }
    }
}

//   Y+
// X-  X+
//   Y-

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vec2D(isize, isize);

impl Add for Vec2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Vec2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Rock {
    HorizontalLine,
    Plus,
    BackwardsL,
    VerticalLine,
    Square,
}

impl Rock {
    #[inline]
    fn from_rock_count(rock_count: usize) -> Self {
        match rock_count % 5 {
            0 => Self::HorizontalLine,
            1 => Self::Plus,
            2 => Self::BackwardsL,
            3 => Self::VerticalLine,
            4 => Self::Square,
            _ => unreachable!(),
        }
    }

    #[inline]
    fn get_dimensions(&self) -> Vec2D {
        match self {
            Self::HorizontalLine => Vec2D(4, 1),
            Self::Plus => Vec2D(3, 3),
            Self::BackwardsL => Vec2D(3, 3),
            Self::VerticalLine => Vec2D(1, 4),
            Self::Square => Vec2D(2, 2),
        }
    }

    /// Returns a BTreeSet containing all points part of the rock.
    /// The origin is always located in the lower left.
    #[inline]
    fn get_shape(&self) -> BTreeSet<Vec2D> {
        match self {
            // 0123
            //0####
            Self::HorizontalLine => {
                BTreeSet::from([Vec2D(0, 0), Vec2D(1, 0), Vec2D(2, 0), Vec2D(3, 0)])
            }
            // 012
            //2 #
            //1###
            //0 #
            Self::Plus => BTreeSet::from([
                Vec2D(1, 0),
                Vec2D(0, 1),
                Vec2D(1, 1),
                Vec2D(2, 1),
                Vec2D(1, 2),
            ]),
            // 012
            //2  #
            //1  #
            //0###
            Self::BackwardsL => BTreeSet::from([
                Vec2D(2, 2),
                Vec2D(2, 1),
                Vec2D(0, 0),
                Vec2D(1, 0),
                Vec2D(2, 0),
            ]),
            // 0
            //3#
            //2#
            //1#
            //0#
            Self::VerticalLine => {
                BTreeSet::from([Vec2D(0, 0), Vec2D(0, 1), Vec2D(0, 2), Vec2D(0, 3)])
            }
            // 01
            //1##
            //0##
            Self::Square => BTreeSet::from([Vec2D(0, 0), Vec2D(1, 0), Vec2D(0, 1), Vec2D(1, 1)]),
        }
    }

    /*fn check_if_intersects(&self, rel_pos: Vec2D) -> bool {
        // Simple first box intersection test // Done by other parts already.
        let dim = self.get_dimensions();
        if rel_pos.0 < 0 || rel_pos.1 < 0 || rel_pos.0 > dim.0 || rel_pos.1 > dim.1 {
            return false;
        }

        // More accurate exact intersection test
        let shape = self.get_shape();
        shape.contains(&rel_pos)
    }*/

    fn check_if_placable(&self, pos: Vec2D, placed: &BTreeMap<Vec2D, Rock>) -> bool {
        for other in placed {
            if self.check_if_intersects_with(pos, other.1, *other.0) {
                return false;
            }
        }
        true
    }

    fn check_if_intersects_with(&self, pos: Vec2D, other: &Self, other_pos: Vec2D) -> bool {
        let own_dim = self.get_dimensions();
        let other_dim = other.get_dimensions();
        let x_padding = (own_dim.0 + other_dim.0) as usize;
        let y_padding = (own_dim.1 + other_dim.1) as usize;

        if pos.0.abs_diff(other_pos.0) > x_padding || pos.1.abs_diff(other_pos.1) > y_padding {
            return false;
        }

        let own_shape = self.get_shape();
        let other_shape = other.get_shape();

        let rel_pos = pos - other_pos;

        for p in own_shape {
            if other_shape.contains(&(p + rel_pos)) {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, Clone)]
struct Chamber {
    width: u8,
    placed: BTreeMap<Vec2D, Rock>,
    stack_height: usize,
    rock_count: usize,
    moves_count: usize,
    wind_loop: Vec<Direction>,
}

impl Chamber {
    fn new(width: u8, wind_loop: Vec<Direction>) -> Self {
        Self {
            width,
            placed: BTreeMap::new(),
            stack_height: 0,
            rock_count: 0,
            moves_count: 0,
            wind_loop,
        }
    }

    fn check_move_in_dir(&self, rock: Rock, pos: Vec2D, direction: Direction) -> bool {
        let dim = rock.get_dimensions();
        match direction {
            Direction::Left => {
                if pos.0 == 0 {
                    return false;
                }
                rock.check_if_placable(Vec2D(pos.0 - 1, pos.1), &self.placed)
            }
            Direction::Right => {
                if pos.0 + dim.0 >= self.width as isize {
                    return false;
                }
                rock.check_if_placable(Vec2D(pos.0 + 1, pos.1), &self.placed)
            }
            Direction::Down => rock.check_if_placable(Vec2D(pos.0, pos.1 - 1), &self.placed),
        }
    }

    /*fn draw_all(&self, rock: Option<Rock>, pos: Option<Vec2D>) {
        let (y_start, y_end) = if let Some(pos) = pos {
            ((pos.1 as usize).saturating_sub(10), (pos.1 + 10) as usize)
        } else {
            (0, self.stack_height + 10)
        };

        for y in (y_start..y_end).rev() {
            print!("{:05}|", y);
            for x in 0..self.width + 1 {
                let c_pos = Vec2D(x as isize, y as isize);
                if let Some(r) = rock {
                    if r.check_if_intersects(c_pos - pos.unwrap()) {
                        print!("@");
                        continue;
                    }
                }
                if self.placed.contains(&c_pos) {
                    print!("#");
                } else if x == self.width {
                    print!("|")
                } else if y == 0 {
                    print!("-")
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }*/

    fn spawn_rock(&mut self) {
        let mut pos = Vec2D(2, (self.stack_height + 4) as isize);
        let rock = Rock::from_rock_count(self.rock_count);
        let rock_dimensions = rock.get_dimensions();
        self.rock_count += 1;

        loop {
            let wind = self.wind_loop[self.moves_count % self.wind_loop.len()];
            self.moves_count += 1;

            if self.check_move_in_dir(rock, pos, wind) {
                match wind {
                    Direction::Left => {
                        pos.0 -= 1;
                    }
                    Direction::Right => {
                        pos.0 += 1;
                    }
                    _ => unreachable!("invalid wind direction."),
                }
            }

            if pos.1 == 1 || !self.check_move_in_dir(rock, pos, Direction::Down) {
                let n_height = (pos.1 + rock_dimensions.1) as usize - 1;
                if n_height > self.stack_height {
                    self.stack_height = n_height;
                }
                self.placed.insert(pos, rock);
                break;
            }

            pos.1 -= 1;
        }
    }
}

const CURRENT_DAY: u8 = 17;

type Data = Vec<Direction>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test17.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(3068), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.chars().map(Direction::from).collect())
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut chamber = Chamber::new(7, data.clone());
        for _ in 0..2022 {
            chamber.spawn_rock();
        }
        Answer::Number(chamber.stack_height as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(0)
    }
}
