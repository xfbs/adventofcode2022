#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Target {
    Lose,
    Draw,
    Win,
}

impl Hand {
    fn parse(from: char) -> Self {
        use Hand::*;
        match from {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => unreachable!(),
        }
    }

    fn beats(&self) -> Self {
        use Hand::*;
        match self {
            Scissors => Paper,
            Rock => Scissors,
            Paper => Rock,
        }
    }

    fn loses(&self) -> Self {
        use Hand::*;
        match self {
            Scissors => Rock,
            Rock => Paper,
            Paper => Scissors,
        }
    }
}

fn parse(data: &str) -> Vec<(Hand, Hand)> {
    data.lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            (Hand::parse(chars[0]), Hand::parse(chars[2]))
        })
        .collect()
}

fn fix_second((left, right): (Hand, Hand)) -> (Hand, Target) {
    use Hand::*;
    use Target::*;
    let target = match right {
        Rock => Lose,
        Paper => Draw,
        Scissors => Win,
    };
    (left, target)
}

fn find_hand((left, target): (Hand, Target)) -> (Hand, Hand) {
    use Target::*;
    match target {
        Draw => (left, left),
        Win => (left, left.loses()),
        Lose => (left, left.beats()),
    }
}

fn solve_part_two(data: &[(Hand, Hand)]) -> Vec<(Hand, Hand)> {
    data.iter()
        .copied()
        .map(fix_second)
        .map(find_hand)
        .collect()
}

fn solve(data: &[(Hand, Hand)]) -> u32 {
    data.iter()
        .map(|(left, right)| {
            use Hand::*;
            let points = match right {
                Rock => 1,
                Paper => 2,
                Scissors => 3,
            };
            let score = match left {
                _ if right.beats() == *left => 6,
                _ if left == right => 3,
                _ => 0,
            };
            points + score
        })
        .sum()
}

#[test]
fn can_parse() {
    use Hand::*;
    let data = include_str!("../example.txt");
    let parsed = parse(&data);
    assert_eq!(
        parsed,
        vec![(Rock, Paper), (Paper, Rock), (Scissors, Scissors)]
    );
}

#[test]
fn can_solve() {
    let data = include_str!("../example.txt");
    assert_eq!(solve(&parse(&data)), 15);
}

#[test]
fn can_solve_part_two() {
    let data = include_str!("../example.txt");
    assert_eq!(solve(&solve_part_two(&parse(&data))), 12);
}

fn main() {
    let file = std::env::args().nth(1).unwrap();
    let data = std::fs::read_to_string(file).unwrap();
    let parsed = parse(&data);
    let result = solve(&parsed);
    println!("{result}");
    let result = solve(&solve_part_two(&parsed));
    println!("{result}");
}
