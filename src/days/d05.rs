use super::{Answer, Day, DayImpl};
use std::collections::{BTreeMap, VecDeque};

const CURRENT_DAY: u8 = 5;

type Stack = BTreeMap<u8, VecDeque<char>>;

#[derive(Debug, Clone)]
pub struct Command {
    num: usize,
    from: u8,
    to: u8,
}

impl Command {
    fn run9000(&self, stacks: &mut Stack) {
        for _ in 0..self.num {
            let v = stacks.get_mut(&self.from).unwrap().pop_front().unwrap();
            stacks.get_mut(&self.to).unwrap().push_front(v);
        }
    }

    fn run9001(&self, stacks: &mut Stack) {
        let mut tmp = Vec::new();

        for _ in 0..self.num {
            let v = stacks.get_mut(&self.from).unwrap().pop_front().unwrap();
            tmp.push(v);
        }

        tmp.reverse();

        for v in tmp {
            stacks.get_mut(&self.to).unwrap().push_front(v);
        }
    }
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        let mut parts = s.split(' ');
        Self {
            num: parts.nth(1).unwrap().parse().unwrap(),
            from: parts.nth(1).unwrap().parse().unwrap(),
            to: parts.nth(1).unwrap().parse().unwrap(),
        }
    }
}

type Data = (Stack, Vec<Command>);

impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test05.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (
            Answer::String("CMZ".to_owned()),
            Answer::String("MCD".to_owned()),
        )
    }

    fn init(input: &str) -> (Self, Data) {
        let mut stacks: Stack = Stack::new();

        let mut s_input = input.split("\n\n");

        // --- PARSE STACKS ---

        let mut stacks_input = s_input.next().unwrap().lines();
        let height = stacks_input.clone().count() - 1;

        let mut i: usize = 0;

        for mut l in stacks_input {
            if i == height {
                break;
            }
            let mut run = true;

            let mut n: u8 = 1;
            while run {
                let p = match l.len() {
                    3 => {
                        run = false;
                        l
                    }
                    _ => {
                        let (p, n_l) = l.split_at(4);
                        l = n_l;
                        &p[..3]
                    }
                };

                let c = p.chars().nth(1).unwrap();
                if c != ' ' {
                    let mut v = stacks.entry(n).or_insert(VecDeque::new());
                    v.push_back(c);
                }
                n += 1;
            }

            i += 1;
        }

        // --- PARSE INPUTS ---

        let mut commands = Vec::new();

        for l in s_input.next().unwrap().lines() {
            commands.push(Command::from(l));
        }

        (Self {}, (stacks, commands))
    }

    fn one(&self, data: &mut Data) -> Answer {
        for cmd in &data.1 {
            cmd.run9000(&mut data.0);
        }

        let mut output: String = String::new();

        for s in &data.0 {
            output.push(*s.1.front().unwrap());
        }

        Answer::String(output)
    }

    fn two(&self, data: &mut Data) -> Answer {
        for cmd in &data.1 {
            cmd.run9001(&mut data.0);
        }

        let mut output: String = String::new();

        for s in &data.0 {
            output.push(*s.1.front().unwrap());
        }

        Answer::String(output)
    }
}
