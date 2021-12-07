fn fuel_exp(steps: usize) -> usize {
    steps.clone() * (steps + 1) / 2
}

fn main() {
    let mut input = "16,1,2,0,4,2,7,1,2,14"
        .split(',')
        .map(|r| r.parse::<f64>().unwrap())
        .collect::<Vec<_>>();

    input.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Part 1
    let mid = input.len() / 2;
    let med = if input.len() % 2 == 0 {
        input[(mid-1)..(mid+1)].iter().fold(0f64, |a, r| a + r) / 2f64
    } else {
        input[mid] as f64
    };

    let total_to_move = input.iter().fold(0f64, |a, r| a + (r - med).abs());
    println!("{}", total_to_move);

    // Part 2
    let min = input.first().unwrap().clone() as usize;
    let max = input.last().unwrap().clone() as usize;
    
    let brute: usize = (min..max)
        .into_iter()
        .map(|pos| {
            input.iter().fold(0, |a, r| a + fuel_exp((r - (pos as f64)).abs() as usize))
        })
        .min()
        .unwrap();

    println!("{}", brute);
}
