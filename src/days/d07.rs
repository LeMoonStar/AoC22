use super::{Answer, Day, DayImpl};
use std::collections::BTreeMap;

// For your own sanity, don't read this... This is absolute bullshit
// Trying to write this shit quickly resulted in a mess of horrible code,
// especially since I misinterpreted the task MULTIPLE times.
// Additionally I had to leave for unrelated stuff multiple times too, which doesn't improve anything.
// I should probably rewrite all of this, however, I lack motivation to do so.
// Feel free to call me dumb

// This is a gigantic mess and I hate myself for it.

const CURRENT_DAY: u8 = 7;

#[derive(Clone, Debug)]
pub struct File {
    is_dir: bool,
    size: Option<u64>,
    children: BTreeMap<String, File>,
    parent: Option<*mut File>,
}

impl File {
    fn new(is_dir: bool, size: Option<u64>) -> Self {
        Self {
            is_dir,
            size,
            children: BTreeMap::new(),
            parent: None,
        }
    }

    // My first try, while not reading the task correctly. Gonna kep it in  case I need it for part 2
    fn get_size_min_max(&self, min: u64, max: u64) -> u64 {
        if self.is_dir {
            self.children
                .values()
                .map(|v| v.get_size_min_max(min, max))
                .filter(|v| v >= &min && v <= &max)
                .sum()
        } else if self.size.unwrap() >= min && self.size.unwrap() <= max {
            self.size.unwrap()
        } else {
            0
        }
    }

    // Second attempt, still haven't read it correctly I guess
    fn get_size(&self) -> u64 {
        if self.is_dir {
            self.children.values().map(|v| v.get_size()).sum()
        } else {
            self.size.unwrap()
        }
    }

    // Guess I need this, however, shit still doesn't fucking work.
    // AND NO, GUESS WHAT, GET_SIZE IS CORRECT; BUT I GOTTA MAKE IT STACKED
    // GOOOD I CANT WAIT FOR PART 2, LOVELY
    fn get_size_files_only(&self) -> u64 {
        self.children
            .values()
            .filter(|v| !v.is_dir)
            .map(|v| v.size.unwrap())
            .sum()
    }

    fn insert_child(&mut self, name: &str, mut child: File) -> &mut File {
        child.parent = Some(self as *mut File);
        self.children.insert(name.to_owned(), child);
        self.children.get_mut(name).unwrap()
    }

    // fuck this function
    fn task_one(&self) -> u64 {
        let mut sum = 0;

        let s = self.get_size();
        if s <= 100000 {
            sum += s;
        }

        for c in self.children.values() {
            if c.is_dir {
                sum += c.task_one();
            }
        }

        sum
    }

    fn task_two(&self, required_space: u64) -> Vec<u64> {
        let mut v = Vec::new();

        let s = self.get_size();
        if s >= required_space {
            v.push(s)
        }

        for c in self.children.values() {
            if c.is_dir {
                v.append(&mut c.task_two(required_space));
            }
        }

        v
    }
}

type Data = File;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test07.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(95437), Answer::Number(24933642))
    }

    fn init(input: &str) -> (Self, Data) {
        let mut root: File = File::new(true, None);
        let mut current_file = &mut root;

        for mut l in input.lines().map(|v| v.split_ascii_whitespace()) {
            let a = l.next().unwrap();
            if a.starts_with('$') {
                match l.next().unwrap() {
                    "cd" => match l.next().unwrap() {
                        "/" => current_file = &mut root,
                        ".." => current_file = unsafe { &mut *current_file.parent.unwrap() },
                        name => {
                            let f = File::new(true, None);
                            current_file = current_file.insert_child(name, f);
                        }
                    },
                    "ls" => {}
                    _ => panic!("unexpected command"),
                }
            } else {
                // This can only be ls output.
                if let Ok(size) = a.parse::<u64>() {
                    let f = File::new(false, Some(size));
                    current_file.insert_child(l.next().unwrap(), f);
                }
            }
        }

        (Self {}, root)
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number(data.task_one())
    }

    fn two(&self, data: &mut Data) -> Answer {
        let max_space = 40000000;
        let space_to_clean = data.get_size() - max_space;

        let mut res = data.task_two(space_to_clean);
        res.sort();

        Answer::Number(*res.first().unwrap())
    }
}
