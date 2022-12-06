use super::{Answer, Day, DayImpl};
use std::collections::VecDeque;

const CURRENT_DAY: u8 = 6;

fn contains_duplicate(queue: &VecDeque<char>) -> bool {
    let mut i = 0;
    for v in queue {
        if queue.iter().filter(|x| x.eq(&v)).count() > 1 {
            return true;
        }
        i += 1;
    }

    false
}

type Data = String;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test06.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(11), Answer::Number(26))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.to_owned())
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut chars = data.chars();
        let mut queue: VecDeque<char> = VecDeque::new();
        let mut i = 1;

        loop {
            queue.push_back(chars.next().unwrap());
            if queue.len() > 4 {
                queue.pop_front();
                if !contains_duplicate(&queue) {
                    return Answer::Number(i);
                }
            }
            i += 1;
        }
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut chars = data.chars();
        let mut queue: VecDeque<char> = VecDeque::new();
        let mut i = 1;

        loop {
            queue.push_back(chars.next().unwrap());
            if queue.len() > 14 {
                queue.pop_front();
                if !contains_duplicate(&queue) {
                    return Answer::Number(i);
                }
            }
            i += 1;
        }
    }
}
