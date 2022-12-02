use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameResult {
    Win,
    Draw,
    Loose,
}

impl GameResult {
    fn score(&self) -> u64 {
        match self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Loose => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Hand {
    StoneOrLoose = 1,
    PaperOrDraw = 2,
    ScissorsOrWin = 3,
}

impl Hand {
    /// Get the score of a hand
    fn score(&self) -> u64 {
        *self as u64
    }

    fn result_score(&self) -> u64 {
        match self {
            Hand::StoneOrLoose => 0,
            Hand::PaperOrDraw => 3,
            Hand::ScissorsOrWin => 6,
        }
    }

    /// Check the result of a game.
    fn play(&self, other: &Self) -> GameResult {
        match (self, other) {
            (Hand::StoneOrLoose, Hand::ScissorsOrWin) => GameResult::Win,
            (Hand::PaperOrDraw, Hand::StoneOrLoose) => GameResult::Win,
            (Hand::ScissorsOrWin, Hand::PaperOrDraw) => GameResult::Win,

            (Hand::StoneOrLoose, Hand::StoneOrLoose) => GameResult::Draw,
            (Hand::PaperOrDraw, Hand::PaperOrDraw) => GameResult::Draw,
            (Hand::ScissorsOrWin, Hand::ScissorsOrWin) => GameResult::Draw,

            _ => GameResult::Loose,
        }
    }

    /// Get the hand that would win or loose against this hand
    fn get_opponent(&self, opponent_win: bool) -> Hand {
        match opponent_win {
            true => match self {
                Hand::StoneOrLoose => Hand::PaperOrDraw,
                Hand::PaperOrDraw => Hand::ScissorsOrWin,
                Hand::ScissorsOrWin => Hand::StoneOrLoose,
            },
            false => match self {
                Hand::StoneOrLoose => Hand::ScissorsOrWin,
                Hand::PaperOrDraw => Hand::StoneOrLoose,
                Hand::ScissorsOrWin => Hand::PaperOrDraw,
            },
        }
    }

    fn get_part2_matching(&self, other: &Self) -> Hand {
        match self {
            Hand::PaperOrDraw => *other,
            Hand::ScissorsOrWin => other.get_opponent(true),
            Hand::StoneOrLoose => other.get_opponent(false),
        }
    }
}

impl From<&str> for Hand {
    fn from<'a>(s: &str) -> Self {
        match s {
            "A" => Hand::StoneOrLoose,
            "B" => Hand::PaperOrDraw,
            "C" => Hand::ScissorsOrWin,
            "X" => Hand::StoneOrLoose,
            "Y" => Hand::PaperOrDraw,
            "Z" => Hand::ScissorsOrWin,
            _ => panic!("Invalid input while parsing"),
        }
    }
}

type Data = Vec<Vec<Hand>>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test02.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(15), Answer::Number(12))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .lines()
                .map(|v| v.split_whitespace().map(Hand::from).collect())
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut score = 0;
        for g in data {
            score += g[1].play(&g[0]).score() + g[1].score();
        }
        Answer::Number(score)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut score = 0;
        for g in data {
            score += g[1].get_part2_matching(&g[0]).score() + g[1].result_score();
        }
        Answer::Number(score)
    }
}
