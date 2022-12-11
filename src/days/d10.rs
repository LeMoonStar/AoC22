use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 10;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Noop,
    AddX(i64),
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        let mut s = value.split_ascii_whitespace();
        match s.next().unwrap() {
            "noop" => Self::Noop,
            "addx" => Self::AddX(s.next().unwrap().parse().unwrap()),
            c => panic!("Unknown Command '{}'", c),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
    Ready,
    InProgress(Command, u8),
    End,
}

#[derive(Debug, Clone)]
pub struct Computer {
    program: Vec<Command>,
    reg_x: i64,
    counter: usize,
    state: State,
    position: usize,
}

impl Computer {
    fn new(program: Vec<Command>) -> Self {
        Self {
            program,
            reg_x: 1,
            // Thank you AOC, for counting from 1... this took me like 40 mins
            counter: 1,
            state: State::Ready,
            position: 0,
        }
    }

    fn execute_cycle(&mut self) {
        /*println!(
            "> C: {} \tS: {:?} \tX:{}",
            self.counter, self.state, self.reg_x
        );*/
        if let State::InProgress(c, t) = &mut self.state {
            match c {
                Command::AddX(v) => {
                    if *t == 1 {
                        self.reg_x += *v;
                        self.state = State::Ready;
                    } else {
                        *t += 1;
                    }
                }
                Command::Noop => {
                    self.state = State::Ready;
                }
            }
            self.counter += 1;
        } else {
            panic!("Unexpected execution")
        }

        /*println!(
            "< C: {} \tS: {:?} \tX:{}",
            self.counter, self.state, self.reg_x
        );>*/
    }

    fn tick(&mut self) {
        match &mut self.state {
            // If ready, fetch next command.
            State::Ready => {
                /*println!(
                    ">?C:{} \tS: {:?} (PC: {})",
                    self.counter, self.state, self.position
                );*/
                if self.position == self.program.len() {
                    self.state = State::End;
                    return;
                }
                let c = &self.program[self.position];
                self.state = State::InProgress(c.clone(), 0);
                self.position += 1;
                /*println!(
                    "<?C:{} \tS: {:?} (PC: {})",
                    self.counter, self.state, self.position
                );*/
                self.execute_cycle();
            }
            // If in Progress, execute cycle.
            State::InProgress(_, _) => {
                self.execute_cycle();
            }
            State::End => panic!("TRIED TO EXECUTE END OF PROGRAM!"),
        }
    }

    fn get_signal_strength(&mut self) -> i64 {
        let mut strength = 0;

        while self.state != State::End {
            self.tick();
            match self.counter {
                20 | 60 | 100 | 140 | 180 | 220 => {
                    /*println!(
                        "==C: {} \tS: {:?} \tX:{} \t=== {}",
                        self.counter,
                        self.state,
                        self.reg_x,
                        self.reg_x * self.counter as i64
                    );*/
                    strength += self.reg_x * self.counter as i64;
                }
                _ => {}
            }
        }

        strength
    }

    fn draw_crt(&mut self) -> String {
        let mut out = String::with_capacity(500);
        while self.state != State::End {
            let x = ((self.counter - 1) % 40) as i64;
            if x == 0 {
                out += "\n";
            }
            if self.reg_x > x - 2 && self.reg_x < x + 2 {
                out += "█";
            } else {
                out += " ";
            }
            self.tick();
        }
        out
    }
}

type Data = Computer;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test10.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(13140), Answer::String("\n██  ██  ██  ██  ██  ██  ██  ██  ██  ██  \n███   ███   ███   ███   ███   ███   ███ \n████    ████    ████    ████    ████    \n█████     █████     █████     █████     \n██████      ██████      ██████      ████\n███████       ███████       ███████     \n ".to_owned()))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            Computer::new(input.lines().map(Command::from).collect()),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number(data.get_signal_strength() as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::String(data.draw_crt())
    }
}
