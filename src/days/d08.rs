use super::{Answer, Day, DayImpl};
use crate::dprintln;

const CURRENT_DAY: u8 = 8;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn check_in_direction(field: &Data, mut x: usize, mut y: usize, direction: Direction) -> bool {
    let width = field[0].len();
    let height = field.len();
    let own_height = field[y][x];

    loop {
        if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
            dprintln!("  {:?} Visible", direction);
            return true;
        }
        match direction {
            Direction::Up => {
                y -= 1;
            }
            Direction::Down => {
                y += 1;
            }
            Direction::Left => {
                x -= 1;
            }
            Direction::Right => {
                x += 1;
            }
        }
        dprintln!("    [{}|{}]", x, y);

        dprintln!("    {} >= {} ?", field[y][x], own_height);
        if field[y][x] >= own_height {
            dprintln!(
                "  {:?} Not visible ({} >= {})",
                direction,
                field[y][x],
                own_height
            );
            return false;
        }
    }
}

fn check_if_visible(field: &Data, x: usize, y: usize) -> bool {
    check_in_direction(field, x, y, Direction::Up)
        || check_in_direction(field, x, y, Direction::Right)
        || check_in_direction(field, x, y, Direction::Down)
        || check_in_direction(field, x, y, Direction::Left)
}

fn get_distance_in_direction(
    field: &Data,
    mut x: usize,
    mut y: usize,
    direction: Direction,
) -> u64 {
    let width = field[0].len();
    let height = field.len();
    let own_height = field[y][x];
    let mut distance = 0;

    loop {
        if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
            return distance;
        }
        match direction {
            Direction::Up => {
                y -= 1;
            }
            Direction::Down => {
                y += 1;
            }
            Direction::Left => {
                x -= 1;
            }
            Direction::Right => {
                x += 1;
            }
        }
        distance += 1;

        if field[y][x] >= own_height {
            return distance;
        }
    }
}

fn get_scenic_score(field: &Data, x: usize, y: usize) -> u64 {
    get_distance_in_direction(field, x, y, Direction::Up)
        * get_distance_in_direction(field, x, y, Direction::Down)
        * get_distance_in_direction(field, x, y, Direction::Left)
        * get_distance_in_direction(field, x, y, Direction::Right)
}

type Data = Vec<Vec<u32>>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test08.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(21), Answer::Number(8))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .lines()
                .map(|v| {
                    v.chars()
                        .collect::<Vec<char>>()
                        .iter()
                        .map(|v| v.to_digit(10).unwrap())
                        .collect()
                })
                .collect::<Vec<Vec<u32>>>(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut visible = 0;

        let width = data[0].len();
        let height = data.len();

        for y in 0..height {
            for x in 0..width {
                dprintln!("({})  {}|{}", data[y][x], x, y);

                if y == 0
                    || x == 0
                    || y == height - 1
                    || x == width - 1
                    || check_if_visible(data, x, y)
                {
                    dprintln!("==VISIBLE==");
                    // The tree is on the edge, it therefore is visible.
                    visible += 1;
                }
            }
        }

        Answer::Number(visible)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut best_score = 0;

        let width = data[0].len();
        let height = data.len();

        for y in 0..height {
            for x in 0..width {
                let s = get_scenic_score(data, x, y);
                if s > best_score {
                    best_score = s;
                }
            }
        }

        Answer::Number(best_score)
    }
}
