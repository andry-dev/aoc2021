use std::fs;

fn bit_occurrences(lines: &Vec<&str>) -> Vec<i32> {
    let line_length = lines[0].len();

    let mut bit_count: Vec<i32> = vec![0; line_length];

    for numbers in lines {
        for (idx, bit) in numbers.char_indices() {
            if bit == '1' {
                bit_count[idx] += 1;
            } else {
                bit_count[idx] -= 1;
            }
        }
    }

    bit_count
}

fn most_common_bit_for_column(lines: &Vec<&str>, column: usize) -> char {
    let mut common_bit = 0;

    for i in 0..lines.len() {
        let c = lines[i].chars().nth(column).unwrap();
        if c == '1' {
            common_bit += 1;
        } else {
            common_bit -= 1;
        }
    }

    if common_bit >= 0 {
        '1'
    } else {
        '0'
    }
}

fn calculate_power_consumption(common_bits: &Vec<i32>) -> i32 {
    let mut gamma_str = String::new();
    let mut epsilon_str = String::new();

    for b in common_bits.iter() {
        if b >= &0 {
            gamma_str += "1";
            epsilon_str += "0";
        } else {
            gamma_str += "0";
            epsilon_str += "1";
        }
    }

    i32::from_str_radix(&gamma_str, 2).unwrap() * i32::from_str_radix(&epsilon_str, 2).unwrap()
}

fn calculate_life_support(lines: &Vec<&str>) -> i32 {
    let mut oxygen_lines = lines.clone();
    let mut current_bit = 0;
    while oxygen_lines.len() > 1 {
        let most_common_bit = most_common_bit_for_column(&oxygen_lines, current_bit);

        oxygen_lines.retain(|x| x.chars().nth(current_bit).unwrap() == most_common_bit);

        current_bit += 1;
    }

    let mut co2_lines = lines.clone();
    let mut current_bit = 0;
    while co2_lines.len() > 1 {
        let most_common_bit = most_common_bit_for_column(&co2_lines, current_bit);

        co2_lines.retain(|x| x.chars().nth(current_bit).unwrap() != most_common_bit);

        current_bit += 1;
    }

    i32::from_str_radix(oxygen_lines[0], 2).unwrap() * i32::from_str_radix(co2_lines[0], 2).unwrap()
}

fn main() {
    let input_contents = fs::read_to_string("data/input").expect("Can't find file!");

    let lines: Vec<&str> = input_contents.split('\n').filter(|&s| s != "").collect();

    let bit_occurrences = bit_occurrences(&lines);
    let power_consumption = calculate_power_consumption(&bit_occurrences);

    println!("Power consumption: \t{}", power_consumption);

    let oxygen = calculate_life_support(&lines);

    println!("Life support rating: \t{}", oxygen);
}
