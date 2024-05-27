#![warn(missing_debug_implementations)]

use std::{fs, str::FromStr};

#[derive(Debug)]
struct ParseError {}

impl From<std::num::ParseIntError> for ParseError {
    fn from(_: std::num::ParseIntError) -> Self {
        ParseError {}
    }
}

#[derive(Debug)]
struct Row {
    springs: Vec<Option<bool>>,
    pattern: Vec<usize>,
}

impl FromStr for Row {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs_string, pattern_string) = s.split_once(' ').ok_or(ParseError {})?;
        let mut springs = vec![];
        let mut pattern = vec![];
        for c in springs_string.chars() {
            springs.push(match c {
                '#' => Some(true),
                '.' => Some(false),
                '?' => None,
                _ => return Err(ParseError {}),
            })
        }

        for i in pattern_string.split(',') {
            pattern.push(i.parse()?);
        }

        // Second gold star
        let springs_len = springs.len();
        let pattern_len = pattern.len();
        for _ in 0..4 {
            springs.push(None);
            springs.extend_from_within(..springs_len);
            pattern.extend_from_within(..pattern_len);
        }

        Ok(Row { springs, pattern })
    }
}

impl Row {
    fn empty_states(&self) -> Vec<Vec<usize>> {
        let mut states: Vec<Vec<usize>> = vec![];
        states.resize(self.pattern.len(), vec![]);
        for i in 0..states.len() {
            states[i].resize(self.pattern[i] + 1, 0);
        }
        states.push(vec![0]);
        states
    }

    fn broken_states(&self, states: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let mut new_states = self.empty_states();
        for i in 0..states.len() {
            for j in 0..(states[i].len() - 1) {
                new_states[i][j + 1] = states[i][j];
            }
        }
        new_states
    }

    fn normal_states(&self, states: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let mut new_states = self.empty_states();
        for i in 0..states.len() {
            new_states[i][0] = states[i][0];
            if i != 0 {
                new_states[i][0] += states[i - 1][states[i - 1].len() - 1];
            }
        }
        new_states
    }

    fn unknown_states(&self, states: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let broken = self.broken_states(states);
        let normal = self.normal_states(states);
        let mut new_states = self.empty_states();
        for i in 0..new_states.len() {
            for j in 0..new_states[i].len() {
                new_states[i][j] = broken[i][j] + normal[i][j];
            }
        }
        new_states
    }

    fn solve(&self) -> usize {
        let mut states = self.empty_states();

        states[0][0] = 1;

        for spring in &self.springs {
            states = match spring {
                Some(true) => self.broken_states(&states),
                Some(false) => self.normal_states(&states),
                None => self.unknown_states(&states),
            };
        }
        let states_len = states.len();
        let last_segment_len = states[states_len - 2].len();
        states[states_len - 2][last_segment_len - 1] + states[states_len - 1][0]
    }
}

#[derive(Debug)]
struct Input {
    rows: Vec<Row>,
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows: Vec<Row> = vec![];
        for l in s.trim().split('\n') {
            rows.push(l.parse()?);
        }
        Ok(Input { rows })
    }
}

impl Input {
    fn solve(&self) -> usize {
        self.rows.iter().fold(0, |init, x| init + x.solve())
    }
}

fn main() {
    let input: Input = fs::read_to_string("input").unwrap().parse().unwrap();
    dbg!(input.solve());
}
