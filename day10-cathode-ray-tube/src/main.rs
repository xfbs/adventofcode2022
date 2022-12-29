use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::str::FromStr;

const TIMES: &[u64] = &[20, 60, 100, 140, 180, 220];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Instruction {
    AddX(i64),
    NoOp,
}

impl Instruction {
    pub fn apply(&self, cpu: &mut Cpu) {
        match self {
            Instruction::AddX(value) => {
                cpu.tick();
                cpu.tick();
                cpu.x += value;
            }
            Instruction::NoOp => {
                cpu.tick();
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut words = line.split(" ");
        let first = words
            .next()
            .ok_or_else(|| "missing instruction".to_string())?;
        match first {
            "addx" => Ok(Instruction::AddX(
                words
                    .next()
                    .ok_or_else(|| format!("missing addx argument"))?
                    .parse()
                    .map_err(|e| format!("error parsing addx argument: {e}"))?,
            )),
            "noop" => Ok(Instruction::NoOp),
            other => Err(format!("invalid instruction {other}")),
        }
    }
}

fn parse(data: &str) -> Vec<Instruction> {
    data.lines().map(|line| line.parse().unwrap()).collect()
}

struct Cpu {
    cycle: u64,
    x: i64,
    callback: Box<dyn Fn(&Cpu)>,
}

impl Cpu {
    pub fn new(callback: Box<dyn Fn(&Cpu)>) -> Self {
        Cpu {
            cycle: 0,
            x: 1,
            callback: callback,
        }
    }

    pub fn tick(&mut self) {
        self.cycle += 1;
        (self.callback)(&self);
    }
}

#[test]
fn can_parse() {
    let data = include_str!("../example.txt");
    let instructions = parse(&data);
    assert_eq!(instructions[0], Instruction::AddX(15));
    assert_eq!(instructions[1], Instruction::AddX(-11));
    assert_eq!(instructions[2], Instruction::AddX(6));
    assert_eq!(instructions[3], Instruction::AddX(-3));
    assert_eq!(instructions[9], Instruction::NoOp);
}

fn solve(instructions: &[Instruction], clocks: &[u64]) -> i64 {
    let times: BTreeMap<u64, Option<i64>> = clocks.into_iter().map(|c| (*c, None)).collect();
    let times = Rc::new(RefCell::new(times));
    let times_clone = times.clone();
    let mut cpu = Cpu::new(Box::new(move |cpu| {
        match times_clone.borrow_mut().get_mut(&cpu.cycle) {
            Some(value) => *value = Some(cpu.cycle as i64 * cpu.x),
            _ => {}
        }
    }));
    for instruction in instructions {
        instruction.apply(&mut cpu);
    }
    cpu.tick();
    let times = times.borrow();
    times.values().filter_map(|v| *v).sum()
}

#[test]
fn can_solve() {
    let data = include_str!("../example.txt");
    let instructions = parse(&data);
    let solution = solve(&instructions, TIMES);
    assert_eq!(solution, 13140);
}

fn main() {
    let file = std::env::args().nth(1).unwrap();
    let data = std::fs::read_to_string(file).unwrap();
    let data = parse(&data);
    let result = solve(&data, TIMES);
    println!("{result}");
}
