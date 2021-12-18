use crate::SnailfishNumber::{Pair, Number};
use itertools::Itertools;
use std::{
    fs,
    env,
    str::FromStr,
    num::ParseIntError,
};

#[derive(Debug, Clone)]
enum SnailfishNumber {
    Number(u8),
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>)
}

impl SnailfishNumber {
    pub fn add(&self, rhs: Self) -> Self {
        let mut resulting_pair = SnailfishNumber::Pair(
            Box::new(self.clone()),
            Box::new(rhs)
        );

        loop {
            let (updated, _, _, exploded) = resulting_pair.explode(0);

            if exploded {
                resulting_pair = updated;
                continue;
            }

            let (updated, splitted) = updated.split();

            if splitted {
                resulting_pair = updated;
                continue;
            }

            break updated;
        }
    }

    fn add_left(&self, value: Option<u8>) -> SnailfishNumber {
        match value {
            Some(v) => match self {
                Number(n) => Number(n + v),
                Pair(l, r) => Pair(Box::new(l.add_left(Some(v))), r.clone()),
            },
            None => self.clone(),
        }
        
    }

    fn add_right(&self, value: Option<u8>) -> SnailfishNumber {
        match value {
            Some(v) => match self {
                Number(n) => Number(n + v),
                Pair(l, r) => Pair(l.clone(), Box::new(r.add_right(Some(v)))),
            },
            None => self.clone(),
        }
        
    }

    fn split(&self) -> (Self, bool) {
        match self {
            Number(n) => {
                if n > &9 {
                    let res = (*n as f32) / 2f32;
                    (Self::Pair(
                        Box::new(SnailfishNumber::Number(res.floor() as u8)),
                        Box::new(SnailfishNumber::Number(res.ceil() as u8))
                    ), true)
                } else {
                    (self.clone(), false)
                }
            },
            Pair(lhs, rhs) => {
                let (updated, splited) = lhs.split();

                if splited {
                    (Pair(Box::new(updated), rhs.clone()), true)
                } else {
                    let (updated, splited) = rhs.split();
                    (Pair(lhs.clone(), Box::new(updated)), splited)
                }
            }
        }
    }

    pub fn explode(&self, depth: usize) -> (Self, Option<u8>, Option<u8>, bool) {
        match self {
            Number(_) => (self.clone(), None, None, false),
            Pair(lhs, rhs) => {
                if depth > 3 {
                    match (&**lhs, &**rhs) {
                        (Number(l), Number(r)) => (Number(0), Some(*l), Some(*r), true),
                        _ => unreachable!()
                    }
                } else {
                    let (l_update, l_ret, l_add, exploded) = lhs.explode(depth + 1);

                    if exploded {
                        (Pair(Box::new(l_update), Box::new(rhs.add_left(l_add))), l_ret, None, true)
                    } else {
                        let (r_update, r_add, r_ret, exploded) = rhs.explode(depth + 1);
                        (Pair(Box::new(l_update.add_right(r_add)), Box::new(r_update)), None, r_ret, exploded)
                    }
                }
            },
        }
    }

    pub fn magnitude(&self) -> usize {
        match self {
            Pair(lhs, rhs) => {
                lhs.magnitude() * 3 + rhs.magnitude() * 2
            },
            Number(n) => *n as usize
        }
    }
}

impl FromStr for SnailfishNumber {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_char = s.chars().next().unwrap();

        if first_char.is_numeric() {
            Ok(SnailfishNumber::Number(first_char.to_string().parse()?))
        } else {
            let mut layer = 0;
            let mut split = 0;

            for (i, c) in s.chars().enumerate() {
                if c == '[' {
                    layer += 1;
                } else if c == ']' {
                    layer -= 1;
                    
                } else if layer == 1 && c == ',' {
                    split = i;
                    break;
                }
            }

            let lhs = s[1..split].to_string();
            let rhs = s[(split + 1)..(s.len() - 1)].to_string();

            Ok(Pair(
                Box::new(Self::from_str(&lhs).unwrap()),
                Box::new(Self::from_str(&rhs).unwrap())
            ))
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let file = fs::read_to_string(args[1].clone()).unwrap();
    let lines = file.lines().collect::<Vec<_>>();
    let mut iter = lines.iter();
    let mut pairs = SnailfishNumber::from_str(iter.next().unwrap()).unwrap();

    while let Some(line) = iter.next() {
        let num = SnailfishNumber::from_str(line).unwrap();
        pairs = pairs.add(num);
    }

    let max = lines.into_iter()
        .permutations(2)
        .map(|p| SnailfishNumber::from_str(p[0]).unwrap().add(SnailfishNumber::from_str(p[1]).unwrap()).magnitude()
    ).max().unwrap();

    println!("Part 1: {}", pairs.magnitude());
    println!("Part 2: {}", max);
}
