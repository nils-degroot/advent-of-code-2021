use std::collections::HashMap;
use std::{env, str::FromStr, num::ParseIntError};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point(usize, usize);

fn range(lhs: usize, rhs: usize) -> Box<dyn Iterator<Item = usize>> {
    if lhs < rhs {
        Box::new(lhs..=rhs)
    } else {
        Box::new((rhs..=lhs).rev())
    }
}

impl Point {
    fn draw_line(&self, rhs: Point) -> Vec<Point> {
        let mut acc = vec![];

        if self.0 == rhs.0 {
            range(self.1, rhs.1).for_each(|i| {
                acc.push(Point::from_str(format!("{},{}", self.0, i).as_str()).unwrap());
            });
        } else if self.1 == rhs.1 {
            range(self.0, rhs.0).for_each(|i| {
                acc.push(Point::from_str(format!("{},{}", i, self.1).as_str()).unwrap());
            });
        } else { // Part 2
            range(self.0, rhs.0).zip(range(self.1, rhs.1)).for_each(|(x, y)| {
                acc.push(Point::from_str(format!("{},{}", x, y).as_str()).unwrap());
            });
        }
        

        acc
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').collect::<Vec<_>>();
        Ok(Point(parts[0].parse()?, parts[1].parse()?))
    }
}

type VentLine = (Point, Point);

#[derive(Debug)]
struct OceanFloor(Vec<VentLine>);

impl OceanFloor {
    pub fn new(input: Vec<String>) -> Self {
        Self(input.chunks(2).map(|chunk| match chunk {
            [l, r] => (Point::from_str(l).unwrap(), Point::from_str(r).unwrap()),
            _ => panic!("Got a odd amounth of arguments")
        }).collect::<Vec<_>>())
    }

    pub fn get_overlapping(&self) -> usize {
        self.0.iter().flat_map(|pair| {
            pair.0.draw_line(pair.1.clone())
        }).fold(HashMap::new(), |mut a, r| {
            match a.get_mut(&r) {
                Some(ptr) => { *ptr += 1; },
                None => { a.insert(r, 1); },
            };
            a
        }).iter()
            .filter(|(_, n)| *n > &1)
            .collect::<Vec<_>>().len()
    }
}

fn main() {
    let args = env::args()
        .filter(|r| r != "->")
        .collect::<Vec<_>>();

    let ocean_floor = OceanFloor::new(args[1..].to_vec());
    
    println!("{}", ocean_floor.get_overlapping());
}
