fn unique<T: PartialEq>(slice: &[T]) -> bool {
    !(1..slice.len()).any(|i| slice[i..].contains(&slice[i - 1]))
}

#[test]
fn can_unique() {
    assert_eq!(unique(&[] as &[char]), true);
    assert_eq!(unique(&['a', 'b']), true);
    assert_eq!(unique(&['a', 'a']), false);
    assert_eq!(unique(&['a', 'c', 'a']), false);
}

fn index_distinct(input: &str, window: usize) -> usize {
    input
        .chars()
        .collect::<Vec<_>>()
        .windows(window)
        .position(|s| unique(&s))
        .unwrap()
        + window
}

fn solve(input: &str) -> usize {
    index_distinct(input, 4)
}

#[test]
fn can_solve() {
    assert_eq!(solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(solve("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
}

fn solve_two(input: &str) -> usize {
    index_distinct(input, 14)
}

#[test]
fn can_solve_two() {
    assert_eq!(solve_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(solve_two("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(solve_two("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    assert_eq!(solve_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(solve_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
}

fn main() {
    let file = std::env::args().nth(1).unwrap();
    let data = std::fs::read_to_string(file).unwrap();
    let result = solve(&data);
    println!("{result}");
    let result = solve_two(&data);
    println!("{result}");
}
