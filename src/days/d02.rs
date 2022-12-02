use super::{Answer, Day, DayImpl};
use crate::dprintln;

const CURRENT_DAY: u8 = 2;

fn get_hand_score(g: &String) -> u64 {
    if g == "X" {
        return 1;
    }
    if g == "Y" {
        return 2;
    }
    if g == "Z" {
        return 3;
    }
    return 0;
}

// A X: Rock     1
// B Y: Paper    2
// C Z: Scissors 3
fn get_hand_score_p2(opponent: &String, own: &String) -> u64 {
    if own == "X" {
        if opponent == "A" {
            return 3;
        }
        if opponent == "B" {
            return 1;
        }
        if opponent == "C" {
            return 2;
        }
    }
    if own == "Y" {
        if opponent == "A" {
            return 1;
        }
        if opponent == "B" {
            return 2;
        }
        if opponent == "C" {
            return 3;
        }
    }
    if own == "Z" {
        if opponent == "A" {
            return 2;
        }
        if opponent == "B" {
            return 3;
        }
        if opponent == "C" {
            return 1;
        }
    }
    return 0;
}

type Data = Vec<Vec<String>>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test02.txt").to_owned())
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(15), Answer::Number(12))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .lines()
                .map(|v| v.split_whitespace().map(|v| v.to_owned()).collect())
                .collect(),
        )
    }

    // I know this is ugly and you could do it with some numbers and offsets. I'll do that later.
    // Wanna finish it quickly rn and am sleepy.
    fn one(&self, data: &mut Data) -> Answer {
        let mut score = 0;
        for g in data {
            score += get_hand_score(&g[1]);
            dprintln!("{:?}", g);
            if g[0] == "A" && g[1] == "X"
                || g[0] == "B" && g[1] == "Y"
                || g[0] == "C" && g[1] == "Z"
            {
                dprintln!("draw!");
                score += 3;
            } else if g[0] == "A" && g[1] == "Y"
                || g[0] == "B" && g[1] == "Z"
                || g[0] == "C" && g[1] == "X"
            {
                dprintln!("win!");
                score += 6;
            }
        }
        Answer::Number(score)
    }

    // Same here again, there are way nicer solutions, and I'll fix this one up later.
    fn two(&self, data: &mut Data) -> Answer {
        let mut score = 0;
        for g in data {
            score += get_hand_score_p2(&g[0], &g[1]);
            if g[1] == "X" {
                score += 0;
            } else if g[1] == "Y" {
                score += 3;
            } else {
                score += 6;
            }
        }
        Answer::Number(score)
    }
}
