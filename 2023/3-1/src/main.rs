#![warn(missing_debug_implementations)]

use std::fs;

const WIDTH: i32 = 140;
const HEIGHT: i32 = WIDTH;

const ADJACENT: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(Debug, Clone, Copy)]
enum Cell {
    None,
    Symbol(char),
    Numeral(i32),
}

fn position_inside(x: i32, y: i32) -> bool {
    x >= 0 && x < HEIGHT && y >= 0 && y < HEIGHT
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut matrix = [[Cell::None; WIDTH as usize]; HEIGHT as usize];

    for (y, l) in input.split('\n').enumerate() {
        for (x, c) in l.chars().enumerate() {
            matrix[y][x] = match c {
                '.' => Cell::None,
                '0' => Cell::Numeral(0),
                '1' => Cell::Numeral(1),
                '2' => Cell::Numeral(2),
                '3' => Cell::Numeral(3),
                '4' => Cell::Numeral(4),
                '5' => Cell::Numeral(5),
                '6' => Cell::Numeral(6),
                '7' => Cell::Numeral(7),
                '8' => Cell::Numeral(8),
                '9' => Cell::Numeral(9),
                a => Cell::Symbol(a),
            }
        }
    }

    let mut number = None;
    let mut symbol_adjacent = false;
    let mut sum = 0;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            match matrix[y as usize][x as usize] {
                Cell::None | Cell::Symbol(_) => {
                    if let Some(n) = number {
                        if symbol_adjacent {
                            sum += n;
                        }
                    }
                    number = None;
                    symbol_adjacent = false;
                }
                Cell::Numeral(n) => {
                    number = match number {
                        Some(number) => Some(number * 10 + n),
                        None => Some(n),
                    };
                    if !symbol_adjacent {
                        for (dx, dy) in ADJACENT {
                            let (nx, ny) = (x + dx, y + dy);

                            if position_inside(nx, ny) {
                                if let Cell::Symbol(c) = matrix[ny as usize][nx as usize] {
                                    println!("{:?} {}, {}", number, nx, ny);
                                    println!("{}", c);
                                    symbol_adjacent = true;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        if let Some(n) = number {
            if symbol_adjacent {
                sum += n;
            }
        }
        number = None;
        symbol_adjacent = false;
    }

    println!("{}", sum);
}
