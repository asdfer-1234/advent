#![warn(missing_debug_implementations)]

use std::{fs, ops::Neg, str::FromStr};

#[derive(Debug)]
struct ParseError {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Vector(i32, i32);

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector(-self.0, -self.1)
    }
}

const WEST: Vector = Vector(-1, 0);
const EAST: Vector = Vector(1, 0);
const NORTH: Vector = Vector(0, -1);
const SOUTH: Vector = Vector(0, 1);
const ADJACENT: [Vector; 4] = [WEST, EAST, NORTH, SOUTH];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Position(usize, usize);

impl Position {
    fn checked_add(&self, v: Vector) -> Option<Position> {
        Some(Position(
            TryInto::<usize>::try_into(TryInto::<i32>::try_into(self.0).ok()? + v.0).ok()?,
            TryInto::<usize>::try_into(TryInto::<i32>::try_into(self.1).ok()? + v.1).ok()?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PipeCell {
    None,
    Start,
    Horizontal,
    Vertical,
    SouthEast,
    SouthWest,
    NorthEast,
    NorthWest,
}

impl TryFrom<char> for PipeCell {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(PipeCell::None),
            'S' => Ok(PipeCell::Start),
            '-' => Ok(PipeCell::Horizontal),
            '|' => Ok(PipeCell::Vertical),
            'F' => Ok(PipeCell::SouthEast),
            '7' => Ok(PipeCell::SouthWest),
            'L' => Ok(PipeCell::NorthEast),
            'J' => Ok(PipeCell::NorthWest),
            _ => Err(ParseError {}),
        }
    }
}

impl PipeCell {
    fn connections(&self) -> Option<(Vector, Vector)> {
        match self {
            PipeCell::None => None,
            PipeCell::Start => None,
            PipeCell::Horizontal => Some((WEST, EAST)),
            PipeCell::Vertical => Some((NORTH, SOUTH)),
            PipeCell::SouthEast => Some((SOUTH, EAST)),
            PipeCell::SouthWest => Some((SOUTH, WEST)),
            PipeCell::NorthEast => Some((NORTH, EAST)),
            PipeCell::NorthWest => Some((NORTH, WEST)),
        }
    }

    fn other_connection(&self, position: Vector) -> Option<Vector> {
        let connections = self.connections()?;
        if connections.0 == position {
            Some(connections.1)
        } else if connections.1 == position {
            Some(connections.0)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Input {
    grid: Vec<Vec<PipeCell>>,
    width: usize,
    height: usize,
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = vec![];
        for l in s.trim().split('\n') {
            let mut row = vec![];
            for c in l.chars() {
                row.push(TryInto::<PipeCell>::try_into(c)?);
            }
            grid.push(row);
        }
        let width = grid[0].len();
        let height = grid.len();
        Ok(Input {
            grid,
            width,
            height,
        })
    }
}

impl Input {
    fn start_position(&self) -> Option<Position> {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if self.grid[y][x] == PipeCell::Start {
                    return Some(Position(x, y));
                }
            }
        }
        None
    }

    fn at(&self, position: Position) -> PipeCell {
        self.grid[position.1][position.0]
    }

    fn solve(&self) -> Option<u32> {
        let start_position = self.start_position()?;

        for starting_direction in ADJACENT {
            let mut position = start_position;
            let mut direction = starting_direction;

            let mut pipe_length = 0;

            let mut result_found = false;

            loop {
                pipe_length += 1;
                position = match position.checked_add(direction) {
                    Some(x) => x,
                    None => break,
                };
                let pipe_cell = self.at(position);

                if pipe_cell == PipeCell::Start {
                    result_found = true;
                    break;
                }

                direction = match pipe_cell.other_connection(-direction) {
                    Some(x) => x,
                    None => break,
                };
                dbg!(&position, &direction);
            }

            if result_found {
                return Some(pipe_length / 2);
            }
        }

        None
    }
}

fn main() {
    let input: Input = fs::read_to_string("input").unwrap().parse().unwrap();
    dbg!(&input);
    dbg!(&input.solve());
}
