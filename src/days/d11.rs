use std::str::Lines;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 11;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Item {
    worry_level: u64,
}

impl From<&str> for Item {
    fn from(value: &str) -> Self {
        Self {
            worry_level: value.parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    Square,
    Add(u64),
    Multiply(u64),
}

impl Operation {
    fn execute(&self, value: u64) -> u64 {
        match self {
            Self::Square => value * value,
            Self::Add(x) => value + x,
            Self::Multiply(x) => value * x,
        }
    }
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        if value.starts_with("new = old * old") {
            Self::Square
        } else if value.starts_with("new = old *") {
            Self::Multiply(value.split_once("* ").unwrap().1.parse().unwrap())
        } else if value.starts_with("new = old +") {
            Self::Add(value.split_once("+ ").unwrap().1.parse().unwrap())
        } else {
            unreachable!("Unexpected input for Operation")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Decision {
    divisible_by: u64,
    true_monkey: usize,
    false_monkey: usize,
}

impl Decision {
    fn decide(&self, item: &Item) -> usize {
        if item.worry_level % self.divisible_by == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        }
    }
}

impl Decision {
    fn from(lines: &mut Lines) -> Self {
        Self {
            divisible_by: lines
                .next()
                .unwrap()
                .split_once("by ")
                .unwrap()
                .1
                .parse()
                .unwrap(),
            true_monkey: lines
                .next()
                .unwrap()
                .split_once("ey ")
                .unwrap()
                .1
                .parse()
                .unwrap(),
            false_monkey: lines
                .next()
                .unwrap()
                .split_once("ey ")
                .unwrap()
                .1
                .parse()
                .unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
struct Throw {
    item: Item,
    destination: usize,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    decision: Decision,
    inspection_count: u64,
}

impl Monkey {
    fn inspect_items(&mut self, relieve_enabled: bool) {
        for i in &mut self.items {
            self.inspection_count += 1;
            i.worry_level = self.operation.execute(i.worry_level);
            if relieve_enabled {
                i.worry_level /= 3;
            }
        }
    }

    fn take_turn(&mut self, relieve_enabled: bool) -> Vec<Throw> {
        let mut throws = Vec::new();
        self.inspect_items(relieve_enabled);

        for i in &self.items {
            throws.push(Throw {
                destination: self.decision.decide(i),
                item: i.clone(),
            })
        }
        self.items.clear();

        throws
    }
}

impl From<&str> for Monkey {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        if !lines.next().unwrap().starts_with("Monkey") {
            panic!("Unexpected input.");
        }

        // Parse items
        let items = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(Item::from)
            .collect();

        let operation = Operation::from(lines.next().unwrap().split_once(": ").unwrap().1);

        Self {
            items,
            operation,
            decision: Decision::from(&mut lines),
            inspection_count: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MonkeyTroop {
    monkeys: Vec<Monkey>,
    common: u64,
}

impl MonkeyTroop {
    fn round(&mut self, relieve_enabled: bool) {
        // No, Gewi, I cant use a `for m in self.monkeys` here, I'd double borrow the Vec.
        // And this is the simplest solution that I know of. If there is a better solution, please let me know.
        for i in 0..self.monkeys.len() {
            let throws = self.monkeys[i].take_turn(relieve_enabled);
            for mut t in throws {
                t.item.worry_level %= self.common;
                self.monkeys[t.destination].items.push(t.item);
            }
        }
    }
}

impl From<&str> for MonkeyTroop {
    fn from(value: &str) -> Self {
        let monkeys: Vec<Monkey> = value.split("\n\n").map(Monkey::from).collect();
        Self {
            common: monkeys.iter().fold(1, |a, m| a * m.decision.divisible_by),
            monkeys,
        }
    }
}

type Data = MonkeyTroop;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test11.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(10605), Answer::Number(2713310158))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, MonkeyTroop::from(input))
    }

    fn one(&self, data: &mut Data) -> Answer {
        //println!("{:?}", data);

        for _ in 0..20 {
            data.round(true);
        }

        let mut counts: Vec<u64> = data.monkeys.iter().map(|v| v.inspection_count).collect();
        counts.sort_by(|a, b| b.cmp(a));

        Answer::Number(counts[0] * counts[1])
    }

    fn two(&self, data: &mut Data) -> Answer {
        for _ in 0..10000 {
            data.round(false);
        }

        let mut counts: Vec<u64> = data.monkeys.iter().map(|v| v.inspection_count).collect();
        counts.sort_by(|a, b| b.cmp(a));

        Answer::Number(counts[0] * counts[1])
    }
}
