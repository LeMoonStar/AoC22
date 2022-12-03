use super::{Answer, Day, DayImpl};
use std::collections::BTreeSet;

const CURRENT_DAY: u8 = 3;

fn get_character_priority(c: &char) -> u64 {
    (((*c as u8) - 65) % 31) as u64
        + match c.is_uppercase() {
            true => 27,
            false => 0,
        }
}

type Data = Vec<(BTreeSet<char>, BTreeSet<char>)>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test03.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(157), Answer::Number(70))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .lines()
                .map(|v| {
                    let a = v.split_at(v.len() / 2);
                    (a.0.chars().collect(), a.1.chars().collect())
                })
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut sum = 0;

        for s in data {
            sum += get_character_priority(s.0.intersection(&s.1).next().unwrap());
        }
        Answer::Number(sum)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut sum = 0;

        // And this next thing took way too long to code...
        // I was tempted to just throw my whole setup out the window...
        let mut data2: Vec<BTreeSet<char>> = Vec::new();
        for s in data.iter_mut() {
            let mut e: BTreeSet<char> = BTreeSet::new();

            e.append(&mut s.0);
            e.append(&mut s.1);

            data2.push(e);
        }

        for i in (0..data2.len()).step_by(3) {
            for c in &data2[i] {
                if data2[i + 1].contains(c) && data2[i + 2].contains(c) {
                    sum += get_character_priority(c);
                    break;
                }
            }
        }

        Answer::Number(sum)
    }
}
