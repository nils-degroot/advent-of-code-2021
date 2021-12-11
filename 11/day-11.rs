use std::{fs, env, collections::HashMap};

#[derive(Debug)]
struct OctopusMatrix {
    inner: Vec<Vec<u32>>,
    flash_count: usize
}

impl OctopusMatrix {
    pub fn new(input: String) -> Self {
        Self {
            flash_count: 0,
            inner: input.lines().collect::<Vec<_>>()
                .iter()
                .map(|r| r.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        }
    }

    fn x_max(&self) -> i32 {
        self.inner.first().unwrap().len().try_into().unwrap()
    }

    fn y_max(&self) -> i32 {
        self.inner.len().try_into().unwrap()
    }

    fn increment_arround_point(&mut self, x: i32, y: i32) {
        for x_acc in (-1)..=1 {
            for y_acc in (-1)..=1 {
                let actual_x = (x + x_acc) as usize;
                let actual_y = (y + y_acc) as usize;

                if (x + x_acc >= 0 && x + x_acc < self.x_max()) && (y + y_acc >= 0 && y + y_acc < self.y_max()) {
                    self.inner[actual_x][actual_y] = self.inner[actual_x][actual_y] + 1;
                }
            }
        }
    }

    pub fn n_steps(&mut self, count: usize) {
        for _ in 0..count {
            self.step();
        }
    }

    pub fn step(&mut self) -> usize {
        let mut flashed_this_step = HashMap::new();

        // Increment all octopuses
        for x in 0..(self.y_max() as usize) {
            for y in 0..(self.x_max() as usize) {
                self.inner[x][y] = self.inner[x][y] + 1;
            }
        }

        loop {
            let mut flashes_this_round = 0;

            for x in 0..(self.y_max() as usize) {
                for y in 0..(self.x_max() as usize) {
                    if self.inner[x][y] > 9 && flashed_this_step.get(&(x, y)).is_none() {
                        flashes_this_round += 1;
                        flashed_this_step.insert((x, y), ());
                        self.increment_arround_point(x.try_into().unwrap(), y.try_into().unwrap())
                    }
                }
            }

            if flashes_this_round == 0 {
                for ((x, y), _) in &flashed_this_step {
                    self.inner[*x as usize][*y as usize] = 0;
                }

                self.flash_count += flashed_this_step.len();
                break flashed_this_step.len()
            }
        }
    }

    pub fn flashes(&self) -> usize {
        self.flash_count
    }

    pub fn print_matrix(&self) {
        for x in 0..(self.x_max()) {
            for y in 0..(self.y_max()) {
                print!("{} ", self.inner[x as usize][y as usize]);
            }
            println!();
        }
        println!();
    }

    pub fn total_len(&self) -> usize {
        self.inner.len() * self.inner.first().unwrap().len()
    }
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let input = fs::read_to_string(args[1].clone()).unwrap();

    // Part 1
    let mut matrix = OctopusMatrix::new(input.clone());
    matrix.n_steps(100);
    println!("{:?}", matrix.flashes());

    // Part 2
    let mut matrix = OctopusMatrix::new(input);
    let mut i = 0;
    loop { 
        i += 1;
        if matrix.step() == matrix.total_len() { break }
    }
    println!("{:?}", i);
}
