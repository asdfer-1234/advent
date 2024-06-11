use std::convert::Infallible;
use std::{fmt::Debug, str::FromStr};

#[derive(Debug)]
struct ParseError();

impl From<std::num::ParseIntError> for ParseError {
    fn from(_: std::num::ParseIntError) -> Self {
        ParseError()
    }
}

impl From<std::array::TryFromSliceError> for ParseError {
    fn from(_: std::array::TryFromSliceError) -> Self {
        ParseError()
    }
}

impl From<Infallible> for ParseError {
    fn from(_: Infallible) -> Self {
        ParseError()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum OperationType {
    Insert(u32),
    Remove,
}

impl FromStr for OperationType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (operator, index_string) = s.split_at(1);
        match operator {
            "=" => Ok(OperationType::Insert(index_string.parse()?)),
            "-" => Ok(OperationType::Remove),
            _ => Err(ParseError()),
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
struct Index {
    raw: String,
}

impl Index {
    fn hash(&self) -> u8 {
        self.raw
            .as_bytes()
            .iter()
            .fold(0, |init, x| init.wrapping_add(*x).wrapping_mul(17))
    }
}

impl Debug for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

impl FromStr for Index {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { raw: s.to_string() })
    }
}

#[derive(Debug, Clone)]
struct Operation {
    index: Index,
    operation_type: OperationType,
}

impl Operation {}

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for (i, c) in s.chars().enumerate() {
            match c {
                '=' | '-' => {
                    let (index_string, operator) = s.split_at(i);
                    return Ok(Self {
                        index: index_string.parse()?,
                        operation_type: operator.parse()?,
                    });
                }
                _ => continue,
            }
        }
        Err(ParseError())
    }
}

#[derive(Debug, Clone)]
struct Input {
    operations: Vec<Operation>,
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            operations: s.split(',').filter_map(|x| x.parse().ok()).collect(),
        })
    }
}

#[derive(Eq, PartialEq)]
struct Lens {
    index: Index,
    focal_length: u32,
}

impl Debug for Lens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?} {}]", self.index, self.focal_length)
    }
}

impl Input {
    fn solve(&self) -> u32 {
        let mut lenses: [Vec<Lens>; 256] = std::array::from_fn(|_| vec![]);

        for operation in &self.operations {
            let vec = &mut lenses[operation.index.hash() as usize];
            match operation.operation_type {
                OperationType::Insert(x) => {
                    let mut replaced = false;
                    for i in &mut *vec {
                        if i.index == operation.index {
                            i.focal_length = x;
                            replaced = true;
                            break;
                        }
                    }

                    if !replaced {
                        vec.push(Lens {
                            index: operation.index.clone(),
                            focal_length: x,
                        });
                    }
                }
                OperationType::Remove => {
                    for (index, i) in vec.iter().enumerate() {
                        if i.index == operation.index {
                            vec.remove(index);
                            break;
                        }
                    }
                }
            }
            dbg!(operation.index.hash(), vec);
        }

        let mut total = 0;
        for (box_index, vec) in lenses.iter().enumerate() {
            for (index, lens) in vec.iter().enumerate() {
                total += lens.focal_length * (index as u32 + 1) * (box_index as u32 + 1);
            }
        }
        total
    }
}

fn main() {
    let input: Input = include_str!("../input").parse().unwrap();
    dbg!(input.solve());
}
