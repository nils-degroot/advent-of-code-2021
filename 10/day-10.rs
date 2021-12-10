use std::{fs, env, collections::HashMap};

#[derive(Debug)]
struct SimpleStack<T> {
    inner: Vec<T>
}

impl<T> SimpleStack<T> {
    pub fn new() -> Self { Self { inner: vec![] } }

    pub fn push(&mut self, item: T) { self.inner.push(item); }

    pub fn pop(&mut self) -> Option<T> { self.inner.pop() }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum LegalCharacters {
    LeftBracket, // (
    LeftSquare, // [
    LeftCurly, // {
    LessThen, // <
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let file = fs::read_to_string(args[1].clone()).unwrap();

    // Part 1
    let mut score1 = 0;
    let mut score1_lookup = HashMap::new();
    score1_lookup.insert(LegalCharacters::LeftBracket, 3);
    score1_lookup.insert(LegalCharacters::LeftSquare, 57);
    score1_lookup.insert(LegalCharacters::LeftCurly, 1197);
    score1_lookup.insert(LegalCharacters::LessThen, 25137);

    for line in file.lines() {
        let mut stack = SimpleStack::<LegalCharacters>::new();

        for c in line.chars() {
            match c {
                '(' => stack.push(LegalCharacters::LeftBracket),
                '[' => stack.push(LegalCharacters::LeftSquare),
                '{' => stack.push(LegalCharacters::LeftCurly),
                '<' => stack.push(LegalCharacters::LessThen),
                _ => { // Danger zone
                    let parsed = match c { // Map all to the matching character
                        ')' => LegalCharacters::LeftBracket,
                        ']' => LegalCharacters::LeftSquare,
                        '}' => LegalCharacters::LeftCurly,
                        '>' => LegalCharacters::LessThen,
                        _ => panic!("Invalid character")
                    };

                    if Some(parsed) != stack.pop() {
                        score1 += score1_lookup.get(&parsed).unwrap();
                        break;
                    }
                }
            }
        }
    }

    println!("{}", score1);

    // Part 2
    let mut score2 = vec![];
    let mut score2_lookup = HashMap::new();
    score2_lookup.insert(LegalCharacters::LeftBracket, 1);
    score2_lookup.insert(LegalCharacters::LeftSquare, 2);
    score2_lookup.insert(LegalCharacters::LeftCurly, 3);
    score2_lookup.insert(LegalCharacters::LessThen, 4);

    for line in file.lines().rev() {
        let mut corrupted = false;
        let mut stack = SimpleStack::<LegalCharacters>::new();

        for c in line.chars() {
            match c {
                '(' => stack.push(LegalCharacters::LeftBracket),
                '[' => stack.push(LegalCharacters::LeftSquare),
                '{' => stack.push(LegalCharacters::LeftCurly),
                '<' => stack.push(LegalCharacters::LessThen),
                _ => { // Danger zone
                    let parsed = match c { // Map all to the matching character
                        ')' => LegalCharacters::LeftBracket,
                        ']' => LegalCharacters::LeftSquare,
                        '}' => LegalCharacters::LeftCurly,
                        '>' => LegalCharacters::LessThen,
                        _ => panic!("Invalid character")
                    };

                    if Some(parsed) != stack.pop() {
                        corrupted = true;
                        break; // Corrupted stack
                    }
                }
            }
        }

        if !corrupted {
            let mut inner_score: usize = 0;
            while let Some(item) = stack.pop() {
                inner_score = inner_score * 5 + score2_lookup.get(&item).unwrap();
            }
            score2.push(inner_score);
        }
    }

    score2.sort();
    println!("{}", score2[score2.len() / 2]);
}
