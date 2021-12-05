use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("File path missing");
    let res = data
        .lines()
        .map(|x| x.split_once(" ").unwrap())
        .map(|x| (x.0, x.1.parse::<i32>().unwrap()))
        .map(|x| match x { // first value in tuple is thrust delta, second value in tuple is aim delta
            ("forward", _)   => (x.1, 0),
            ("backwards", _) => (-x.1, 0),
            ("up", _)        => (0, -x.1),
            ("down", _)      => (0, x.1),
            (_ , _)          => (0, 0),
        })
        .fold(
            // aim, depth, position
            (0, 0, 0), |acc, x|(
                // The new aim is the old aim + the aim delta
                acc.0 + x.1,

                // The new depth is the old aim times the current thrust
                acc.1 + x.0 * acc.0,

                // The new position is the old position + thrust delta
                acc.2 + x.0)) ;
    println!("{:?}", res.1 * res.2);
}
