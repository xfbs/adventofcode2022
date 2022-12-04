use std::collections::BTreeSet;

fn parse(data: &str) -> Vec<(Vec<char>, Vec<char>)> {
    data.lines()
        .map(|line| {
            let chars: Vec<_> = line.chars().collect();
            let middle = chars.len() / 2;
            (
                chars[0..middle].iter().copied().collect(),
                chars[middle..].iter().copied().collect(),
            )
        })
        .collect()
}

#[test]
fn can_parse() {
    let data = include_str!("../example.txt");
    let parsed = parse(&data);
    assert_eq!(parsed.len(), 6);
    assert_eq!(parsed[4].0, vec!['t', 't', 'g', 'J', 't', 'R', 'G', 'J']);
    assert_eq!(parsed[4].1, vec!['Q', 'c', 't', 'T', 'Z', 't', 'Z', 'T']);
}

fn priority(input: char) -> u32 {
    match input {
        'a'..='z' => input as u32 - 'a' as u32 + 1,
        'A'..='Z' => input as u32 - 'A' as u32 + 27,
        _ => unreachable!(),
    }
}

#[test]
fn can_compute_priority() {
    assert_eq!(priority('a'), 1);
    assert_eq!(priority('b'), 2);
    assert_eq!(priority('c'), 3);
    assert_eq!(priority('z'), 26);

    assert_eq!(priority('A'), 27);
    assert_eq!(priority('B'), 28);
    assert_eq!(priority('C'), 29);
    assert_eq!(priority('Z'), 52);
}

fn duplicate_item_types(left: &[char], right: &[char]) -> Vec<char> {
    let left: BTreeSet<_> = left.iter().copied().collect();
    let right: BTreeSet<_> = right.iter().copied().collect();
    left.intersection(&right).copied().collect()
}

#[test]
fn can_compute_duplicate_item_types() {
    let data = include_str!("../example.txt");
    let parsed = parse(&data);
    assert_eq!(duplicate_item_types(&parsed[0].0, &parsed[0].1), vec!['p']);
    assert_eq!(duplicate_item_types(&parsed[1].0, &parsed[1].1), vec!['L']);
    assert_eq!(duplicate_item_types(&parsed[2].0, &parsed[2].1), vec!['P']);
    assert_eq!(duplicate_item_types(&parsed[3].0, &parsed[3].1), vec!['v']);
    assert_eq!(duplicate_item_types(&parsed[4].0, &parsed[4].1), vec!['t']);
    assert_eq!(duplicate_item_types(&parsed[5].0, &parsed[5].1), vec!['s']);
}

fn solve(data: &Vec<(Vec<char>, Vec<char>)>) -> u32 {
    data.iter()
        .map(|(left, right)| {
            duplicate_item_types(&left, &right)
                .into_iter()
                .map(priority)
                .sum::<u32>()
        })
        .sum()
}

#[test]
fn can_solve() {
    let data = include_str!("../example.txt");
    let parsed = parse(&data);
    assert_eq!(solve(&parsed), 157);
}

fn solve_two(data: &[(Vec<char>, Vec<char>)]) -> u32 {
    data.chunks(3)
        .map(|chunk| {
            chunk
                .iter()
                .map(|(left, right)| {
                    left.iter()
                        .chain(right.iter())
                        .copied()
                        .collect::<BTreeSet<char>>()
                })
                .fold(None as Option<BTreeSet<char>>, |res, line| match res {
                    Some(res) => Some(res.intersection(&line).copied().collect()),
                    None => Some(line),
                })
                .unwrap()
                .into_iter()
                .map(priority)
                .sum::<u32>()
        })
        .sum()
}

#[test]
fn can_solve_two() {
    let data = include_str!("../example.txt");
    let parsed = parse(&data);
    assert_eq!(solve_two(&parsed), 70);
}

fn main() {
    let file = std::env::args().nth(1).unwrap();
    let data = std::fs::read_to_string(file).unwrap();
    let parsed = parse(&data);
    let result = solve(&parsed);
    println!("{result}");
    let result = solve_two(&parsed);
    println!("{result}");
}
