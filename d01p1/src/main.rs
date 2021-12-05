use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("File path missing");
    let values = data.lines().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    let ans = values.iter().zip(values[1..].iter()).filter(|(x, y)| x < y).count();
    println!("{:?}", ans);
}
