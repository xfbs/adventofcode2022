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

fn main() {
    let file = std::env::args().nth(1).unwrap();
    let data = std::fs::read_to_string(file).unwrap();
    let data = parse(&data);
    let result = solve(&data);
    println!("{result}");
}
