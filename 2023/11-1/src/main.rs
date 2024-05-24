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

fn expand(grid: &Grid<Tile>) -> Grid<Tile> {
    let horizontal_empty_indices = empty_indices(
        (0..grid.size().0)
            .map(move |x| (0..grid.size().1).map(move |y| *grid.get(Position(x, y)).unwrap())),
    );

    let vertical_empty_indices = empty_indices(
        (0..grid.size().1)
            .map(move |y| (0..grid.size().0).map(move |x| *grid.get(Position(x, y)).unwrap())),
    );
    let total_offset = Position(horizontal_empty_indices.len(), vertical_empty_indices.len());

    let mut new_grid = Grid::new_with(grid.size() + total_offset, |_| Tile::Empty);

    dbg!(&horizontal_empty_indices);

    for i in grid.size().into_iter() {
        let offset = Position(
            horizontal_empty_indices
                .iter()
                .fold(0, |offset, x| match *x <= i.0 {
                    true => offset + 1,
                    false => offset,
                }),
            vertical_empty_indices
                .iter()
                .fold(0, |offset, x| match *x <= i.1 {
                    true => offset + 1,
                    false => offset,
                }),
        );
        new_grid.set(i + offset, *grid.get(i).unwrap()).unwrap();
    }

    new_grid
}

fn galaxy_positions(grid: Grid<Tile>) -> Vec<Position> {
    let mut vec = vec![];
    for (pos, tile) in grid {
        if tile == Tile::Galaxy {
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
        let new_grid = expand(&self.grid);

        for j in 0..new_grid.size().1 {
            for i in 0..new_grid.size().0 {
                match new_grid.get(Position(i, j)).unwrap() {
                    Tile::Galaxy => print!("#"),
                    Tile::Empty => print!("."),
                }
            }
            println!();
        }

        let positions = galaxy_positions(new_grid);
        let mut distance_total = 0;
        for i in 0..(positions.len() - 1) {
            for j in (i + 1)..positions.len() {
                distance_total += Position::distance(positions[i], positions[j]);
                println!(
                    "{}({:?}), {}({:?}) -> {}",
                    i,
                    positions[i],
                    j,
                    positions[j],
                    Position::distance(positions[i], positions[j])
                )
            }
        }
        distance_total
    }
}

fn main() {
    let input: Input = fs::read_to_string("input").unwrap().parse().unwrap();
    dbg!(&input.solve());
}
