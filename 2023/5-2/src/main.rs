#[warn(missing_debug_implementations)]
use std::fs;
use std::ops::{Range, RangeInclusive};
use std::str::FromStr;

fn range_by_size(start: i64, size: i64) -> Range<i64> {
    start..(start + size)
}

fn inclusive_range_by_size(start: i64, size: i64) -> RangeInclusive<i64> {
    start..=(start + size)
}

fn move_range(range: &Range<i64>, offset: i64) -> Range<i64> {
    range.start + offset..range.end + offset
}

#[derive(Debug)]
struct ParseError {}

impl From<std::num::ParseIntError> for ParseError {
    fn from(_: std::num::ParseIntError) -> Self {
        ParseError {}
    }
}

enum RangePosition {
    NotOverlapping,
    Inner,
    Overlapping,
}

#[derive(Debug, PartialEq, Eq)]
struct RangeMap {
    range: Range<i64>,
    offset: i64,
}

impl RangeMap {
    fn split(&self, splitter: i64) -> Option<(RangeMap, RangeMap)> {
        if splitter != self.range.start && self.range.contains(&splitter) {
            Some((
                RangeMap {
                    range: self.range.start..splitter,
                    offset: self.offset,
                },
                RangeMap {
                    range: splitter..self.range.end,
                    offset: self.offset,
                },
            ))
        } else {
            None
        }
    }

    fn split_range(&self, range: &Range<i64>) -> RangeMapped {
        use RangePosition::*;
        let split_range = &self.range;
        let end_overlaps = match () {
            _ if split_range.end <= range.start => NotOverlapping,
            _ if split_range.end >= range.end => Overlapping,
            _ => Inner,
        };

        let start_overlaps = match () {
            _ if split_range.start >= range.end => NotOverlapping,
            _ if split_range.start <= range.start => Overlapping,
            _ => Inner,
        };

        match (start_overlaps, end_overlaps) {
            (Inner, Inner) => RangeMapped::Inner(
                range.start..self.range.start,
                self.range.start..self.range.end,
                self.range.end..range.end,
            ),
            (_, Inner) => RangeMapped::Left(range.start..self.range.end, self.range.end..range.end),
            (Inner, _) => {
                RangeMapped::Right(range.start..self.range.start, self.range.start..range.end)
            }
            (Overlapping, Overlapping) if self.range.contains(&range.start) => {
                RangeMapped::Full(range.start..range.end)
            }
            _ => RangeMapped::None(range.start..range.end),
        }
    }

    fn map_range(&self, range: &Range<i64>) -> RangeMapped {
        match self.split_range(range) {
            RangeMapped::None(n) => RangeMapped::None(n),
            RangeMapped::Inner(l, m, r) => RangeMapped::Inner(l, move_range(&m, self.offset), r),
            RangeMapped::Left(m, r) => RangeMapped::Left(move_range(&m, self.offset), r),
            RangeMapped::Right(l, m) => RangeMapped::Right(l, move_range(&m, self.offset)),
            RangeMapped::Full(m) => RangeMapped::Full(move_range(&m, self.offset)),
        }
    }
}

impl FromStr for RangeMap {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split_whitespace();

        let destination: i64 = splitted.next().ok_or(ParseError {})?.parse()?;
        let source: i64 = splitted.next().ok_or(ParseError {})?.parse()?;
        let size: i64 = splitted.next().ok_or(ParseError {})?.parse()?;
        Ok(RangeMap {
            range: range_by_size(source, size),
            offset: destination - source,
        })
    }
}

#[derive(Debug)]
struct Map {
    range_maps: Vec<RangeMap>,
}

impl Map {
    fn map_range(&self, range: Range<i64>) -> Vec<Range<i64>> {
        let mut unmapped_ranges = vec![range];
        let mut mapped_ranges = vec![];
        for i in &self.range_maps {
            let mut new_unmapped_ranges = vec![];
            for j in unmapped_ranges {
                match i.map_range(&j) {
                    RangeMapped::None(n) => new_unmapped_ranges.push(n),
                    RangeMapped::Inner(l, m, r) => {
                        new_unmapped_ranges.push(l);
                        new_unmapped_ranges.push(r);
                        mapped_ranges.push(m);
                    }
                    RangeMapped::Left(m, r) => {
                        new_unmapped_ranges.push(r);
                        mapped_ranges.push(m);
                    }
                    RangeMapped::Right(l, m) => {
                        new_unmapped_ranges.push(l);
                        mapped_ranges.push(m);
                    }
                    RangeMapped::Full(m) => {
                        mapped_ranges.push(m);
                    }
                }
            }
            unmapped_ranges = new_unmapped_ranges;
        }
        mapped_ranges.extend(unmapped_ranges.into_iter());
        mapped_ranges
    }
}

impl FromStr for Map {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Map { range_maps: vec![] };
        for l in s.split('\n').skip(1).filter(|x| x != &"") {
            map.range_maps.push(l.parse::<RangeMap>()?);
        }
        Ok(map)
    }
}

#[derive(Debug)]
enum RangeMapped {
    None(Range<i64>),
    Inner(Range<i64>, Range<i64>, Range<i64>),
    Left(Range<i64>, Range<i64>),
    Right(Range<i64>, Range<i64>),
    Full(Range<i64>),
}

#[derive(Debug)]
struct ParsedInput {
    seeds: Vec<Range<i64>>,
    maps: Vec<Map>,
}

impl ParsedInput {
    fn map_range(&self, range: Range<i64>) -> Vec<Range<i64>> {
        let mut ranges = vec![range];
        dbg!(&ranges);
        for i in &self.maps {
            let mut new_ranges = vec![];
            for j in ranges {
                new_ranges.append(&mut i.map_range(j));
            }
            ranges = new_ranges;
            dbg!(&ranges);
        }
        ranges
    }

    fn solve(&self) -> Option<i64> {
        Some(
            self.seeds
                .iter()
                .map(|seed| {
                    self.map_range(seed.clone())
                        .iter()
                        .filter(|x| x.start < x.end)
                        .map(|x| x.start)
                        .min()
                        .unwrap()
                })
                .min()
                .unwrap(),
        )
    }
}

impl FromStr for ParsedInput {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split("\n\n");
        let mut seeds: Vec<Range<i64>> = vec![];
        for i in splitted
            .next()
            .ok_or(ParseError {})?
            .split(": ")
            .skip(1)
            .next()
            .ok_or(ParseError {})?
            .split_whitespace()
            .collect::<Vec<_>>()
            .chunks(2)
        {
            let start: i64 = i[0].parse()?;
            let size: i64 = i[1].parse()?;
            seeds.push(start..(start + size));
        }
        let mut maps = vec![];
        for s in splitted {
            maps.push(s.parse::<Map>()?);
        }
        Ok(ParsedInput { seeds, maps })
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();

    dbg!(RangeMap {
        range: 0..10,
        offset: 100,
    }
    .map_range(&(10..20)));

    let parsed_input: ParsedInput = input.parse().unwrap();
    for map in &parsed_input.maps {
        for range_map in &map.range_maps {
            for range_map2 in &map.range_maps {
                if range_map != range_map2 {
                    println!("{:?} {:?}", range_map.range, range_map2.range);
                    match range_map.split_range(&range_map2.range) {
                        RangeMapped::None(_) => {}
                        _ => panic!(),
                    }
                }
            }
        }
    }
    dbg!(&parsed_input);

    dbg!(parsed_input.solve());
}
