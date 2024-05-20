#![warn(missing_debug_implementations)]

use enum_map::{enum_map, Enum, EnumMap};
use std::{fs, num::ParseIntError, str::FromStr};

#[derive(Debug)]
struct ParseError {}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        ParseError {}
    }
}

#[derive(Debug, Copy, Clone, Enum, PartialEq, Eq, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (*self as usize).partial_cmp(&(*other as usize))
    }
}

impl TryFrom<char> for Card {
    type Error = ParseError;

    fn try_from(s: char) -> Result<Self, Self::Error> {
        use Card::*;
        match s {
            'A' => Ok(A),
            'K' => Ok(K),
            'Q' => Ok(Q),
            'J' => Ok(J),
            'T' => Ok(T),
            '9' => Ok(Nine),
            '8' => Ok(Eight),
            '7' => Ok(Seven),
            '6' => Ok(Six),
            '5' => Ok(Five),
            '4' => Ok(Four),
            '3' => Ok(Three),
            '2' => Ok(Two),
            _ => Err(ParseError {}),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord)]
enum RowType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl PartialOrd for RowType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (*self as usize).partial_cmp(&(*other as usize))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord)]
struct Row {
    cards: [Card; 5],
}

impl PartialOrd for Row {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let type_cmp = self.row_type().cmp(&other.row_type());
        Some(match type_cmp {
            std::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
            a => a,
        })
    }
}

impl Row {
    fn row_type(&self) -> RowType {
        let counts: EnumMap<Card, u32> = enum_map!(card => self.cards.iter().fold(0, |init, x| init + match *x == card{ true => 1, false => 0 }));
        dbg!(counts);
        let mut five = 0;
        let mut four = 0;
        let mut three = 0;
        let mut two = 0;
        let mut one = 0;
        for (_, i) in counts {
            match i {
                5 => five += 1,
                4 => four += 1,
                3 => three += 1,
                2 => two += 1,
                1 => one += 1,
                _ => (),
            }
        }
        if five == 1 {
            RowType::FiveOfAKind
        } else if four == 1 {
            RowType::FourOfAKind
        } else if three == 1 && two == 1 {
            RowType::FullHouse
        } else if three == 1 {
            RowType::ThreeOfAKind
        } else if two == 2 {
            RowType::TwoPair
        } else if two == 1 {
            RowType::OnePair
        } else {
            RowType::HighCard
        }
    }
}

impl FromStr for Row {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = [Card::A; 5];
        for (i, c) in s.chars().enumerate() {
            cards[i] = c.try_into()?;
        }
        Ok(Row { cards })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord)]
struct Item {
    row: Row,
    bid: u32,
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.row.partial_cmp(&other.row)
    }
}

impl FromStr for Item {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split_whitespace();
        let row = splitted.next().ok_or(ParseError {})?.parse()?;
        let bid = splitted.next().ok_or(ParseError {})?.parse()?;
        Ok(Item { row, bid })
    }
}

#[derive(Debug)]
struct Input {
    items: Vec<Item>,
}

impl Input {
    fn solve(&self) -> u32 {
        let mut items = self.items.clone();
        items.sort();
        let mut summation = 0;
        for (rank, score) in items
            .iter()
            .enumerate()
            .map(|(index, x)| ((index + 1) as u32, x.bid))
        {
            summation += rank * score;
        }
        summation
    }
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input {
            items: s
                .split('\n')
                .filter(|x| !x.is_empty())
                .filter_map(|x| x.parse::<Item>().ok())
                .collect(),
        })
    }
}

fn main() {
    let input = fs::read_to_string("input")
        .unwrap()
        .parse::<Input>()
        .unwrap();

    dbg!(input.solve());
}
