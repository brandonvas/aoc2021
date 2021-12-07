use std::fs;

fn calculate(bitwidth : usize, mut data :Vec<Vec<u32>>, mode : bool) -> u32 {
    let mut half_count;
    for i in 0..bitwidth {
        if data.len() == 1 { break;}
        half_count = data.len() as f32 / 2.0;
        let bit_sum = (data.iter().fold(0, |acc, x| acc + x[i]) as f32 / half_count) as u32;
        data.retain(|x| x[i] == if mode {bit_sum} else {1-(bit_sum%2)});
    }
    data[0].iter().fold(0, |acc, x| acc << 1 | *x as u32)
}

fn main() {
    let data = fs::read_to_string("example.txt").expect("File path missing");
    let vector_of_bits_ref: Vec<Vec<u32>> = data
        .lines()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();
    let bitwidth = vector_of_bits_ref[0].len();

    let mut bits_copy = vector_of_bits_ref.clone();
    let oxy_number = calculate(bitwidth, bits_copy, true);
    bits_copy = vector_of_bits_ref.clone();
    let co2_number = calculate(bitwidth, bits_copy, false);
    println!("prod {:?}", oxy_number * co2_number);
}