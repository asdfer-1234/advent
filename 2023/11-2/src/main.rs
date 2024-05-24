#![warn(missing_debug_implementations)]

use std::{
    collections::{hash_set, HashSet},
    fs,
    str::FromStr,
};

mod grid;
use grid::*;

#[derive(Debug)]
struct ParseError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Galaxy,
    Empty,
}

impl TryFrom<char> for Tile {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::Galaxy),
            '.' => Ok(Tile::Empty),
            _ => Err(ParseError {}),
        }
    }
}

#[derive(Debug)]
struct Input {
    grid: Grid<Tile>, // Do the grid right(not using jagged vectors) this time
}

fn empty_indices<I, J>(iter: I) -> Vec<usize>
where
    I: Iterator<Item = J>,
    J: Iterator<Item = Tile>,
{
    let mut indices = Vec::new();
    for (index, mut line) in iter.enumerate() {
        if line.all(|x| x == Tile::Empty) {
            indices.push(index);
        }
    }
    indices
}

fn galaxy_positions(grid: &Grid<Tile>) -> Vec<Position> {
    let mut vec = vec![];
    for (pos, tile) in grid {
        if *tile == Tile::Galaxy {
            vec.push(pos);
        }
    }
    vec
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let jagged: Vec<Vec<Tile>> = s
            .trim()
            .split('\n')
            .map(|x| {
                x.chars()
                    .filter_map(|c| TryInto::<Tile>::try_into(c).ok())
                    .collect()
            })
            .collect();
        let size = Position(jagged[0].len(), jagged.len());
        Ok(Input {
            grid: Grid::new_with(size, |Position(x, y)| jagged[y][x]),
        })
    }
}

impl Input {
    fn solve(&self) -> usize {
        let horizontal_empty_indices = empty_indices((0..self.grid.size().0).map(move |x| {
            (0..self.grid.size().1).map(move |y| *self.grid.get(Position(x, y)).unwrap())
        }));

        let vertical_empty_indices = empty_indices((0..self.grid.size().1).map(move |y| {
            (0..self.grid.size().0).map(move |x| *self.grid.get(Position(x, y)).unwrap())
        }));

        let positions = galaxy_positions(&self.grid);
        let mut distance_total = 0;
        let mut millions_to_be_added = 0;
        for i in 0..(positions.len() - 1) {
            for j in (i + 1)..positions.len() {
                dbg!(i, j);
                distance_total += Position::distance(positions[i], positions[j]);

                let horizontal_min = std::cmp::min(positions[i].0, positions[j].0);
                let horizontal_max = std::cmp::max(positions[i].0, positions[j].0);

                let vertical_min = std::cmp::min(positions[i].1, positions[j].1);
                let vertical_max = std::cmp::max(positions[i].1, positions[j].1);

                millions_to_be_added += horizontal_empty_indices.iter().fold(0, |init, x| {
                    match (horizontal_min..horizontal_max).contains(x) {
                        true => init + 1,
                        false => init,
                    }
                }) + vertical_empty_indices.iter().fold(0, |init, y| {
                    match (vertical_min..vertical_max).contains(y) {
                        true => init + 1,
                        false => init,
                    }
                });
            }
        }
        distance_total + millions_to_be_added * 999999
    }
}

fn main() {
    let input: Input = fs::read_to_string("input").unwrap().parse().unwrap();
    dbg!(&input.solve());
}
