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

#[derive(Debug, Enum, Clone, Copy, PartialEq, Eq)]
enum NodeType {
    Starting,
    Ending,
    None,
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
struct Node {
    node_index: usize,
    destinations: EnumMap<Turn, usize>,
    node_type: NodeType,
}

impl FromStr for Node {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (node_string, node_destination_string) = s.split_once(" = ").ok_or(ParseError {})?;
        let (left_node_string, right_node_string) = node_destination_string[1..9]
            .split_once(", ")
            .ok_or(ParseError {})?;

        let (node_index, left_node_index, right_node_index) = (
            node_string_to_index(node_string).ok_or(ParseError {})?,
            node_string_to_index(left_node_string).ok_or(ParseError {})?,
            node_string_to_index(right_node_string).ok_or(ParseError {})?,
        );

        Ok(Self {
            node_index,
            destinations: enum_map! {
                Turn::Left => left_node_index,
                Turn::Right => right_node_index,
            },
            node_type: if node_string.ends_with('A') {
                NodeType::Starting
            } else if node_string.ends_with('Z') {
                NodeType::Ending
            } else {
                NodeType::None
            },
        })
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            node_index: 0,
            destinations: enum_map! { _ =>  0},
            node_type: NodeType::None,
        }
    }
}

#[derive(Debug)]
struct Input {
    turns: Vec<Turn>,
    graph: [Node; 26 * 26 * 26],
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (turn_string, graph_string) = s.trim().split_once("\n\n").unwrap();

        let mut new = Input {
            turns: vec![],
            graph: core::array::from_fn(|_| Node::default()),
        };

        for c in turn_string.chars() {
            new.turns.push(TryInto::<Turn>::try_into(c)?);
        }

        for l in graph_string.split('\n') {
            let node: Node = l.parse()?;
            let index = node.node_index;
            new.graph[index] = node;
        }

        Ok(new)
    }
}

impl Input {
    fn solve(&self) -> u128 {
        self.graph
            .iter()
            .filter(|x| x.node_type == NodeType::Starting)
            .map(|n| {
                let mut node = n;
                let mut move_count: u128 = 0;
                while node.node_type != NodeType::Ending {
                    node = &self.graph
                        [node.destinations[self.turns[(move_count as usize) % self.turns.len()]]];
                    move_count += 1;
                }
                dbg!(&move_count);
                move_count
            })
            .fold(1, |init, x| num::integer::lcm(init, x))
    }
}

fn main() {
    let input: Input = fs::read_to_string("input").unwrap().parse().unwrap();
    dbg!(&input);
    dbg!(input.solve());
}
