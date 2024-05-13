#[warn(missing_debug_implementations)]
use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut counts = vec![];

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
        let mut count = 0;
        for i in &posessed_numbers {
            for j in &winning_numbers {
                if i == j {
                    count += 1;
                    break;
                }
            }
        }
        counts.push(count);
    }

    let mut cards = counts.iter().map(|_| 1).collect::<Vec<_>>();

    for i in 0..counts.len() {
        for j in 0..counts[i] {
            cards[i + j + 1] += cards[i];
        }
    }

    dbg!(cards.iter().fold(0, |a, x| a + x));
}
