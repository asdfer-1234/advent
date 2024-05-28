#![warn(missing_debug_implementations)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::{fs, str::FromStr};

mod grid;
use grid::*;

#[derive(Debug)]
struct ParseError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Orientation {
    Horizontal,
    Vertical,
}

fn reflect_usize(value: usize, mirror: usize) -> Option<usize> {
    if value < mirror {
        Some(value + (mirror - value) * 2 - 1)
    } else {
        Some(value.checked_sub((value - mirror) * 2 + 1)?)
    }
}

fn reflect_horizontally(value: &Position, mirror: usize) -> Option<Position> {
    Some(Position(reflect_usize(value.0, mirror)?, value.1))
}

fn reflect_vertically(value: &Position, mirror: usize) -> Option<Position> {
    Some(Position(value.0, reflect_usize(value.1, mirror)?))
}

impl Grid<bool> {
    fn reflects_with_function<F>(&self, mirror: F) -> bool
    where
        F: Fn(&Position) -> Option<Position>,
    {
        self.size().into_iter().fold(0, |init, position| {
            let reflected_position = match mirror(&position) {
                Some(x) => x,
                None => return init,
            };
            let original_get = match self.get(position) {
                Some(x) => x,
                None => return init,
            };
            let reflected_get = match self.get(reflected_position) {
                Some(x) => x,
                None => return init,
            };
            init + if original_get == reflected_get { 0 } else { 1 }
        }) == 2
    }

    fn solve(&self) -> usize {
        for y in 1..self.size().1 {
            if self.reflects_with_function(|p| reflect_vertically(p, y)) {
                return y * 100;
            }
        }

        for x in 1..self.size().0 {
            if self.reflects_with_function(|p| reflect_horizontally(p, x)) {
                return x;
            }
        }
        return 0;
    }
}

#[derive(Debug)]
struct Input {
    grids: Vec<Grid<bool>>,
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grids: Vec<Grid<bool>> = vec![];
        for g in s.trim().split("\n\n") {
            let mut width = None;
            let mut vec = vec![];
            for (i, c) in g.chars().enumerate() {
                match c {
                    '\n' => {
                        if width == None {
                            width = Some(i);
                        }
                    }
                    '#' => vec.push(true),
                    '.' => vec.push(false),
                    _ => return Err(ParseError {}),
                }
            }
            grids.push(Grid::with_vec(vec, width.ok_or(ParseError {})?).ok_or(ParseError {})?);
        }
        Ok(Input { grids })
    }
}

impl Input {
    fn solve(&self) -> usize {
        self.grids
            .iter()
            .fold(0, |init, x| init + Grid::<bool>::solve(x))
    }
}

fn main() {
    let input: Input = fs::read_to_string("input").unwrap().parse().unwrap();
    dbg!(input.solve());
}
