use std::{fs, env};

#[derive(Debug)]
struct FoldingPaper {
    dots: Vec<(i32, i32)>,
    folds: Vec<(char, i32)>,
    size: (i32, i32)
}

impl FoldingPaper {
    pub fn new(dots: Vec<(i32, i32)>, folds: Vec<(char, i32)>) -> Self {
        let size = dots.iter()
            .fold((0, 0), |mut a, dot| {
                if dot.0 > a.0 { a.0 = dot.0; }
                if dot.1 > a.1 { a.1 = dot.1; }
                a
            });

        Self { dots, folds, size }
    }

    pub fn print(&self) {
        for y in 0..=(self.size.1) {
            let row_dots = self.dots.iter()
                .filter(|(_, inner_y)| inner_y == &y)
                .collect::<Vec<_>>();

            for x in 0..=(self.size.0) {
                if row_dots.iter().find(|(inner_x, _)| inner_x == &x).is_some() {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    fn dot_pos(&self, p: i32, l: i32) -> i32 {
        (p - l * 2).abs()
    }

    pub fn total_dots(&self) -> usize {
        self.dots.len()
    }

    pub fn can_fold(&self) -> bool {
        self.folds.len() > 0
    }

    pub fn perform_fold(&self) -> Self {
        let mut folds = self.folds.clone().into_iter();
        let fold = folds.next().unwrap();
        let folds = folds.collect::<Vec<_>>();

        let mut dots = self.dots.clone();
        let mut size = self.size.clone();

        dots = dots.into_iter().map(|mut d| {
            if fold.0 == 'y' && d.1 > fold.1 { 
                d.1 = self.dot_pos(d.1, fold.1);
            } else if fold.0 == 'x' && d.0 > fold.1 {
                d.0 = self.dot_pos(d.0, fold.1);
            }
            d
        }).collect::<Vec<_>>();

        if fold.0 == 'y' {
            size.1 = size.1 / 2 - 1;
        } else if fold.0 == 'x' {
            size.0 = size.0 / 2 - 1;
        } else { panic!("Very illigal"); }

        let mut dots_filted = vec![];
        
        for d in dots {
            if !dots_filted.contains(&d) { dots_filted.push(d.clone()); }
        }

        Self { folds, dots: dots_filted, size }
    }
} 

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let input = fs::read_to_string(args[1].clone()).unwrap();

    let mut dots = vec![];
    let mut folds = vec![];

    for l in input.lines() {
        if l.starts_with("fold") {
            let mut part = l.split_whitespace().collect::<Vec<_>>()
                .last().unwrap().split("=");
            folds.push((
                part.next().unwrap().chars().next().unwrap(),
                part.next().unwrap().parse().unwrap()
            ));
        } else if !l.is_empty() {
            let mut split = l.split(",");
            dots.push((
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap()
            ));
        }
    }

    let mut paper = FoldingPaper::new(dots, folds);
    paper = paper.perform_fold();

    println!("Dots after first fold: {}", paper.total_dots());

    loop {
        if paper.can_fold() {
            paper = paper.perform_fold();
        } else {
            break;
        }
    }

    println!("Secret code:");
    paper.print();
}
