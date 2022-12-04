use std::fs::File;

fn parse(data: &str) -> Vec<Vec<u32>> {
    let mut out = vec![vec![]];
    for line in data.lines() {
        if line.len() == 0 {
            out.push(vec![]);
        } else {
            out.last_mut().unwrap().push(line.parse().unwrap());
        }
    }
    out
}

fn solve(data: &Vec<Vec<u32>>) -> (usize, u32) {
    data.iter()
        .map(|data| data.iter().sum())
        .enumerate()
        .max_by_key(|(index, sum)| *sum)
        .unwrap()
}

fn solve_part_two(data: &Vec<Vec<u32>>) -> (Vec<usize>, u32) {
    let mut data: Vec<(usize, u32)> = data
        .iter()
        .map(|data| data.iter().sum())
        .enumerate()
        .collect();
    data.sort_by_key(|data| data.1);
    data.reverse();
    (
        data.iter().take(3).map(|e| e.0).collect(),
        data.iter().take(3).map(|e| e.1).sum(),
    )
}

#[test]
fn can_solve() {
    let data = include_str!("../example.txt");
    assert_eq!(solve(&parse(data)), (3, 24000));
}

#[test]
fn can_solve_part_two() {
    let data = include_str!("../example.txt");
    assert_eq!(solve_part_two(&parse(data)), (vec![3, 2, 4], 45000));
}

fn main() {
    let file = std::env::args().nth(1).unwrap();
    let data = std::fs::read_to_string(file).unwrap();
    let parsed = parse(&data);
    let (index, sum) = solve(&parsed);
    println!("index {index} sum {sum}");
    let (indexes, sum) = solve_part_two(&parsed);
    println!("indexes {indexes:?} sum {sum}");
}
