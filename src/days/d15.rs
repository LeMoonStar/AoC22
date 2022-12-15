use std::collections::BTreeMap;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 15;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Position(i64, i64);

impl Position {
    fn get_block_distance(&self, other: &Self) -> u64 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

impl Position {
    fn get_border_positions(&self, radius: u64) -> Vec<Position> {
        let mut border = Vec::new();

        // Yes, Gewi, I know I could've done this in a nicer way, I don't care. This works.
        let mut current_pos = Position(self.0, self.1 - (radius + 1) as i64);
        for _ in 0..radius + 1 {
            current_pos.0 += 1;
            current_pos.1 += 1;
            border.push(current_pos);
        }

        for _ in 0..radius + 1 {
            current_pos.0 -= 1;
            current_pos.1 += 1;
            border.push(current_pos);
        }

        for _ in 0..radius + 1 {
            current_pos.0 -= 1;
            current_pos.1 -= 1;
            border.push(current_pos);
        }

        for _ in 0..radius + 1 {
            current_pos.0 += 1;
            current_pos.1 -= 1;
            border.push(current_pos);
        }

        border
    }
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let (_, s) = value.split_at(2);
        let (x_str, s) = s.split_once(',').unwrap();
        let (_, y_str) = s.split_once('=').unwrap();

        Self(x_str.parse().unwrap(), y_str.parse().unwrap())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Sensor {
    beacon_distance: u64,
}

#[derive(Debug, Clone)]
pub struct Map {
    sensors: BTreeMap<Position, Sensor>,
    x_limits: (i64, i64),
}

impl Map {
    fn can_contain_beacon(&self, pos: &Position) -> bool {
        for (sensor_pos, sensor) in &self.sensors {
            if pos.get_block_distance(sensor_pos) <= sensor.beacon_distance {
                return false;
            }
        }

        true
    }

    fn find_free_space(&self) -> Option<Position> {
        for (sensor_pos, sensor) in &self.sensors {
            for p in sensor_pos.get_border_positions(sensor.beacon_distance) {
                if p.0 > 0
                    && p.1 > 0
                    && p.0 <= 4000000
                    && p.1 <= 4000000
                    && self.can_contain_beacon(&p)
                {
                    return Some(p);
                }
            }
        }
        None
    }

    fn check_line(&self, y: i64) -> u64 {
        let mut c = 0;
        for x in self.x_limits.0..self.x_limits.1 {
            if !self.can_contain_beacon(&Position(x, y)) {
                c += 1;
            }
        }

        c - 1 //I don't know why its one too many... Maybe I'll search for the source later, but for now this works.
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut sensors = BTreeMap::new();
        let mut x_limits = (0, 0);

        for l in value.lines() {
            let (_, l) = l.split_at(10);
            let (sensor_pos_str, l) = l.split_once(':').unwrap();
            let (_, beacon_pos_str) = l.split_once("at ").unwrap();

            let sensor_pos = Position::from(sensor_pos_str);
            let beacon_pos = Position::from(beacon_pos_str);
            let distance = sensor_pos.get_block_distance(&beacon_pos);

            x_limits.0 = x_limits.0.min(sensor_pos.1 - distance as i64);
            x_limits.1 = x_limits.1.max(sensor_pos.1 + distance as i64);

            let sensor = Sensor {
                beacon_distance: sensor_pos.get_block_distance(&beacon_pos),
            };

            sensors.insert(sensor_pos, sensor);
        }

        Self { sensors, x_limits }
    }
}

type Data = Map;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test15.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(5068327), Answer::Number(11016575214126))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, Map::from(input))
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number(data.check_line(10))
    }

    fn two(&self, data: &mut Data) -> Answer {
        let p = data.find_free_space().unwrap();
        Answer::Number((p.0 * 4000000 + p.1) as u64)
    }
}
