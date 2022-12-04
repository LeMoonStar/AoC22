use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 4;

#[derive(Debug, Clone)]
pub struct Range(u64, u64);

impl Range {
    fn contains(&self, other: &Self) -> bool {
        self.0 <= other.0 && other.1 <= self.1
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.0 <= other.1 && other.0 <= self.1
    }
}

impl From<&str> for Range {
    fn from(v: &str) -> Self {
        let mut s = v.split('-');
        Self(
            s.next().unwrap().parse().unwrap(),
            s.next().unwrap().parse().unwrap(),
        )
    }
}

type Data = Vec<Vec<Range>>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test04.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(2), Answer::Number(4))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .lines()
                .map(|v| v.split(',').map(Range::from).collect())
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut c = 0;
        for d in data {
            if d[0].contains(&d[1]) || d[1].contains(&d[0]) {
                c += 1;
            }
        }
        Answer::Number(c)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut c = 0;
        for d in data {
            if d[0].overlaps(&d[1]) {
                c += 1;
            }
        }
        Answer::Number(c)
    }
}
