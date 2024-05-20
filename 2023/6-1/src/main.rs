#![warn(missing_debug_implementations)]

use std::fs;

use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct ParseError {}

#[derive(Debug)]
struct Input {
    time_record: Vec<(u32, u32)>,
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut time_record = vec![];

        let splitted = s
            .split('\n')
            .filter(|x| !x.is_empty())
            .map(|x| {
                x.split_once(':')
                    .unwrap()
                    .1
                    .split_whitespace()
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        for i in 0..splitted[0].len() {
            time_record.push((splitted[0][i], splitted[1][i]));
        }
        Ok(Input { time_record })
    }
}

fn main() {
    let input = fs::read_to_string("input")
        .unwrap()
        .parse::<Input>()
        .unwrap();
    let mut product = 1;
    for (time, record) in input.time_record {
        let mut case_count = 0;
        for i in 0..=time {
            if i * (time - i) > record {
                case_count += 1;
            }
        }
        product *= case_count;
    }
    dbg!(product);
}
