use std::{env, collections::HashMap};

struct SpeedSchool (HashMap<usize, usize>);

impl SpeedSchool {
    pub fn new(input: Vec<usize>) -> Self {
        let mut acc = HashMap::new();

        // Initialize accumulator
        for i in 0..=8 { acc.insert(i, 0); }
        // Push all fish
        for fish in input { *acc.get_mut(&fish).unwrap() += 1; }

        Self(acc)
    }

    pub fn pass_day(&mut self) {
        let mut new_acc = HashMap::new();
        for i in 0..=8 { new_acc.insert(i, 0); }

        for (key, count) in &self.0 {
            if key == &0 {
                *new_acc.get_mut(&6).unwrap() += *count;
                *new_acc.get_mut(&8).unwrap() += *count;
            } else {
                *new_acc.get_mut(&(key - 1)).unwrap() += count;
            }
        }

        self.0 = new_acc;
    }

    pub fn pass_n_days(&mut self, days: usize) {
        for i in 1..=days { 
            println!("Passed day {}", i);
            self.pass_day();
        }
    }

    pub fn len(&self) -> usize {
        self.0.iter().fold(0, |a, (_, c)| a + c)
    }
}

fn main() {
    let args = env::args()
        .collect::<Vec<_>>()
        .get(1).unwrap()
        .split(',').into_iter()
        .map(|r| r.parse().unwrap())
        .collect::<Vec<usize>>();

    let mut school = SpeedSchool::new(args);
    school.pass_n_days(256);
    println!("Got {} fish", school.len());
}
