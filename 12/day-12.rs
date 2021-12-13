use std::collections::HashMap;
use std::{fs, env};

#[derive(Debug)]
struct Path(String, String);

impl Path {
    pub fn new<S: ToString>(from: S, to: S) -> Self {
        Self(from.to_string(), to.to_string())
    }

    pub fn from(&self) -> String {
        self.0.clone()
    }

    pub fn to(&self) -> String {
        self.1.clone()
    }
}

#[derive(Debug)]
struct Route(Vec<String>);

#[derive(Debug)]
struct RouteMapper {
    input: Vec<Path>,
    acc: Vec<Route>
}

impl RouteMapper {
    const ENTRY: &'static str = "start";
    const EXIT: &'static str = "end";

    pub fn new(input: Vec<Path>) -> Self {
        Self { input, acc: vec![] }
    }

    fn paths_for(&self, entry: String) -> Vec<String> {
        self.input.iter()
            .filter_map(|p| if p.from() == entry { 
                Some(p.to()) 
            } else if p.to() == entry { 
                Some(p.from()) 
            } else { 
                None 
            }).collect::<Vec<_>>()
    }

    fn recursive_magic(&self, from: String, mut illigal: Vec<String>, mut path: Vec<String>) -> Vec<Vec<String>> {
        // Push current path
        path.push(from.clone());

        if from == Self::EXIT {
            return vec![path];
        }

        // Possible routes
        let paths = self.paths_for(from.clone());
        let paths = paths.iter()
            .filter(|to| to.chars().next().unwrap().is_uppercase() || (!illigal.contains(to) && to != &&Self::ENTRY.to_string()))
            .collect::<Vec<_>>();

        paths.iter()
            .flat_map(|p| {
                if p.chars().next().unwrap().is_uppercase() {
                    self.recursive_magic(p.to_string(), illigal.clone(), path.clone())
                } else {
                    if path.contains(&p.to_string()) {
                        illigal.push(p.to_string());
                    }
                    self.recursive_magic(p.to_string(), illigal.clone(), path.clone())
                }
            }).collect::<Vec<_>>()
    }

    pub fn crunch(&self) -> usize {
        let paths = self.recursive_magic(Self::ENTRY.to_string(), vec![], vec![]);

        for p in &paths {
            println!("{}", p.join(","));
        }

        paths.len()
    }
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let input = fs::read_to_string(args[1].clone()).unwrap();

    let paths = input.lines().map(|l| {
        let split = l.split("-").collect::<Vec<_>>();
        Path::new(split[0], split[1])
    }).collect::<Vec<_>>();

    let mapper = RouteMapper::new(paths);
    let len = mapper.crunch();

    println!("{}", len);
}
