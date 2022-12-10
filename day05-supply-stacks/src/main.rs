#[derive(PartialEq, Eq, Debug, Copy, Clone)]
struct Move {
    count: usize,
    source: usize,
    target: usize,
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut moves = Vec::new();
    let mut lines = input.lines();

    for line in &mut lines {
        if line.starts_with(" 1") {
            break;
        }

        let mut chars = line.chars();
        chars.next();
        let mut chars = chars.step_by(4).enumerate();
        for (i, c) in chars {
            if c != ' ' {
                if stacks.len() <= i {
                    stacks.resize_with(i + 1, Default::default);
                }

                stacks[i].insert(0, c);
            }
        }
    }

    lines.next();

    for line in lines {
        let mut words = line.split(" ");
        words.next();
        let count: usize = words.next().unwrap().parse().unwrap();
        words.next();
        let source: usize = words.next().unwrap().parse().unwrap();
        words.next();
        let target: usize = words.next().unwrap().parse().unwrap();
        moves.push(Move {
            count,
            source,
            target,
        });
    }

    (stacks, moves)
}

#[test]
fn can_parse() {
    let example = include_str!("../example.txt");
    let (stack, moves) = parse(example);
    assert_eq!(
        moves,
        vec![
            Move {
                count: 1,
                source: 2,
                target: 1
            },
            Move {
                count: 3,
                source: 1,
                target: 3
            },
            Move {
                count: 2,
                source: 2,
                target: 1
            },
            Move {
                count: 1,
                source: 1,
                target: 2
            },
        ]
    );
    assert_eq!(stack, vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P'],]);
}

fn apply(mut stacks: Vec<Vec<char>>, moves: &[Move]) -> Vec<Vec<char>> {
    for mov in moves {
        for _ in 0..mov.count {
            let item = stacks[mov.source - 1].pop().unwrap();
            stacks[mov.target - 1].push(item);
        }
    }

    stacks
}

fn solve(stacks: &Vec<Vec<char>>) -> String {
    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .copied()
        .collect()
}

#[test]
fn can_solve() {
    let example = include_str!("../example.txt");
    let (stack, moves) = parse(example);
    let stack = apply(stack, &moves);
    let result = solve(&stack);
    assert_eq!(result, "CMZ");
}

fn main() {
    let file = std::env::args().nth(1).unwrap();
    let data = std::fs::read_to_string(file).unwrap();
    let (stack, moves) = parse(&data);
    let stack = apply(stack, &moves);
    let result = solve(&stack);
    println!("{result}");
}
