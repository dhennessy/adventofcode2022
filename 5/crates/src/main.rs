use regex::Regex;
use std::str::FromStr;

static INPUT: &str = include_str!("../../input.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> String {
    let mut stacks = parse_stacks(input);
    let steps = parse_steps(input);
    for step in steps {
        for _ in 1..=step.0 {
            let src = &mut stacks[step.1 as usize - 1];
            let v = src.pop().unwrap();
            let dst = &mut stacks[step.2 as usize - 1];
            dst.push(v);
        }
    }
    stacks
        .iter()
        .map(|v| v.last().unwrap())
        .collect()
}

fn part2(input: &str) -> String {
    let mut stacks = parse_stacks(input);
    let steps = parse_steps(input);
    for step in steps {
        let mut temp: Vec<char> = Vec::new();
        for _ in 1..=step.0 {
            let src = &mut stacks[step.1 as usize - 1];
            let v = src.pop().unwrap();
            temp.insert(0, v);
        }
        let dst = &mut stacks[step.2 as usize - 1];
        dst.append(&mut temp);
    }
    stacks
        .iter()
        .map(|v| v.last().unwrap())
        .collect()
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        if line.starts_with(" 1") {
            break
        }
        let mut i = 0;
        for ch in line.chars() {
            if ch.is_alphabetic() {
                let idx = i/4;
                while stacks.len() <= idx {
                    stacks.push(Vec::new());
                }
                stacks[idx].insert(0, ch);
            }
            i += 1;
        }
    }
    stacks
}

fn parse_steps(input: &str) -> Vec<(u64, u64, u64)> {
    let mut steps: Vec<(u64, u64, u64)> = Vec::new();
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in input.lines() {
        if let Some(c) = re.captures(line) {
            let count = u64::from_str(&c[1]).unwrap();
            let src = u64::from_str(&c[2]).unwrap();
            let dst = u64::from_str(&c[3]).unwrap();
            steps.push((count, src, dst));
        }
    }
    steps
}

#[cfg(test)]
static SAMPLE: &str = include_str!("../../sample.txt");

#[test]
fn test_parse_stacks() {
    let stacks = parse_stacks(SAMPLE);
    assert_eq!(stacks.len(), 3);
    assert_eq!(stacks[0], vec!['Z', 'N']);
    assert_eq!(stacks[1], vec!['M', 'C', 'D']);
    assert_eq!(stacks[2], vec!['P']);
}

#[test]
fn test_parse_steps() {
    let steps = parse_steps(SAMPLE);
    assert_eq!(steps.len(), 4);
    assert_eq!(steps[0], (1, 2, 1));
}

#[test]
fn test_part1() {
    assert_eq!(part1(SAMPLE), "CMZ");
    assert_eq!(part1(INPUT), "CNSZFDVLJ");
}

#[test]
fn test_part2() {
    assert_eq!(part2(SAMPLE), "MCD");
    assert_eq!(part2(INPUT), "QNDWLMGNS");
}