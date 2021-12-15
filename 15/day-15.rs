use std::{
    fs, 
    env, 
    collections::{HashMap, BinaryHeap},
    convert::TryInto
};

type Point = (i32, i32);

#[derive(Debug)]
struct Chiton {
    map: Vec<Vec<i32>>
}

impl Chiton {
    fn inc_overflow_safe(i: i32, inc: i32) -> i32 {
        if i + inc >= 10 {
            i + inc - 9
        } else {
            i + inc
        }
    }

    pub fn new(input: String, map_increments: i32) -> Self {
        let initial_map: Vec<Vec<i32>> = input.lines().map(|l| {
            l.chars()
                .map(|r| r.to_string().parse().unwrap())
                .collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        let top_row = (0..map_increments).map(|i| {
            initial_map.iter().map(|r| {
                r.iter()
                    .map(|c| Self::inc_overflow_safe(*c, i))
                    .collect::<Vec<_>>()
            }).collect::<Vec<_>>()
        }).fold(vec![], |mut a: Vec<Vec<i32>>, r| {
            r.iter().enumerate().for_each(|(i, row)| {
                if a.get(i).is_none() { a.push(vec![]); }
                let mut row = row.clone();
                a[i].append(&mut row);
            });
            a
        });

        let map = (0..map_increments).flat_map(|i| {
            top_row.iter().map(|r| {
                r.iter()
                    .map(|c| Self::inc_overflow_safe(*c, i))
                    .collect::<Vec<_>>()
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        Self { map }
    }

    fn end_pos(&self) -> Point {
        (
            (self.map.len() - 1).try_into().unwrap(),
            (self.map.last().unwrap().len() - 1).try_into().unwrap()
        )
    }

    fn neighbors(&self, pos: Point) -> Vec<Point> {
        let mut acc = vec![];
        let end = self.end_pos();

        let (x, y) = pos;

        if x < end.0 { acc.push((x + 1, y)) }
        if x > 0 { acc.push((x - 1, y)) }
        if y < end.1 { acc.push((x, y + 1)) }
        if y > 0 { acc.push((x, y - 1)) }
        
        acc
    }

    /// Runs a dijkstra that checks all paths 
    ///
    /// The return value will always be the cost of the smallest one
    pub fn dijkstra(&self) -> usize {
        let mut costs: HashMap<Point, usize> = HashMap::new();
        let mut heap = BinaryHeap::new();

        heap.push(((0, 0), 0));

        while let Some(pos) = heap.pop() {
            if self.end_pos() == pos.0 {
                continue;
            }

            if pos.1 <= *costs.get(&pos.0).unwrap_or(&usize::MAX) {
                for n in self.neighbors(pos.0) {
                    let new_cost = self.map[n.0 as usize][n.1 as usize] as usize + pos.1;

                    if new_cost < *costs.get(&n).unwrap_or(&usize::MAX) {
                        heap.push((n, new_cost));
                        costs.insert(n, new_cost);
                    }
                }
            }
        }

        *costs.get(&self.end_pos()).unwrap()
    }
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let file = fs::read_to_string(args[1].clone()).unwrap();

    let chiton = Chiton::new(file.clone(), 1);
    println!("Part 1: {:?}", chiton.dijkstra());

    let chiton = Chiton::new(file, 5);
    println!("Part 2: {:?}", chiton.dijkstra());
}
