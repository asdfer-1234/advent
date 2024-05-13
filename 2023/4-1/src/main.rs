#[warn(missing_debug_implementations)]
use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut sum = 0;

    for line in input.split('\n') {
        if line == "" {
            continue;
        }
        let number_line = line.split(": ").collect::<Vec<_>>()[1]
            .split(" | ")
            .collect::<Vec<_>>();
        let posessed_numbers: Vec<u32> = number_line[1]
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let winning_numbers: Vec<u32> = number_line[0]
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let mut point = 0;
        for i in &posessed_numbers {
            for j in &winning_numbers {
                if i == j {
                    if point == 0 {
                        point = 1;
                    } else {
                        point *= 2;
                    }
                    break;
                }
            }
        }
        sum += point;
    }
    dbg!(sum);
}
