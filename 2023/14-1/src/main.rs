#![warn(missing_debug_implementations)]

use std::fs;
use std::str::FromStr;

mod grid;
use grid::*;

#[derive(Debug)]
struct ParseError {}

#[derive(Debug)]
enum Tile {
    Cube,
    Sphere,
    Empty,
}

#[derive(Debug)]
struct Input {
    grid: Grid<Tile>,
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vec = vec![];
        let mut width = None;
        for (i, c) in s.trim().chars().enumerate() {
            match c {
                '\n' => {
                    if width == None {
                        width = Some(i);
                    }
                }
                '#' => {
                    vec.push(Tile::Cube);
                }
                'O' => {
                    vec.push(Tile::Sphere);
                }
                '.' => {
                    vec.push(Tile::Empty);
                }
                _ => return Err(ParseError {}),
            }
        }

        Ok(Input {
            grid: Grid::with_vec(vec, width.ok_or(ParseError {})?).ok_or(ParseError {})?,
        })
    }
}

impl Input {
    fn solve(&self) -> Option<usize> {
        let mut total_load = 0;
        for x in 0..self.grid.size().0 {
            let mut sphere_settle_point = 0;
            for y in 0..self.grid.size().1 {
                match self.grid.get(Position(x, y))? {
                    Tile::Cube => sphere_settle_point = y + 1,
                    Tile::Sphere => {
                        total_load += self.grid.size().1 - sphere_settle_point;
                        sphere_settle_point += 1;
                    }
                    Tile::Empty => (),
                }
            }
        }
        Some(total_load)
    }
}

fn main() {
    let input: Input = fs::read_to_string("input").unwrap().parse().unwrap();

    dbg!(input.solve());
}
