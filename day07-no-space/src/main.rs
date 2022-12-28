use std::collections::BTreeMap;

fn parse_line(line: &str) -> Line {
    let mut words = line.split(" ");
    let first = words.next().unwrap();
    match first {
        "$" => match words.next().unwrap() {
            "ls" => Line::ListFiles,
            "cd" => Line::ChangeDirectory(words.next().unwrap().into()),
            _ => unreachable!(),
        },
        "dir" => Line::Directory(words.next().unwrap().into()),
        size => Line::File(words.next().unwrap().into(), size.parse().unwrap()),
    }
}

fn parse(input: &str) -> Vec<Line> {
    input.lines().map(parse_line).collect()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Line {
    ChangeDirectory(String),
    ListFiles,
    Directory(String),
    File(String, u64),
}

#[derive(Debug, Clone, Default)]
pub struct Dir {
    entries: BTreeMap<String, Node>,
    total: u64,
}

#[derive(Debug, Clone)]
pub enum Node {
    Dir(Dir),
    File(u64),
}

impl Default for Node {
    fn default() -> Self {
        Node::Dir(Dir::default())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Solver {
    position: Vec<String>,
    root: Dir,
}

fn add_file(dir: &mut Dir, path: &[String], name: &str, size: u64) -> u64 {
    if let Some(next) = path.first() {
        let next = match dir.entries.get_mut(next) {
            Some(Node::Dir(dir)) => dir,
            _ => unreachable!(),
        };
        let add = add_file(next, &path[1..path.len()], name, size);
        dir.total += add;
        add
    } else {
        if let Some(Node::File(_existing)) = dir.entries.get(name) {
            0
        } else {
            dir.entries.insert(name.to_string(), Node::File(size));
            dir.total += size;
            size
        }
    }
}

impl Solver {
    fn parse(&mut self, line: &Line) {
        match line {
            Line::ChangeDirectory(path) => match path.as_str() {
                ".." => {
                    self.position.pop();
                }
                "/" => self.position.clear(),
                dir => {
                    // make sure dir exists
                    self.parse(&Line::Directory(path.to_string()));
                    self.position.push(dir.to_string());
                }
            },
            Line::ListFiles => {}
            Line::Directory(dir) => {
                let mut current = &mut self.root;
                for path in &self.position {
                    current = match current.entries.get_mut(path) {
                        Some(Node::Dir(dir)) => dir,
                        _ => unreachable!(),
                    };
                }
                current.entries.entry(dir.clone()).or_default();
            }
            Line::File(name, size) => {
                add_file(&mut self.root, &self.position[..], name, *size);
            }
        }
    }

    fn solve_dir(&self, max: u64, dir: &Dir) -> u64 {
        let mut sum = dir
            .entries
            .values()
            .map(|entry| match entry {
                Node::Dir(dir) => self.solve_dir(max, dir),
                _ => 0,
            })
            .sum();
        if dir.total < max {
            sum += dir.total;
        }
        sum
    }

    fn solve(&self, max: u64) -> u64 {
        self.solve_dir(max, &self.root)
    }

    fn solve_two_dir(&self, needs: u64, dir: &Dir) -> Option<u64> {
        dir.entries
            .iter()
            .filter_map(|(_key, entry)| match entry {
                Node::Dir(dir) => Some(dir),
                _ => None,
            })
            .filter_map(|dir| self.solve_two_dir(needs, dir))
            .chain(dir.total.checked_sub(needs).map(|_| dir.total).into_iter())
            .min()
    }

    fn solve_two(&self, needs: u64, has: u64) -> u64 {
        let free = has - self.root.total;
        let needs = needs - free;
        self.solve_two_dir(needs, &self.root).unwrap()
    }
}

fn solve(lines: &[Line]) -> (u64, u64) {
    let mut solver = Solver::default();
    for line in lines {
        solver.parse(&line);
    }
    (solver.solve(100000), solver.solve_two(30000000, 70000000))
}

#[test]
fn can_parse() {
    let lines = parse(include_str!("../example.txt"));
    assert_eq!(lines[0], Line::ChangeDirectory("/".into()));
    assert_eq!(lines[1], Line::ListFiles);
    assert_eq!(lines[2], Line::Directory("a".into()));
    assert_eq!(lines[3], Line::File("b.txt".into(), 14848514));
    assert_eq!(lines[4], Line::File("c.dat".into(), 8504156));
    assert_eq!(lines[5], Line::Directory("d".into()));
}

#[test]
fn can_solve() {
    let lines = parse(include_str!("../example.txt"));
    let (one, two) = solve(&lines);
    assert_eq!(95437, one);
    assert_eq!(24933642, two);
}

fn main() {
    let file = std::env::args().nth(1).unwrap();
    let data = std::fs::read_to_string(file).unwrap();
    let data = parse(&data);
    let (result, result_two) = solve(&data);
    println!("{result}");
    println!("{result_two}");
}
