use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("File path missing");
    let res = data
        .lines()
        .map(|x| x.split_once(" ").unwrap())
        .map(|x| (x.0, x.1.parse::<i32>().unwrap()))
        .map(|x| match x {
            ("forward", _)   => (x.1, 0),
            ("backwards", _) => (-x.1, 0),
            ("up", _)        => (0, -x.1),
            ("down", _)      => (0, x.1),
            (_ , _)          => (0, 0),
        })
        .fold((0,0), |acc, x| (acc.0+x.0, acc.1+x.1)) ;
    println!("{:?}", res.0 * res.1);
}
