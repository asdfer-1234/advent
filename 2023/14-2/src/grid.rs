use std::{iter::FusedIterator, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(pub usize, pub usize);

impl Position {
    pub fn within(&self, position: Position) -> bool {
        self.0 < position.0 && self.1 < position.1
    }

    fn index_to_position(&self, index: usize) -> Option<Position> {
        let position = Position(index % self.0, index / self.0);
        match position.within(*self) {
            true => Some(position),
            false => None,
        }
    }

    fn position_to_index(&self, position: Position) -> Option<usize> {
        match position.within(*self) {
            true => Some(position.1 * self.0 + position.0),
            false => None,
        }
    }

    fn size(&self) -> usize {
        self.0 * self.1
    }

    pub fn distance(a: Self, b: Self) -> usize {
        a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl IntoIterator for Position {
    type Item = Position;

    type IntoIter = PositionIterator;

    fn into_iter(self) -> Self::IntoIter {
        PositionIterator {
            now: 0,
            until: self,
        }
    }
}

#[derive(Debug)]
pub struct PositionIterator {
    now: usize,
    until: Position,
}

impl Iterator for PositionIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let returner = self.until.index_to_position(self.now);

        self.now += 1;

        returner
    }
}

impl FusedIterator for PositionIterator {} // FusedIterator is a marker trait? cooooool.

#[derive(Debug)]
pub struct Vector(i32, i32);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Grid<T> {
    grid: Vec<T>,
    size: Position,
}

#[derive(Debug)]
pub struct OutOfGridError {
    access: Position,
    size: Position,
}

impl<T> Grid<T> {
    pub fn with<F>(size: Position, mut f: F) -> Grid<T>
    where
        F: FnMut(Position) -> T,
    {
        let mut grid = Vec::with_capacity(size.size());
        for i in size {
            grid.push(f(i))
        }
        Grid { grid, size }
    }

    pub fn with_vec(vec: Vec<T>, width: usize) -> Option<Grid<T>> {
        if vec.len() % width == 0 {
            let size = Position(width, vec.len() / width);
            Some(Self { grid: vec, size })
        } else {
            None
        }
    }

    fn index_to_position(&self, index: usize) -> Option<Position> {
        self.size.index_to_position(index)
    }

    fn position_to_index(&self, position: Position) -> Option<usize> {
        self.size.position_to_index(position)
    }

    pub fn get(&self, position: Position) -> Option<&T> {
        self.grid.get(self.position_to_index(position)?)
    }

    pub fn set(&mut self, position: Position, t: T) -> Result<(), OutOfGridError> {
        let index = self.position_to_index(position).ok_or(OutOfGridError {
            access: position,
            size: self.size,
        })?;
        self.grid[index] = t;
        Ok(())
    }

    pub fn size(&self) -> Position {
        self.size
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = (Position, T);

    type IntoIter = std::iter::Zip<PositionIterator, std::vec::IntoIter<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.size.into_iter().zip(self.grid.into_iter())
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = (Position, &'a T);

    type IntoIter = std::iter::Zip<PositionIterator, std::slice::Iter<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.size.into_iter().zip(self.grid.iter())
    }
}
