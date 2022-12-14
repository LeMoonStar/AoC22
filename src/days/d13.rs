use std::{cmp::Ordering, str::Chars};

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 13;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ComparisonResult {
    Continue,
    Correct,
    Wrong,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Item {
    Value(u8),
    List(Vec<Item>),
}

impl Item {
    fn compare(&self, other: &Self) -> ComparisonResult {
        match self {
            Self::List(s) => match other {
                Self::List(o) => {
                    for i in 0..o.len().max(s.len()) {
                        if i == s.len() {
                            return ComparisonResult::Correct;
                        }
                        if i == o.len() {
                            return ComparisonResult::Wrong;
                        }

                        match s[i].compare(&o[i]) {
                            ComparisonResult::Correct => return ComparisonResult::Correct,
                            ComparisonResult::Continue => continue,
                            ComparisonResult::Wrong => return ComparisonResult::Wrong,
                        }
                    }

                    ComparisonResult::Continue
                }
                Self::Value(_) => self.compare(&Self::List(vec![other.clone()])),
            },
            Self::Value(s) => match other {
                Self::Value(o) => match s.cmp(o) {
                    Ordering::Equal => ComparisonResult::Continue,
                    Ordering::Less => ComparisonResult::Correct,
                    _ => ComparisonResult::Wrong,
                },
                Self::List(_) => Self::List(vec![self.clone()]).compare(other),
            },
        }
    }

    // Disgusting parser, please don't blame, at this point some other stuff came in the way and I don't have the time or motivation to do stuff nicely.
    fn parse_list(chars: &mut Chars) -> Self {
        let mut items = Vec::new();

        while let Some(c) = chars.next() {
            match c {
                '[' => items.push(Self::parse_list(chars)),
                ']' => break,
                ',' => {}
                c => {
                    if c.is_numeric() {
                        let (value, exit) = Self::parse_value(String::from(c), chars);
                        items.push(value);
                        if exit {
                            break;
                        }
                    } else {
                        panic!("unexpected input");
                    }
                }
            }
        }

        Self::List(items)
    }

    fn parse_value(mut s: String, chars: &mut Chars) -> (Self, bool) {
        for c in chars.by_ref() {
            if c.is_numeric() {
                s.push(c)
            } else {
                return (Self::Value(s.parse().unwrap()), c == ']');
            }
        }

        unreachable!("Unexpected input.")
    }
}

impl From<&str> for Item {
    fn from(value: &str) -> Self {
        Self::parse_list(&mut value.chars())
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.compare(other) {
            ComparisonResult::Correct => Some(Ordering::Less),
            ComparisonResult::Wrong => Some(Ordering::Greater),
            ComparisonResult::Continue => None, //Yes, this will fail when used with `Ord`, but it should never be the case and I don't care rn
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct Pair(Item, Item);

impl Pair {
    fn check_order(&self) -> bool {
        self.0.compare(&self.1) == ComparisonResult::Correct
    }
}

impl From<&str> for Pair {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        Self(
            Item::from(lines.next().unwrap()),
            Item::from(lines.next().unwrap()),
        )
    }
}

type Data = Vec<Pair>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test13.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(13), Answer::Number(140))
    }

    fn init(input: &str) -> (Self, Data) {
        (Self {}, input.split("\n\n").map(Pair::from).collect())
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut out = 0;

        for (i, p) in data.iter().enumerate() {
            if p.check_order() {
                out += i + 1;
            }
        }

        Answer::Number(out as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let a_package = Item::List(vec![Item::List(vec![Item::Value(2)])]);
        let b_package = Item::List(vec![Item::List(vec![Item::Value(6)])]);

        let mut l: Vec<&Item> = Vec::with_capacity(data.len() * 2 + 2);

        data.iter().for_each(|v| {
            l.push(&v.0);
            l.push(&v.1)
        });

        l.push(&a_package);
        l.push(&b_package);

        l.sort();

        let mut a = 0;
        let mut b = 0;

        for (i, k) in l.iter().enumerate() {
            if **k == a_package {
                a = i + 1;
            }
            if **k == b_package {
                b = i + 1;
            }
        }

        Answer::Number((a * b) as u64)
    }
}
