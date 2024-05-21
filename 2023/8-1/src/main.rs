#![warn(missing_debug_implementations)]

use enum_map::{enum_map, Enum, EnumMap};
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct ParseError {}

fn char_to_index(c: char) -> Option<usize> {
    let index = (c as usize).wrapping_sub('A' as usize);
    if (0..26).contains(&index) {
        Some(index)
    } else {
        None
    }
}

fn node_string_to_index(node_string: &str) -> Option<usize> {
    if node_string.len() != 3 {
        return None;
    }

    let mut index = 0;

    for c in node_string.chars() {
        index *= 26;
        index += char_to_index(c)?;
    }

    Some(index)
}

#[derive(Debug, Enum, Clone, Copy)]
enum Turn {
    Left,
    Right,
}

impl TryFrom<char> for Turn {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Turn::Left),
            'R' => Ok(Turn::Right),
            _ => Err(ParseError {}),
        }
    }
}

#[derive(Debug)]
struct Input {
    turns: Vec<Turn>,
    graph: [EnumMap<Turn, usize>; 26 * 26 * 26],
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (turn_string, graph_string) = s.trim().split_once("\n\n").unwrap();

        let mut new = Input {
            turns: vec![],
            graph: [enum_map! {_ => 0}; 26 * 26 * 26],
        };

        for c in turn_string.chars() {
            new.turns.push(TryInto::<Turn>::try_into(c)?);
        }

        for l in graph_string.split('\n') {
            let (node_string, node_destination_string) =
                l.split_once(" = ").ok_or(ParseError {})?;
            let (left_node_string, right_node_string) = node_destination_string[1..9]
                .split_once(", ")
                .ok_or(ParseError {})?;

            dbg!(node_string, left_node_string, right_node_string);

            let (node_index, left_node_index, right_node_index) = (
                node_string_to_index(node_string).ok_or(ParseError {})?,
                node_string_to_index(left_node_string).ok_or(ParseError {})?,
                node_string_to_index(right_node_string).ok_or(ParseError {})?,
            );

            new.graph[node_index][Turn::Left] = left_node_index;
            new.graph[node_index][Turn::Right] = right_node_index;
        }

        Ok(new)
    }
}

impl Input {
    fn solve(&self) -> u32 {
        let mut index = 0;
        let mut move_count = 0;
        while index != 26 * 26 * 26 - 1 {
            index = self.graph[index][self.turns[(move_count as usize) % self.turns.len()]];
            move_count += 1;
        }
        move_count
    }
}

fn main() {
    let input: Input = fs::read_to_string("input").unwrap().parse().unwrap();
    dbg!(input.solve());
}
