use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 1;

type Data = Vec<Vec<u64>>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test01.txt").to_owned())
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(24000), Answer::Number(45000))
    }

    fn init(input: &str) -> (Self, Data) {
        let mut o: Vec<Vec<u64>> = Vec::new();

        input.split("\n\n").for_each(|v| {
            let v: Vec<u64> = v.lines()
                .map(|v| v.parse::<u64>().expect("error while parsing input."))
                .collect();
                o.push(v);
        });

        (
            Self {},
            o
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let n: u64 = data.iter().map(|v| v.iter().sum()).max().unwrap();
        Answer::Number(n as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut d: Vec<u64> = data.iter().map(|v| v.iter().sum::<u64>()).collect();
        d.sort_by(|a, b| b.cmp(a));
        Answer::Number(d[0]+d[1]+d[2] as u64)
    }
}
