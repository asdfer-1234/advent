#![warn(missing_debug_implementations)]

use enum_map::enum_map;
use enum_map::Enum;
use enum_map::EnumMap;
use std::cmp::max;
use std::fs;

#[derive(Copy, Clone, Debug, Enum)]
enum Color {
    Red,
    Green,
    Blue,
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

fn parse_ball_set(s: &str) -> EnumMap<Color, u32> {
    let mut results: EnumMap<Color, u32> = enum_map!(_ => 0);
    for balls in s.split(", ") {
        let (color, value) = parse_ball_color(balls);
        results[color] = value;
    }
    results
}

fn parse_ball_set_set(s: &str) -> u32 {
    let mut results: EnumMap<Color, u32> = enum_map!(_ => 0);
    for set in s.split("; ") {
        let set_result = parse_ball_set(set);
        for (color, value) in set_result {
            results[color] = max(results[color], value);
        }
    }

    println!("{}", results.iter().fold(1, |x, (_, y)| x * y));
    results.iter().fold(1, |x, (_, y)| x * y)
}

fn parse_line(s: &str) -> (u32, u32) {
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
        let (_, value) = parse_line(l);
        sum += value;
    }
    println!("{}", sum);
}
