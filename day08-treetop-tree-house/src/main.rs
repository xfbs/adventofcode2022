use std::collections::BTreeSet;

fn visible_left(heights: &[u8]) -> Vec<usize> {
    let mut trees = vec![0];
    heights
        .into_iter()
        .enumerate()
        .fold(heights[0], |acc, (index, item)| {
            if *item > acc {
                trees.push(index);
                *item
            } else {
                acc
            }
        });
    trees
}

fn visible_right(heights: &[u8]) -> Vec<usize> {
    let mut trees = vec![heights.len() - 1];
    heights
        .into_iter()
        .enumerate()
        .rev()
        .fold(*heights.last().unwrap(), |acc, (index, item)| {
            if *item > acc {
                trees.push(index);
                *item
            } else {
                acc
            }
        });
    trees
}

fn visible_sides(heights: &Vec<Vec<u8>>) -> BTreeSet<(usize, usize)> {
    heights
        .iter()
        .enumerate()
        .map(|(y, line)| {
            visible_left(&line)
                .into_iter()
                .chain(visible_right(&line).into_iter())
                .map(move |x| (x, y))
        })
        .flatten()
        .collect()
}

fn transpose(data: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    (0..data[0].len())
        .map(|offset| data.iter().map(|line| line[offset]).collect())
        .collect()
}

fn visible(heights: &Vec<Vec<u8>>) -> BTreeSet<(usize, usize)> {
    let mut result = visible_sides(heights);
    for (y, x) in visible_sides(&transpose(heights)) {
        result.insert((x, y));
    }
    result
}

fn parse(data: &str) -> Vec<Vec<u8>> {
    data.lines()
        .map(|line| {
            line.split("")
                .filter(|s| !s.is_empty())
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

fn solve(data: &Vec<Vec<u8>>) -> usize {
    visible(data).len()
}

fn scenic_score_right(row: &[u8], pos: usize) -> u64 {
    assert!(pos <= row.len());
    let trees = row
        .into_iter()
        .skip(pos + 1)
        .take_while(|e| **e < row[pos])
        .count();
    let trees = (trees + 1).min(row.len() - pos - 1);
    trees as u64
}

#[test]
fn can_compute_scenic_score_right() {
    assert_eq!(scenic_score_right(&[2, 5, 5, 1, 2], 0), 1);
    assert_eq!(scenic_score_right(&[2, 5, 5, 1, 2], 1), 1);
    assert_eq!(scenic_score_right(&[2, 5, 5, 1, 2], 2), 2);
    assert_eq!(scenic_score_right(&[2, 5, 5, 1, 2], 3), 1);
    assert_eq!(scenic_score_right(&[2, 5, 5, 1, 2], 4), 0);
    assert_eq!(scenic_score_right(&[3, 3, 5, 4, 9], 0), 1);
    assert_eq!(scenic_score_right(&[3, 3, 5, 4, 9], 1), 1);
    assert_eq!(scenic_score_right(&[3, 3, 5, 4, 9], 2), 2);
    assert_eq!(scenic_score_right(&[3, 3, 5, 4, 9], 3), 1);
    assert_eq!(scenic_score_right(&[3, 3, 5, 4, 9], 4), 0);
}

fn scenic_score_left(row: &[u8], pos: usize) -> u64 {
    assert!(pos <= row.len());
    let trees = row
        .into_iter()
        .rev()
        .skip(row.len() - pos)
        .take_while(|e| **e < row[pos])
        .count();
    (trees + 1).min(pos) as u64
}

#[test]
fn can_compute_scenic_score_left() {
    assert_eq!(scenic_score_left(&[2, 5, 5, 1, 2], 0), 0);
    assert_eq!(scenic_score_left(&[2, 5, 5, 1, 2], 1), 1);
    assert_eq!(scenic_score_left(&[2, 5, 5, 1, 2], 2), 1);
    assert_eq!(scenic_score_left(&[2, 5, 5, 1, 2], 3), 1);
    assert_eq!(scenic_score_left(&[2, 5, 5, 1, 2], 4), 2);
    assert_eq!(scenic_score_left(&[3, 3, 5, 4, 9], 0), 0);
    assert_eq!(scenic_score_left(&[3, 3, 5, 4, 9], 1), 1);
    assert_eq!(scenic_score_left(&[3, 3, 5, 4, 9], 2), 2);
    assert_eq!(scenic_score_left(&[3, 3, 5, 4, 9], 3), 1);
    assert_eq!(scenic_score_left(&[3, 3, 5, 4, 9], 4), 4);
}

fn scenic_score(row: &[u8], col: &[u8], x: usize, y: usize) -> u64 {
    assert!(x < row.len());
    assert!(y < col.len());
    assert_eq!(row[x], col[y]);

    [
        scenic_score_left(row, x),
        scenic_score_right(row, x),
        scenic_score_left(col, y),
        scenic_score_right(col, y),
    ]
    .into_iter()
    .product()
}

#[test]
fn can_compute_scenic_score() {
    assert_eq!(scenic_score(&[2, 5, 5, 1, 2], &[3, 5, 3, 5, 3], 2, 1), 4);
    assert_eq!(scenic_score(&[3, 3, 5, 4, 9], &[3, 5, 3, 5, 3], 2, 3), 8);
}

fn solve_two(data: &Vec<Vec<u8>>) -> u64 {
    let transposed = transpose(data);
    let (_x, _y, score) = data
        .iter()
        .enumerate()
        .map(|(y, row)| {
            transposed
                .iter()
                .enumerate()
                .map(|(x, col)| (x, y, scenic_score(&row, &col, x, y)))
                .max_by_key(|(_, _, score)| *score)
                .unwrap()
        })
        .max_by_key(|(_, _, score)| *score)
        .unwrap();
    score
}

#[test]
fn can_check_visible_left() {
    assert_eq!(visible_left(&[3, 0, 3, 7, 3]), vec![0, 3]);
    assert_eq!(visible_left(&[3, 3, 5, 4, 9]), vec![0, 2, 4]);
    assert_eq!(visible_left(&[3, 5, 3, 9, 0]), vec![0, 1, 3]);
}

#[test]
fn can_check_visible_right() {
    assert_eq!(visible_right(&[3, 0, 3, 7, 3]), vec![4, 3]);
    assert_eq!(visible_right(&[3, 3, 5, 4, 9]), vec![4]);
    assert_eq!(visible_right(&[3, 5, 3, 9, 0]), vec![4, 3]);
}

#[test]
fn can_parse() {
    let data = parse(include_str!("../example.txt"));
    assert_eq!(data[0], vec![3, 0, 3, 7, 3]);
    assert_eq!(data[1], vec![2, 5, 5, 1, 2]);
    assert_eq!(data[2], vec![6, 5, 3, 3, 2]);
}

#[test]
fn can_check_visible() {
    let data = parse(include_str!("../example.txt"));
    let vis = visible(&data);
    assert_eq!(vis.len(), 21);
}

#[test]
fn can_transpose() {
    assert_eq!(
        transpose(&vec![vec![1, 2], vec![3, 4]]),
        vec![vec![1, 3], vec![2, 4]]
    );
    assert_eq!(
        transpose(&vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![0, 0, 0]
        ]),
        vec![vec![1, 4, 7, 0], vec![2, 5, 8, 0], vec![3, 6, 9, 0]]
    );
}

#[test]
fn can_solve_two() {
    let data = parse(include_str!("../example.txt"));
    let score = solve_two(&data);
    assert_eq!(score, 8);
}

fn main() {
    let file = std::env::args().nth(1).unwrap();
    let data = std::fs::read_to_string(file).unwrap();
    let data = parse(&data);
    let result = solve(&data);
    println!("{result}");
    let score = solve_two(&data);
    println!("{score}");
}
