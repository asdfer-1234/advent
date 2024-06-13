#![warn(missing_debug_implementations)]

use grid::Grid;
use std::fs;
use std::str::FromStr;

use enum_map::{enum_map, Enum, EnumMap};

fn position_checked_add_signed(
    position: (usize, usize),
    vector: (isize, isize),
    size: (usize, usize),
) -> Option<(usize, usize)> {
    let added_position = (
        position.0.checked_add_signed(vector.0)?,
        position.1.checked_add_signed(vector.1)?,
    );

    match added_position.0 < size.0 && added_position.1 < size.1 {
        true => Some(added_position),
        false => None,
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Enum)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    const HORIZONTAL: [Direction; 2] = [Direction::Left, Direction::Right];
    const VERTICAL: [Direction; 2] = [Direction::Up, Direction::Down];

    fn vector(&self) -> (isize, isize) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }

    fn adjacent(&self) -> [Direction; 2] {
        match self {
            Direction::Left => Self::VERTICAL,
            Direction::Right => Self::VERTICAL,
            Direction::Up => Self::HORIZONTAL,
            Direction::Down => Self::HORIZONTAL,
        }
    }
}

#[derive(Debug)]
struct ParseError();

#[derive(Debug)]
struct Input {
    grid: Grid<u32>,
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = None;
        let mut vec = vec![];
        for (index, c) in s.chars().enumerate() {
            match c {
                '\n' => {
                    if width == None {
                        width = Some(index);
                    }
                }
                x => {
                    vec.push(x.to_digit(10).unwrap());
                }
            }
        }

        Ok(Self {
            grid: Grid::from_vec(vec, width.ok_or(ParseError())?),
        })
    }
}

fn min_changed(object: &mut Option<u32>, apply: u32) -> bool {
    match object {
        Some(x) => {
            let change = *x > apply;

            if change {
                *x = apply;
            }
            change
        }
        None => {
            *object = Some(apply);
            true
        }
    }
}

fn debug_movement_grid(grid: &Grid<EnumMap<Direction, [Option<u32>; 10]>>) {
    for y in 0..grid.size().1 {
        for x in 0..grid.size().0 {
            print!(
                "{: >8}",
                match grid[(x, y)]
                    .iter()
                    .filter_map(|(_, x)| x.iter().filter_map(|x| *x).min())
                    .min()
                {
                    Some(x) => x.to_string(),
                    None => String::from('_'),
                }
            );
        }
        println!();
    }
}

impl Input {
    fn solve(&self) -> Option<u32> {
        let size = self.grid.size();
        let cost_grid = &self.grid;

        let mut movement_grid: Grid<EnumMap<Direction, [Option<u32>; 10]>> = Grid::init(
            cost_grid.size().0,
            cost_grid.size().1,
            enum_map! {
                _ => [None; 10],
            },
        );

        movement_grid[(0, 0)] = enum_map! {
            _ => [Some(0), None, None, None, None, None, None, None, None, None],
        };

        let mut iteration = 0;

        loop {
            iteration += 1;
            dbg!(iteration);
            let mut changed = false;
            for y in 0..movement_grid.size().1 {
                for x in 0..movement_grid.size().0 {
                    let pos = (x, y);
                    let movement = movement_grid[pos]; // Implicitly Cloned
                    for (direction, directed_movement) in movement {
                        // Forward Movement
                        for (crucible_instability, cost) in directed_movement.iter().enumerate() {
                            let directed_pos =
                                match position_checked_add_signed(pos, direction.vector(), size) {
                                    Some(x) => x,
                                    None => continue,
                                };
                            if crucible_instability == 9 {
                                continue;
                            }

                            let cost = match cost {
                                Some(x) => x,
                                None => continue,
                            };
                            let cost_on_directed_pos = cost + cost_grid[directed_pos];

                            let reference = &mut movement_grid[directed_pos][direction]
                                [crucible_instability + 1];

                            changed |= min_changed(reference, cost_on_directed_pos);
                        }

                        // 90-degrees Turn Movement
                        for direction in direction.adjacent() {
                            let directed_pos =
                                match position_checked_add_signed(pos, direction.vector(), size) {
                                    Some(x) => x,
                                    None => continue,
                                };

                            // 0, 1, 2, 3, 4, 5, 6, 7, 8, 9
                            let cost =
                                match directed_movement.iter().skip(3).filter_map(|x| *x).min() {
                                    Some(x) => x,
                                    None => continue,
                                };

                            let cost_on_directed_pos = cost + cost_grid[directed_pos];

                            let reference = &mut movement_grid[directed_pos][direction][0];

                            changed |= min_changed(reference, cost_on_directed_pos);
                        }
                    }
                }
            }
            if !changed {
                break;
            }
        }

        let last_position = (movement_grid.size().0 - 1, movement_grid.size().1 - 1);

        debug_movement_grid(&movement_grid);

        let value = movement_grid[last_position]
            .iter()
            .map(|(_, x)| x.iter().skip(3).filter_map(|x| *x).min())
            .filter_map(|x| x)
            .min();
        value
    }
}

fn main() {
    dbg!(position_checked_add_signed((0, 0), (1, 1), (2, 2)));
    let input: Input = fs::read_to_string("input").unwrap().parse().unwrap();
    dbg!(input.solve());
}
