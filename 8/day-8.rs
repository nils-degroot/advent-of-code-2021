use std::{fs, env, collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DisplaySides {
    Top,
    TopLeft,
    TopRight,
    Center,
    BottomLeft,
    BottomRight,
    Bottom
}

impl DisplaySides {
    pub fn all() -> Vec<DisplaySides> {
        vec![DisplaySides::Top, DisplaySides::TopLeft, DisplaySides::TopRight, DisplaySides::Center, DisplaySides::BottomLeft, DisplaySides::BottomRight, DisplaySides::Bottom]
    }
}

#[derive(Debug)]
struct BrokenDisplay {
    input: Vec<String>,
    output: Vec<String>,
    decoded: HashMap<DisplaySides, Vec<char>>,
}

impl BrokenDisplay {
    const VALID_DIGITS: &'static str = "abcdefg";

    pub fn new<S: ToString>(input: Vec<S>, output: Vec<S>) -> Self {
        let mut input = input.iter().map(|i| i.to_string()).collect::<Vec<_>>();
        let output = output.iter().map(|i| i.to_string()).collect::<Vec<_>>();

        input.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());

        let mut decoded = HashMap::new();

        for side in DisplaySides::all() {
            decoded.insert(side, Self::VALID_DIGITS.chars().collect::<Vec<_>>());
        }

        Self { input, output, decoded }
    }

    pub fn number_contains(number: char, side: DisplaySides) -> bool {
        match number {
            '0' => !matches!(side, DisplaySides::Center),
            '1' => matches!(side, DisplaySides::TopRight | DisplaySides::BottomRight),
            '2' => !matches!(side, DisplaySides::TopLeft | DisplaySides::BottomRight),
            '3' => !matches!(side, DisplaySides::TopLeft | DisplaySides::BottomLeft),
            '4' => !matches!(side, DisplaySides::Top | DisplaySides::BottomLeft | DisplaySides::Bottom),
            '5' => !matches!(side, DisplaySides::TopRight | DisplaySides::BottomLeft),
            '6' => !matches!(side, DisplaySides::TopRight),
            '7' => matches!(side, DisplaySides::TopRight | DisplaySides::BottomRight | DisplaySides::Top),
            '8' => true, // 8 contains all sides
            '9' => !matches!(side, DisplaySides::BottomLeft),
            _ => panic!("Not a valid numeric digit")
        }
    }

    pub fn part1(&self) -> usize {
        let legal = vec![2, 3, 4, 7];

        self.output.iter()
            .filter(|r| legal.contains(&r.len()))
            .collect::<Vec<_>>()
            .len()
    }

    pub fn decode(&mut self) {
        if let Some(v) = self.input.iter().find(|r| r.len() == 2) {
            let v_collected = v.chars().collect::<Vec<char>>();

            for (side, chars) in self.decoded.iter_mut() {
                *chars = chars.iter()
                    .filter(|c| if Self::number_contains('1', side.clone()) { 
                        v_collected.contains(c) 
                    } else { 
                        !v_collected.contains(c) 
                    }).map(|c| c.clone()).collect::<Vec<_>>();
            }
        }

        if let Some(v) = self.input.iter().find(|r| r.len() == 3) {
            let v_collected = v.chars().collect::<Vec<char>>();

            for (side, chars) in self.decoded.iter_mut() {
                *chars = chars.iter()
                    .filter(|c| if Self::number_contains('7', side.clone()) { 
                        v_collected.contains(c) 
                    } else { 
                        !v_collected.contains(c) 
                    }).map(|c| c.clone()).collect::<Vec<_>>();
            }
        }

        if let Some(v) = self.input.iter().find(|r| r.len() == 4) {
            let v_collected = v.chars().collect::<Vec<char>>();

            for (side, chars) in self.decoded.iter_mut() {
                *chars = chars.iter()
                    .filter(|c| if Self::number_contains('4', side.clone()) { 
                        v_collected.contains(c) 
                    } else { 
                        !v_collected.contains(c) 
                    }).map(|c| c.clone()).collect::<Vec<_>>();
            }
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let file = fs::read_to_string(args[1].clone()).unwrap();

    let mut inputs = vec![];
    let mut outputs = vec![];

    for row in file.lines() {
        let sides = row.split('|').collect::<Vec<_>>();
        inputs.push(sides[0].split_whitespace().collect::<Vec<_>>());
        outputs.push(sides[1].split_whitespace().collect::<Vec<_>>());
    }

    let displays = inputs.iter().zip(outputs)
        .map(|(i, o)| BrokenDisplay::new(i.clone(), o.clone()))
        .collect::<Vec<_>>();

    let part1 = displays.iter().fold(0, |a, r| a + r.part1());
    println!("{}", part1);

    for mut d in displays {
        d.decode();
        println!("{:#?}", d);
    }
}
