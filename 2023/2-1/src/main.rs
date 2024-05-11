#![warn(missing_debug_implementations)]

use std::fs;

#[derive(Copy, Clone, Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn max_in_bag(&self) -> u32 {
        match self {
            Color::Red => 12,
            Color::Green => 13,
            Color::Blue => 14,
        }
    }
}

fn parse_id(s: &str) -> u32 {
    s[5..].parse().unwrap()
}

fn parse_color(s: &str) -> Color {
    match s {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        _ => panic!(),
    }
}

fn parse_ball_color(s: &str) -> (Color, u32) {
    let splitted: Vec<&str> = s.split(' ').collect();
    (parse_color(splitted[1]), splitted[0].parse().unwrap())
}

fn parse_ball_set(s: &str) -> bool {
    println!("{}", s);
    s.split(", ").find(|x| {
        let (color, count) = parse_ball_color(x);
        count > color.max_in_bag()
    }) == None
}

fn parse_ball_set_set(s: &str) -> bool {
    s.split("; ").find(|x| !parse_ball_set(x)) == None
}

fn parse_line(s: &str) -> (u32, bool) {
    let splitted: Vec<&str> = s.split(": ").collect();
    (parse_id(splitted[0]), parse_ball_set_set(splitted[1]))
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut sum = 0;
    for l in input.split("\n") {
        if l == "" {
            continue;
        }
        let (id, valid) = parse_line(l);
        if valid {
            println!("id: {}", id);
            sum += id;
        }
    }
    println!("{}", sum);
}
