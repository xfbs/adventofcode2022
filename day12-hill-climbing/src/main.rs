use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Height(u8),
    Start,
    End,
}

impl Square {
    fn elevation(&self) -> u8 {
        match self {
            Self::Height(height) => *height,
            Self::Start => 0,
            Self::End => 25,
        }
    }

    fn can_move(&self, other: &Square) -> bool {
        (self.elevation() + 1) >= other.elevation()
    }
}

#[test]
fn can_check_move() {
    use Square::*;
    assert_eq!(Height(5).can_move(&Height(6)), true);
    assert_eq!(Height(5).can_move(&Height(7)), false);
    assert_eq!(Height(5).can_move(&Height(5)), true);
    assert_eq!(Height(5).can_move(&Height(4)), true);
    assert_eq!(Height(5).can_move(&Height(3)), true);
    assert_eq!(Height(5).can_move(&Height(2)), true);
    assert_eq!(Height(5).can_move(&Height(1)), true);
    assert_eq!(Height(5).can_move(&Height(0)), true);
}

impl FromStr for Square {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let single = input.chars().next().ok_or("Empty input")?;
        match single {
            'S' => Ok(Square::Start),
            'E' => Ok(Square::End),
            'a'..='z' => Ok(Square::Height(single as u8 - 'a' as u8)),
            _ => Err("Unknown char"),
        }
    }
}

fn parse(data: &str) -> BTreeMap<(usize, usize), Square> {
    data.lines()
        .enumerate()
        .map(|(x, line)| {
            line.split("")
                .filter(|c| !c.is_empty())
                .enumerate()
                .map(move |(y, c)| ((x, y), c.parse().unwrap()))
        })
        .flatten()
        .collect()
}

#[test]
fn can_parse() {
    let data = parse(include_str!("../example.txt"));
    assert_eq!(data[&(0, 0)], Square::Start);
    assert_eq!(data[&(0, 1)], Square::Height(0));
    assert_eq!(data[&(0, 2)], Square::Height(1));
    assert_eq!(data[&(4, 0)], Square::Height(0));
    assert_eq!(data[&(4, 1)], Square::Height(1));
    assert_eq!(data[&(4, 2)], Square::Height(3));
    assert_eq!(data[&(4, 3)], Square::Height(4));
    assert_eq!(data[&(4, 4)], Square::Height(5));
}

fn directions(coord: (usize, usize)) -> Vec<(usize, usize)> {
    let mut out = vec![];

    match coord {
        (0, y) => {
            out.push((1, y));
        }
        (x, y) => {
            out.push((x - 1, y));
            out.push((x + 1, y));
        }
    }

    match coord {
        (x, 0) => {
            out.push((x, 1));
        }
        (x, y) => {
            out.push((x, y - 1));
            out.push((x, y + 1));
        }
    }

    out
}

#[test]
fn can_get_directions() {
    assert_eq!(directions((5, 5)), vec![(4, 5), (6, 5), (5, 4), (5, 6)]);
    assert_eq!(directions((0, 5)), vec![(1, 5), (0, 4), (0, 6)]);
    assert_eq!(directions((5, 0)), vec![(4, 0), (6, 0), (5, 1)]);
    assert_eq!(directions((0, 0)), vec![(1, 0), (0, 1)]);
}

fn shortest_distance(
    data: &BTreeMap<(usize, usize), Square>,
    start: (usize, usize),
    end: (usize, usize),
) -> u64 {
    let mut distances: BTreeMap<(usize, usize), u64> = BTreeMap::new();
    let mut current = BTreeSet::new();

    distances.insert(start, 0);
    current.insert(start);

    loop {
        if let Some(length) = distances.get(&end) {
            return *length;
        }

        assert!(!current.is_empty());

        for entry in std::mem::take(&mut current) {
            let square = data.get(&entry).unwrap();
            let next_distance = distances.get(&entry).unwrap() + 1;
            let directions = directions(entry)
                .into_iter()
                .filter_map(|t| data.get(&t).map(|square| (t, square)));
            for (next_pos, next_square) in directions {
                if !square.can_move(next_square) {
                    continue;
                }

                if distances.get(&next_pos) != Some(&next_distance) {
                    distances.insert(next_pos, next_distance);
                    current.insert(next_pos);
                }
            }
        }
    }
}

fn solve(data: &BTreeMap<(usize, usize), Square>) -> u64 {
    let start = *data
        .iter()
        .find(|(_, value)| **value == Square::Start)
        .unwrap()
        .0;
    let end = *data
        .iter()
        .find(|(_, value)| **value == Square::End)
        .unwrap()
        .0;

    shortest_distance(data, start, end)
}

fn solve_two(data: &BTreeMap<(usize, usize), Square>) -> u64 {
    let end = *data
        .iter()
        .find(|(_, value)| **value == Square::End)
        .unwrap()
        .0;

    data.iter()
        .filter(|(_, value)| **value == Square::Height(0))
        .map(|(start, _)| shortest_distance(&data, *start, end))
        .min()
        .unwrap()
}

#[test]
fn can_solve() {
    let data = parse(include_str!("../example.txt"));
    assert_eq!(solve(&data), 31);
}

fn main() {
    let file = std::env::args().nth(1).unwrap();
    let data = std::fs::read_to_string(file).unwrap();
    let data = parse(&data);
    let result = solve(&data);
    println!("{result}");
    let result = solve_two(&data);
    println!("{result}");
}
