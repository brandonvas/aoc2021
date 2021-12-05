use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("File path missing");
    let values = data
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();
    let triple_sums : Vec<i32> = values.windows(3).map(|x| x.iter().sum()).collect();
    let ans = triple_sums.windows(2).filter(|x| x[0] < x[1]).count();
    println!("{:?}", ans);
}
