#![warn(missing_debug_implementations)]

use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

mod grid;
use grid::*;

#[derive(Debug)]
struct ParseError {}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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
    /*
    I should *really* make grid.rs into trait based definitions
    so I can handle these rotations with a simple .flip() function
    just like how iterators work

    ...or go a with a more hard and versatile solution called "taking a mathematical vector as a parameter".
    */

    fn tilt_north(&mut self) {
        for x in 0..self.grid.size().0 {
            let mut sphere_settle_point = 0;
            for y in 0..self.grid.size().1 {
                match self.grid.get(Position(x, y)).unwrap() {
                    Tile::Cube => sphere_settle_point = y + 1,
                    Tile::Sphere => {
                        let swap1 = Position(x, sphere_settle_point);
                        let swap2 = Position(x, y);

                        let swapper1 = *self.grid.get(swap1).unwrap();
                        let swapper2 = *self.grid.get(swap2).unwrap();
                        self.grid.set(swap1, swapper2).unwrap();
                        self.grid.set(swap2, swapper1).unwrap();

                        sphere_settle_point += 1;
                    }
                    Tile::Empty => (),
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for x in 0..self.grid.size().0 {
            let mut sphere_settle_point = self.grid.size().1 - 1;
            for y in (0..self.grid.size().1).rev() {
                match self.grid.get(Position(x, y)).unwrap() {
                    Tile::Cube => sphere_settle_point = y.saturating_sub(1),
                    Tile::Sphere => {
                        let swap1 = Position(x, sphere_settle_point);
                        let swap2 = Position(x, y);

                        let swapper1 = *self.grid.get(swap1).unwrap();
                        let swapper2 = *self.grid.get(swap2).unwrap();
                        self.grid.set(swap1, swapper2).unwrap();
                        self.grid.set(swap2, swapper1).unwrap();

                        sphere_settle_point = sphere_settle_point.saturating_sub(1);
                    }
                    Tile::Empty => (),
                }
            }
        }
    }
    fn tilt_west(&mut self) {
        for y in 0..self.grid.size().1 {
            let mut sphere_settle_point = 0;
            for x in 0..self.grid.size().0 {
                match self.grid.get(Position(x, y)).unwrap() {
                    Tile::Cube => sphere_settle_point = x + 1,
                    Tile::Sphere => {
                        let swap1 = Position(sphere_settle_point, y);
                        let swap2 = Position(x, y);

                        let swapper1 = *self.grid.get(swap1).unwrap();
                        let swapper2 = *self.grid.get(swap2).unwrap();
                        self.grid.set(swap1, swapper2).unwrap();
                        self.grid.set(swap2, swapper1).unwrap();

                        sphere_settle_point += 1;
                    }
                    Tile::Empty => (),
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for y in 0..self.grid.size().1 {
            let mut sphere_settle_point = self.grid.size().0 - 1;
            for x in (0..self.grid.size().0).rev() {
                match self.grid.get(Position(x, y)).unwrap() {
                    Tile::Cube => sphere_settle_point = x.saturating_sub(1),
                    Tile::Sphere => {
                        let swap1 = Position(sphere_settle_point, y);
                        let swap2 = Position(x, y);

                        let swapper1 = *self.grid.get(swap1).unwrap();
                        let swapper2 = *self.grid.get(swap2).unwrap();
                        self.grid.set(swap1, swapper2).unwrap();
                        self.grid.set(swap2, swapper1).unwrap();

                        sphere_settle_point = sphere_settle_point.saturating_sub(1);
                    }
                    Tile::Empty => (),
                }
            }
        }
    }

    fn load(grid: &Grid<Tile>) -> usize {
        let mut total_load = 0;
        for (position, t) in grid {
            if *t == Tile::Sphere {
                total_load += grid.size().1 - position.1;
            }
        }

        total_load
    }

    fn print(&self) {
        for y in 0..self.grid.size().1 {
            for x in 0..self.grid.size().1 {
                print!(
                    "{}",
                    match self.grid.get(Position(x, y)).unwrap() {
                        Tile::Cube => '#',
                        Tile::Sphere => 'O',
                        Tile::Empty => '.',
                    }
                )
            }
            println!();
        }
        println!();
    }

    fn solve(&mut self) -> usize {
        let mut hash_map: HashMap<Grid<Tile>, usize> = HashMap::new();
        let mut history = vec![];

        // whatever this number is—a billion?
        for cycle_count in 0..1_000_000_000 {
            history.push(self.grid.clone());
            if let Some(before_cycle) = hash_map.get(&self.grid) {
                let cycle_distance = cycle_count - before_cycle;
                let cycles_left = 1_000_000_000 - cycle_count;
                let offset = cycles_left % cycle_distance;
                return Self::load(&history[cycle_count - cycle_distance + offset]);
            }
            hash_map.insert(self.grid.clone(), cycle_count);

            self.tilt_north();
            self.tilt_west();
            self.tilt_south();
            self.tilt_east();
        }

        Self::load(&self.grid)
    }
}

fn main() {
    let mut input: Input = fs::read_to_string("input").unwrap().parse().unwrap();
    dbg!(input.solve());
}
