use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let res = BingoGame::new(args).last_to_win();

    println!("Result: {}", res);
}

#[derive(Debug, Copy, Clone)]
struct BingoBoard {
    id: usize,
    inner: [[usize; 5]; 5]
}

impl BingoBoard {
    pub fn new(id: usize, input: Vec<usize>) -> Self {
        assert!(input.len() == 25, "A board should contain 25 inputs");

        let mut inner = [[0; 5]; 5];

        for x in 0..5 {
            for y in 0..5 {
                inner[x][y] = input[x * 5 + y];
            }
        }

        Self { id, inner }
    }
    
    pub fn get_id(&self) -> usize {
        self.id
    }

    fn sat_horizontal(&self, input: Vec<usize>) -> bool {
        for x in 0..5 {
            let mut check = true;

            for y in 0..5 {
                if !input.contains(&self.inner[x][y]) {
                    check = false;
                    break
                }
            }

            if check {
                return true;
            }
        }

        false
    }
    
    fn sat_vertical(&self, input: Vec<usize>) -> bool {
        for y in 0..5 {
            let mut check = true;

            for x in 0..5 {
                if !input.contains(&self.inner[x][y]) {
                    check = false;
                    break
                }
            }

            if check {
                return true;
            }
        }

        false
    }

    pub fn is_winner(&self, input: Vec<usize>) -> bool {
        self.sat_vertical(input.clone()) || self.sat_horizontal(input)
    }

    pub fn unmarked_numbers_sum(&self, rolled: Vec<usize>) -> usize {
        self.inner.iter()
            .fold(0, |acc, x| {
                acc + x.iter()
                    .filter(|y| !rolled.contains(y))
                    .fold(0, |a, r| a + r)
            })
    }
}

#[derive(Debug)]
struct BingoGame {
    boards: Vec<BingoBoard>,
    inputs: Vec<usize>
}

impl BingoGame {
    pub fn new(input: Vec<String>) -> Self {
        let inputs = input[1].split(",")
            .map(|r| r.parse::<usize>().expect("Some input was not a number"))
            .collect::<Vec<_>>();

        let mut boards = vec![];

        for i in 0..((input.len() - 2) / 25) {
            boards.push(BingoBoard::new(i, input[(i * 25 + 2)..((i + 1) * 25 + 2)]
                .iter()
                .map(|r| r.parse::<usize>().expect("Some input was not a number"))
                .collect::<Vec<_>>()));
        }

        Self { inputs, boards }
    }

    pub fn play(&self) -> usize {
        let mut rolled_balls: Vec<usize> = vec![];

        for ball in &self.inputs {
            rolled_balls.push(ball.clone());

            for board in &self.boards {
                if board.is_winner(rolled_balls.clone()) {
                    return board.unmarked_numbers_sum(rolled_balls) * ball
                }
            }
        }

        panic!("Failed to get a winner");
    } 

    pub fn last_to_win(&mut self) -> usize {
        let mut rolled_balls: Vec<usize> = vec![];

        for ball in &self.inputs {
            let mut indices_to_remove: Vec<usize> = vec![];
            rolled_balls.push(ball.clone());

            for board in &self.boards {
                if board.is_winner(rolled_balls.clone()) {
                    indices_to_remove.push(board.get_id());
                }
            }

            if self.boards.len() == 1 && self.boards.last().unwrap().is_winner(rolled_balls.clone()) {
                println!("{:?}", rolled_balls);
                println!("{:#?}", self.boards.first().unwrap());
                return self.boards
                    .first().unwrap()
                    .unmarked_numbers_sum(rolled_balls.clone()) * ball
            }

            self.boards = self.boards.iter().filter(|r| !indices_to_remove.contains(&r.get_id()))
                .map(|b| *b)
                .collect::<Vec<_>>();
        }

        panic!("Failed to get a winner");
    } 
}
