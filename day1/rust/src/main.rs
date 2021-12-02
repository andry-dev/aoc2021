use std::fs;

fn count_increments(depths: &Vec<i32>) -> i32 {
    let mut count = 0;
    let mut prev = &depths[0];

    for x in depths {
        if x > prev {
            count += 1
        }

        prev = x;
    }

    count
}

fn main() {
    let input_contents = fs::read_to_string("data/input").expect("Can't find file.");

    let depths: Vec<i32> = input_contents
        .split('\n')
        .filter_map(|s| s.parse().ok())
        .collect();

    println!("{}", count_increments(&depths));

    let chunked_depths = &depths.windows(3).map(|x| x.iter().sum()).collect();

    println!("{}", count_increments(&chunked_depths));
}
