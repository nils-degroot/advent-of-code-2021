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
    all_decoded: Option<HashMap<char, DisplaySides>>
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

        Self { input, output, decoded, all_decoded: None }
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
        let mut safe_values = HashMap::new();
        safe_values.insert(2, '1');
        safe_values.insert(3, '7');
        safe_values.insert(4, '4');

        for (len, num) in safe_values {
            // We can safely find the entries of 1, 4 and 7 due to the length
            let val = self.input.iter().find(|r| r.len() == len).unwrap();
            let v_collected = val.chars().collect::<Vec<char>>();

            for (side, chars) in self.decoded.iter_mut() {
                *chars = chars.iter()
                    .filter(|c| if Self::number_contains(num, side.clone()) { 
                        v_collected.contains(c)
                    } else { 
                        !v_collected.contains(c)
                    }).map(|c| c.clone()).collect::<Vec<_>>();
            }
        }

        // Filter out the safe values
        self.input = self.input.iter()
            .filter(|r| !vec![2, 3, 4, 8].contains(&r.len()))
            .map(|r| r.clone())
            .collect::<Vec<String>>();

        // 6 is the only value with 6 sides which does not contain top right
        let decode_cpy = self.decoded.clone();
        
        let tr = decode_cpy.get(&DisplaySides::TopRight).unwrap();
        let six = self.input.iter()
            .filter(|r| r.len() == 6)
            .map(|r| r.clone().chars().collect::<Vec<_>>())
            .filter(|r| {
                tr.iter()
                    .filter(|tr_c| r.contains(tr_c))
                    .collect::<Vec<_>>()
                    .len() == 1
            }).collect::<Vec<_>>();
        let six_chars = six.first().unwrap();

        let br = tr.iter().find(|c| six_chars.contains(c)).unwrap();
        let tr = tr.iter().find(|c| c.clone() != br).unwrap();

        *self.decoded.get_mut(&DisplaySides::BottomRight).unwrap() = vec![*br];
        *self.decoded.get_mut(&DisplaySides::TopRight).unwrap() = vec![tr.clone()];

        // Next we can find 9 for the same reason but bottom left
        let bl = decode_cpy.get(&DisplaySides::BottomLeft).unwrap();
        let nine = self.input.iter()
            .filter(|r| r.len() == 6)
            .map(|r| r.clone().chars().collect::<Vec<_>>())
            .filter(|r| {
                bl.iter()
                    .filter(|bl_c| r.contains(bl_c))
                    .collect::<Vec<_>>()
                    .len() == 1
            }).collect::<Vec<_>>();
        let nine_chars = nine.first().unwrap();

        let btm = bl.iter().find(|c| nine_chars.contains(c)).unwrap();
        let bl = bl.iter().find(|c| c.clone() != btm).unwrap();

        *self.decoded.get_mut(&DisplaySides::Bottom).unwrap() = vec![*btm];
        *self.decoded.get_mut(&DisplaySides::BottomLeft).unwrap() = vec![bl.clone()];

        // Center is missing in 0. Last filter
        let center = decode_cpy.get(&DisplaySides::Center).unwrap();
        let zero = self.input.iter()
            .filter(|r| r.len() == 6)
            .map(|r| r.clone().chars().collect::<Vec<_>>())
            .filter(|r| {
                center.iter()
                    .filter(|c_c| r.contains(c_c))
                    .collect::<Vec<_>>()
                    .len() == 1
            }).collect::<Vec<_>>();
        let zero = zero.first().unwrap();

        let center = decode_cpy.get(&DisplaySides::Center).unwrap().clone();
        let center = center.iter()
            .filter(|c| !zero.contains(c)).map(|c| c.clone())
            .collect::<Vec<_>>();
        *self.decoded.get_mut(&DisplaySides::Center).unwrap() = center;

        let tl = decode_cpy.get(&DisplaySides::TopLeft).unwrap().clone();
        let tl = tl.iter()
            .filter(|c| zero.contains(c)).map(|c| c.clone())
            .collect::<Vec<_>>();
        *self.decoded.get_mut(&DisplaySides::TopLeft).unwrap() = tl;

        // All filtered
        let mut acc = HashMap::new();
        for (side, ch) in self.decoded.clone() {
            acc.insert(ch.first().unwrap().clone(), side);
        }

        self.all_decoded = Some(acc);
    }

    pub fn parse_outputs(&self) -> String {
        let mut acc = String::new();
        let lookup = self.all_decoded.as_ref().unwrap();

        for out in &self.output {
            let sides = out.chars()
                .map(|r| lookup.get(&r).unwrap())
                .collect::<Vec<_>>();

            // No validation needed
            let out_char = match sides.len() {
                2 => '1',
                3 => '7',
                4 => '4',
                5 => {
                    *vec!['2', '3', '5'].into_iter()
                        .filter(|num| {
                            sides.clone().into_iter()
                                .filter(|s| !Self::number_contains(*num, *s.clone()))
                                .collect::<Vec<_>>().len() == 0
                        }).collect::<Vec<_>>().first().unwrap()
                },
                6 => {
                    *vec!['0', '6', '9'].into_iter()
                        .filter(|num| {
                            sides.clone().into_iter()
                                .filter(|s| !Self::number_contains(*num, *s.clone()))
                                .collect::<Vec<_>>().len() == 0
                        }).collect::<Vec<_>>().first().unwrap()
                }
                7 => '8',
                _ => 'p'
            };

            acc.push(out_char)
        }

        acc
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

    let mut acc = 0;

    for mut d in displays {
        d.decode();
        acc += d.parse_outputs().parse::<i32>().unwrap();
    }

    println!("{}", acc);
}
