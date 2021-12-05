use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("File path missing");
    let vector_of_bits: Vec<Vec<u32>> = data
        .lines()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();
    let bitwidth = vector_of_bits[0].len();
    let count = vector_of_bits.len();
    let mut bit_counts = vec![0; bitwidth];
    for row in 0..count {
        for bit in 0..bitwidth {
            bit_counts[bit] += vector_of_bits[row][bit];
        }
    }
    let gamma_bits  : Vec<u8> = bit_counts.iter().map(|x| (x/(count as u32/2)) as u8).collect();
    let lambda_bits : Vec<u8> = gamma_bits.iter().map(|x| 1-x).collect();
    let gamma  = gamma_bits.iter().fold(0, |res, b|  (res << 1) | *b as u32);
    let lambda = lambda_bits.iter().fold(0, |res, b|  (res << 1) | *b as u32);
    println!("bitwidth {} count {}", bitwidth, count);
    println!("gamma {:?}", gamma);
    println!("lambda {:?}", lambda);
    println!("product {:?}", lambda * gamma);
}
