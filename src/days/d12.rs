use std::{
    collections::BTreeMap,
    ops::{Add, AddAssign},
};

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 12;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn get_as_pos(&self) -> Position {
        match self {
            Direction::Up => Position(0, 1),
            Direction::Down => Position(0, -1),
            Direction::Right => Position(1, 0),
            Direction::Left => Position(-1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Position(i32, i32);

impl Position {
    fn move_in_direction(self, dir: Direction) -> Position {
        self + dir.get_as_pos()
    }

    fn get_distance(&self, other: &Position) -> u64 {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as u64
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    start: Position,
    goal: Position,
    heights: Vec<Vec<u8>>,
}

impl Map {
    fn get_field(&self, pos: Position) -> Option<u8> {
        if let Some(row) = self.heights.get(pos.1 as usize) {
            row.get(pos.0 as usize).copied()
        } else {
            None
        }
    }

    fn get_neighbors_up(&self, pos: Position) -> Vec<Position> {
        let mut accessible = Vec::new();
        let own_h = self.get_field(pos).unwrap();

        for d in [
            Direction::Up,
            Direction::Down,
            Direction::Right,
            Direction::Left,
        ] {
            let n_pos = pos.move_in_direction(d);
            if let Some(h) = self.get_field(n_pos) {
                if h <= own_h + 1 {
                    accessible.push(n_pos);
                }
            }
        }

        accessible
    }

    fn get_neighbors_down(&self, pos: Position) -> Vec<Position> {
        let mut accessible = Vec::new();
        let own_h = self.get_field(pos).unwrap();

        for d in [
            Direction::Up,
            Direction::Down,
            Direction::Right,
            Direction::Left,
        ] {
            let n_pos = pos.move_in_direction(d);
            if let Some(h) = self.get_field(n_pos) {
                if h >= own_h.saturating_sub(1) {
                    accessible.push(n_pos);
                }
            }
        }

        accessible
    }

    // quick, uncommented implementation of the A* algorithm based on the Wikipedia Article's pseudocode.
    fn find_path(&self, start: Position) -> Option<Vec<Position>> {
        let mut open_set = vec![start];
        let mut came_from = BTreeMap::new();

        let mut g_score = BTreeMap::new();
        g_score.insert(start, 0);

        let mut f_score = BTreeMap::new();
        f_score.insert(start, start.get_distance(&self.goal));

        while !open_set.is_empty() {
            let current = *open_set.first().unwrap();
            if current == self.goal {
                return Some(self.reconstruct_path(&came_from, current));
            }
            open_set.remove(0);
            for neighbor in self.get_neighbors_up(current) {
                let tentative_gscore = *g_score.get(&current).get_or_insert(&u64::MAX) + 1;

                if tentative_gscore < **g_score.get(&neighbor).get_or_insert(&u64::MAX) {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_gscore);
                    f_score.insert(
                        neighbor,
                        tentative_gscore + self.goal.get_distance(&neighbor),
                    );

                    if !open_set.contains(&neighbor) {
                        open_set.push(neighbor);
                        open_set
                            .sort_by(|a, b| f_score.get(a).unwrap().cmp(f_score.get(b).unwrap()));
                    }
                }
            }
        }
        None
    }

    // Jerome was here :)
    fn search_from_end(&self, start: Position, goal: u8) -> Option<Vec<Position>> {
        let mut open_set = vec![start];
        let mut came_from = BTreeMap::new();

        let mut g_score = BTreeMap::new();
        g_score.insert(start, 0);

        while !open_set.is_empty() {
            let current = *open_set.first().unwrap();
            if self.get_field(current).unwrap() == goal {
                return Some(self.reconstruct_path(&came_from, current));
            }
            open_set.remove(0);
            for neighbor in self.get_neighbors_down(current) {
                let tentative_gscore = *g_score.get(&current).get_or_insert(&u64::MAX) + 1;

                if tentative_gscore < **g_score.get(&neighbor).get_or_insert(&u64::MAX) {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_gscore);

                    if !open_set.contains(&neighbor) {
                        open_set.push(neighbor);
                    }
                }
            }
        }
        None
    }

    fn reconstruct_path(
        &self,
        came_from: &BTreeMap<Position, Position>,
        mut current: Position,
    ) -> Vec<Position> {
        let mut total_path = vec![current];

        while came_from.contains_key(&current) {
            current = *came_from.get(&current).unwrap();
            total_path.push(current);
        }
        total_path.reverse();
        total_path
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut start: Position = Position(0, 0);
        let mut goal: Position = Position(0, 0);

        let mut heights = Vec::new();
        for (y, line) in value.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                if c.is_uppercase() {
                    match c {
                        'S' => {
                            start = Position(x as i32, y as i32);
                            row.push(0)
                        }
                        'E' => {
                            goal = Position(x as i32, y as i32);
                            row.push(25)
                        }
                        _ => unreachable!("Unexpected Input"),
                    }
                } else {
                    row.push(c as u8 - 97);
                }
            }
            heights.push(row);
        }

        Self {
            start,
            goal,
            heights,
        }
    }
}

type Data = Map;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test12.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(31), Answer::Number(29))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, Map::from(input))
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number((data.find_path(data.start).unwrap().len() - 1) as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number((data.search_from_end(data.goal, 0).unwrap().len() - 1) as u64)
    }
}
