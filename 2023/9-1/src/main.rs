#![warn(missing_debug_implementations)]

use std::{fs, str::FromStr};

#[derive(Debug)]
struct ParseError {}

#[derive(Debug)]
struct Input {
    histories: Vec<Vec<i32>>,
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input {
            histories: s
                .trim()
                .split('\n')
                .filter_map(|x| x.split(' ').map(|x| x.parse().ok()).collect())
                .collect(),
        })
    }
}

impl Input {
    fn extrapolate(history: &Vec<i32>) -> Option<i32> {
        if history.len() == 0 {
            return None;
        }
        let mut relativity_table = vec![history.clone()];
        let mut index = 0;

        loop {
            let mut relativity = vec![];
            for i in 0..(relativity_table[index].len() - 1) {
                let difference = relativity_table[index][i + 1] - relativity_table[index][i];
                relativity.push(difference);
            }
            relativity_table.push(relativity);
            index += 1;

            if relativity_table[index].iter().all(|x| *x == 0) {
                relativity_table[index].push(0);
                break;
            }
        }

        for i in (0..index).rev() {
            let difference = *(relativity_table[i + 1].last().unwrap());
            let before = *relativity_table[i].last().unwrap();
            relativity_table[i].push(before + difference);
        }
        dbg!(&relativity_table);

        relativity_table[0].last().cloned()
    }

    fn solve(&self) -> Option<i32> {
        let mut sum = 0;

        for history in &self.histories {
            sum += Self::extrapolate(history)?;
        }
        Some(sum)
    }
}

fn main() {
    let input: Input = fs::read_to_string("input").unwrap().parse().unwrap();

    dbg!(&input.solve());
}
