use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("File path missing");
    let vector_of_bits_ref: Vec<Vec<u32>> = data
        .lines()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();
    let bitwidth = vector_of_bits_ref[0].len();
    let count = vector_of_bits_ref.len();

    let mut half_count : f32;

    let mut bits_copy = vector_of_bits_ref.clone();
    for i in 0..bitwidth {
        if bits_copy.len() == 1 { break;}
        half_count = bits_copy.len() as f32 / 2.0;
        let bit_sum = (bits_copy.iter().fold(0, |acc, x| acc + x[i]) as f32 / half_count) as u32;
        bits_copy.retain(|x| x[i] == bit_sum.try_into().unwrap());
    }
    let oxy_number = bits_copy[0].iter().fold(0, |acc, x| acc << 1 | *x as u32);

    let mut bits_copy = vector_of_bits_ref.clone();
    for i in 0..bitwidth {
        if bits_copy.len() == 1 { break;}
        half_count = bits_copy.len() as f32 / 2.0;
        let mut bit_sum = (bits_copy.iter().fold(0, |acc, x| acc + x[i]) as f32 / half_count) as u32;
        bit_sum = 1 - (bit_sum%2);
        bits_copy.retain(|x| x[i] == bit_sum);
    }
    let co2_number = bits_copy[0].iter().fold(0, |acc, x| acc << 1 | *x as u32);

    println!("oxygen {:?}", oxy_number);
    println!("co2 {:?}", co2_number);
    println!("prod {:?}", oxy_number * co2_number);
}
