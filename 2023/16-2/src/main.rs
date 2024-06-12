use rayon::prelude::*;
use std::{fs, str::FromStr};

mod grid;
use grid::*;

mod bender {
    pub mod direction {
        use crate::grid::Vector;
        use enum_map::Enum;
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Enum)]
        pub enum Direction {
            Left,
            Right,
            Up,
            Down,
        }

        impl Direction {
            pub fn vector(&self) -> Vector {
                match self {
                    Direction::Left => Vector(-1, 0),
                    Direction::Right => Vector(1, 0),
                    Direction::Up => Vector(0, -1),
                    Direction::Down => Vector(0, 1),
                }
            }
        }
    }

    use direction::Direction;
    use direction::Direction::*;
    use enum_map::{enum_map, EnumMap};

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct Light {
        pub map: EnumMap<Direction, bool>,
    }

    impl Light {
        pub fn empty() -> Self {
            Light {
                map: enum_map! {
                    _ => false,
                },
            }
        }

        pub fn left() -> Self {
            Light {
                map: enum_map! {
                    Left => true,
                    _ => false,
                },
            }
        }

        pub fn right() -> Self {
            Light {
                map: enum_map! {
                    Right => true,
                    _ => false,
                },
            }
        }

        pub fn up() -> Self {
            Light {
                map: enum_map! {
                    Up => true,
                    _ => false,
                },
            }
        }

        pub fn down() -> Self {
            Light {
                map: enum_map! {
                    Down => true,
                    _ => false,
                },
            }
        }

        pub fn horizontal_split() -> Self {
            Light {
                map: enum_map! {
                    Left => true,
                    Right => true,
                    _ => false,
                },
            }
        }

        pub fn vertical_split() -> Self {
            Light {
                map: enum_map! {
                    Up => true,
                    Down => true,
                    _ => false,
                },
            }
        }

        pub fn apply_all<F>(self, other: Self, mut f: F) -> Self
        where
            F: FnMut(bool, bool) -> bool,
        {
            Light {
                map: enum_map! {
                    x => f(self.map[x], other.map[x]),
                },
            }
        }

        pub fn or(self, other: Self) -> Self {
            self.apply_all(other, |a, b| a || b)
        }

        pub fn and(self, other: Self) -> Self {
            self.apply_all(other, |a, b| a && b)
        }

        pub fn superior_to(self, other: Self) -> bool {
            self == self.and(other)
        }

        pub fn energized(self) -> bool {
            self.map.iter().any(|(_, x)| *x)
        }
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct Bender {
        map: EnumMap<Direction, Light>,
    }

    impl Bender {
        pub fn empty() -> Self {
            Bender {
                map: enum_map! {
                    Left => Light::left(),
                    Right => Light::right(),
                    Up => Light::up(),
                    Down => Light::down()
                },
            }
        }

        pub fn slash_mirror() -> Self {
            Bender {
                map: enum_map! {
                    Left => Light::down(),
                    Right => Light::up(),
                    Up => Light::right(),
                    Down => Light::left()
                },
            }
        }

        pub fn backslash_mirror() -> Self {
            Bender {
                map: enum_map! {
                    Left => Light::up(),
                    Right => Light::down(),
                    Up => Light::left(),
                    Down => Light::right()
                },
            }
        }

        pub fn horizontal_splitter() -> Self {
            Bender {
                map: enum_map! {
                    Left => Light::left(),
                    Right => Light::right(),
                    Up => Light::horizontal_split(),
                    Down => Light::horizontal_split(),
                },
            }
        }

        pub fn vertical_splitter() -> Self {
            Bender {
                map: enum_map! {
                    Left => Light::vertical_split(),
                    Right => Light::vertical_split(),
                    Up => Light::up(),
                    Down => Light::down(),
                },
            }
        }

        pub fn bend(&self, direction: Direction) -> &Light {
            &self.map[direction]
        }
    }

    use crate::ParseError;

    impl TryFrom<char> for Bender {
        type Error = ParseError;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '.' => Ok(Self::empty()),
                '/' => Ok(Self::slash_mirror()),
                '\\' => Ok(Self::backslash_mirror()),
                '-' => Ok(Self::horizontal_splitter()),
                '|' => Ok(Self::vertical_splitter()),
                _ => Err(ParseError()),
            }
        }
    }
}

use bender::{direction::Direction, Bender, Light};

#[derive(Debug)]
pub struct ParseError();

#[derive(Debug)]
struct Input {
    grid: Grid<Bender>,
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut benders: Vec<Bender> = vec![];
        let mut width = None;
        for (index, c) in s.chars().enumerate() {
            match c {
                '\n' => {
                    if width == None {
                        width = Some(index);
                    }
                }
                c => benders.push(c.try_into()?),
            }
        }
        Ok(Input {
            grid: Grid::with_vec(benders, width.ok_or(ParseError())?).ok_or(ParseError())?,
        })
    }
}

impl Input {
    fn solve_first_question(&self, position: Position, direction: Direction) -> usize {
        // Yes I have a good computer for this unoptimized algorithm
        let mut lights: Grid<Light> = Grid::with(self.grid.size(), |_| Light::empty());

        let mut starting_light = Light::empty();
        starting_light.map[direction] = true;

        lights.set(position, starting_light).unwrap();

        loop {
            let mut changed = false;
            for pos in self.grid.size() {
                let current_light = *lights.get(pos).unwrap();
                let current_bender = *self.grid.get(pos).unwrap();
                for light_direction in
                    current_light
                        .map
                        .iter()
                        .filter_map(|(x, on)| if *on { Some(x) } else { None })
                {
                    for new_direction in current_bender
                        .bend(light_direction)
                        .map
                        .iter()
                        .filter_map(|(x, on)| if *on { Some(x) } else { None })
                    {
                        let new_position = match pos.checked_add_signed(new_direction.vector()) {
                            Some(x) => x,
                            None => continue,
                        };

                        let change_light = match lights.get_mut(new_position) {
                            Some(x) => &mut x.map[new_direction],
                            None => continue,
                        };

                        if !*change_light {
                            *change_light = true;
                            changed = true;
                        }
                    }
                }
            }
            if !changed {
                break;
            }
        }
        lights
            .into_iter()
            .filter(|(_, light)| light.energized())
            .count()
    }

    fn solve(&self) -> usize {
        let left_side_iterator =
            (0..self.grid.size().1).map(|x| (Position(0, x), Direction::Right));
        let right_side_iterator =
            (0..self.grid.size().1).map(|x| (Position(self.grid.size().0 - 1, x), Direction::Left));
        let up_side_iterator = (0..self.grid.size().0).map(|x| (Position(x, 0), Direction::Down));
        let down_side_iterator =
            (0..self.grid.size().0).map(|x| (Position(x, self.grid.size().1 - 1), Direction::Up));

        let full_iterator = left_side_iterator
            .chain(right_side_iterator)
            .chain(up_side_iterator)
            .chain(down_side_iterator);

        full_iterator
            .collect::<Vec<_>>()
            .par_iter() // I have a good laptop, leave the unoptimized algorithm up there alone and just multithread it with a library
            .enumerate()
            .map(|(index, (position, direction))| {
                println!("{}", index);
                self.solve_first_question(*position, *direction)
            })
            .max()
            .unwrap()
    }
}

fn main() {
    let input: Input = fs::read_to_string("input").unwrap().parse().unwrap();

    dbg!(input.solve());
}
