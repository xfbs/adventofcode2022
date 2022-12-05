#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Range(u32, u32);

impl Range {
    fn contains(&self, number: u32) -> bool {
        self.0 <= number && number <= self.1
    }

    fn fully_contains(&self, other: &Range) -> bool {
        self.contains(other.0) && self.contains(other.1)
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.contains(other.0) || self.contains(other.1)
    }
}

fn parse_range(range: &str) -> Range {
    let mut numbers = range.split("-").map(|num| num.parse().unwrap());
    Range(numbers.next().unwrap(), numbers.next().unwrap())
}

fn parse_line(line: &str) -> (Range, Range) {
    let mut ranges = line.split(",").map(parse_range);
    (ranges.next().unwrap(), ranges.next().unwrap())
}

fn parse(data: &str) -> Vec<(Range, Range)> {
    data.lines().map(parse_line).collect()
}

#[test]
fn can_parse() {
    let data = include_str!("../example.txt");
    let parsed = parse(data);
    assert_eq!(parsed[0], (Range(2, 4), Range(6, 8)));
    assert_eq!(parsed[1], (Range(2, 3), Range(4, 5)));
}

fn either_fully_contains((left, right): &(Range, Range)) -> bool {
    left.fully_contains(&right) || right.fully_contains(&left)
}

fn either_overlaps((left, right): &(Range, Range)) -> bool {
    left.overlaps(&right) || right.overlaps(&left)
}

fn solve(data: &[(Range, Range)]) -> usize {
    data.iter().copied().filter(either_fully_contains).count()
}

fn solve_two(data: &[(Range, Range)]) -> usize {
    data.iter().copied().filter(either_overlaps).count()
}

#[test]
fn can_solve() {
    let data = include_str!("../example.txt");
    let parsed = parse(data);
    let result = solve(&parsed);
    assert_eq!(result, 2);
}

#[test]
fn can_solve_two() {
    let data = include_str!("../example.txt");
    let parsed = parse(data);
    let result = solve_two(&parsed);
    assert_eq!(result, 4);
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
