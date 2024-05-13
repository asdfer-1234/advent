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
    Numeral(u32),
}

#[derive(Debug, Clone, Copy)]
enum GearCell {
    None,
    Gear,
    Number(u32, usize),
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

    let mut id: usize = 0;
    let mut gear_matrix = [[GearCell::None; WIDTH as usize]; HEIGHT as usize];

    for y in 0..HEIGHT {
        let mut numerals: Vec<u32> = Vec::new();
        for x in 0..WIDTH {
            if let Cell::Numeral(n) = matrix[y as usize][x as usize] {
                numerals.push(n);
            } else {
                if let Cell::Symbol('*') = matrix[y as usize][x as usize] {
                    gear_matrix[y as usize][x as usize] = GearCell::Gear;
                }
                if !numerals.is_empty() {
                    let mut number = 0;
                    for n in &numerals {
                        number = number * 10 + n;
                    }
                    let number = number;

                    for i in 0..numerals.len() {
                        gear_matrix[y as usize][(x - (1 + i as i32)) as usize] =
                            GearCell::Number(number, id);
                    }
                    id += 1;
                    numerals = vec![];
                }
            }
        }

        let mut number = 0;
        for n in &numerals {
            number = number * 10 + n;
        }
        let number = number;
        for i in 0..numerals.len() {
            gear_matrix[y as usize][(WIDTH - (1 + i as i32)) as usize] =
                GearCell::Number(number, id);
        }
        id += 1;
    }

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            match gear_matrix[y as usize][x as usize] {
                GearCell::None => print!("."),
                GearCell::Gear => print!("*"),
                a => {
                    dbg!(a);
                    ()
                }
            }
        }
    }

    let mut sum = 0;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if let GearCell::Gear = gear_matrix[y as usize][x as usize] {
                let mut nearby = [0; 2];
                let mut index = 0;
                let mut previous_id = None;
                let mut success = true;
                for (dx, dy) in ADJACENT {
                    let (nx, ny) = (dx + x, dy + y);
                    if position_inside(nx, ny) {
                        if let GearCell::Number(number, id) = gear_matrix[ny as usize][nx as usize]
                        {
                            if previous_id == None || previous_id.unwrap() < id {
                                previous_id = Some(id);
                                nearby[index] = number;
                                index += 1;
                                if index >= 2 {
                                    success = true;
                                    break;
                                }
                            }
                        }
                    }
                }
                if success {
                    sum += nearby[0] * nearby[1];
                }
            }
        }
    }
    println!("{}", sum);
}
