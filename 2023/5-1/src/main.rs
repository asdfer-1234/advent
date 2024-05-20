#[warn(missing_debug_implementations)]
use std::fs;
use std::{iter, ops::Range, str::FromStr};

#[derive(Debug)]
struct ParseError {}

impl From<std::num::ParseIntError> for ParseError {
    fn from(value: std::num::ParseIntError) -> Self {
        ParseError {}
    }
}

#[derive(Debug)]
struct RangeMap {
    source: u64,
    destination: u64,
    size: u64,
}

impl RangeMap {
    fn source_range(&self) -> Range<u64> {
        self.source..(self.source + self.size)
    }

    fn map(&self, from: u64) -> Option<u64> {
        if self.source_range().contains(&from) {
            Some(from + self.destination - self.source)
        } else {
            None
        }
    }
}

impl FromStr for RangeMap {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split_whitespace();
        Ok(RangeMap {
            destination: splitted.next().ok_or(ParseError {})?.parse()?,
            source: splitted.next().ok_or(ParseError {})?.parse()?,
            size: splitted.next().ok_or(ParseError {})?.parse()?,
        })
    }
}

#[derive(Debug)]
struct ParsedInput {
    seeds: Vec<u64>,
    range_maps_maps: Vec<Vec<RangeMap>>,
}

impl ParsedInput {
    fn map(&self, from: u64) -> u64 {
        let mut current = from;
        print!("{}", from);
        for i in &self.range_maps_maps {
            for j in i {
                match j.map(current) {
                    Some(x) => {
                        current = x;
                        break;
                    }
                    None => continue,
                }
            }
            print!(" -> {}", current);
        }
        println!();
        current
    }

    fn solve(&self) -> u64 {
        self.seeds.iter().map(|x| self.map(*x)).min().unwrap()
    }
}

impl FromStr for ParsedInput {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split("\n\n");
        let mut seeds: Vec<u64> = vec![];
        for i in splitted
            .next()
            .ok_or(ParseError {})?
            .split(": ")
            .skip(1)
            .next()
            .ok_or(ParseError {})?
            .split_whitespace()
        {
            seeds.push(i.parse()?)
        }
        let mut range_maps_maps: Vec<Vec<RangeMap>> = vec![];
        for i in splitted {
            let mut range_maps: Vec<RangeMap> = vec![];
            for j in i
                .split(":")
                .skip(1)
                .next()
                .ok_or(ParseError {})?
                .split("\n")
                .filter(|x| x != &"")
            {
                range_maps.push(j.parse()?);
            }
            range_maps_maps.push(range_maps);
        }
        Ok(ParsedInput {
            seeds,
            range_maps_maps,
        })
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let parsed_input: ParsedInput = input.parse().unwrap();
    dbg!(parsed_input.solve());
}
