use regex::Regex;
use std::ops::RangeInclusive;
use std::str::FromStr;

static INPUT: &str = include_str!("../../input.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> u64 {
    input.lines()
    .map(|line| parse_ranges(line))
    .filter(|r| contains_range(&r.0, &r.1))
    .count() as u64
}

fn part2(input: &str) -> u64 {
    input.lines()
    .map(|line| parse_ranges(line))
    .filter(|r| overlaps_range(&r.0, &r.1))
    .count() as u64
}

fn parse_ranges(line: &str) -> (RangeInclusive<u64>, RangeInclusive<u64>) {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let c = re.captures(line).unwrap();
    let s1 = u64::from_str(&c[1]).unwrap();
    let e1 = u64::from_str(&c[2]).unwrap();
    let s2 = u64::from_str(&c[3]).unwrap();
    let e2 = u64::from_str(&c[4]).unwrap();
    (s1..=e1,s2..=e2)
}

fn contains_range(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> bool {
    (a.contains(b.start()) && a.contains(b.end())) || 
    (b.contains(a.start()) && b.contains(a.end()))
}

fn overlaps_range(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> bool {
    a.contains(b.start()) || a.contains(b.end()) || 
    b.contains(a.start()) || b.contains(a.end())
}

#[cfg(test)]
static SAMPLE: &str = include_str!("../../sample.txt");

#[test]
fn test_parse_ranges() {
    assert_eq!(parse_ranges("2-4,6-8"), (2..=4, 6..=8));
    assert_eq!(parse_ranges("6-6,4-6"), (6..=6, 4..=6));
    assert_eq!(parse_ranges("2-8,3-7"), (2..=8, 3..=7));
}

#[test]
fn test_contains_range() {
    assert_eq!(contains_range(&(0..=2), &(3..=6)), false);
    assert_eq!(contains_range(&(6..=6), &(4..=6)), true);
    assert_eq!(contains_range(&(2..=8), &(3..=7)), true);
}

#[test]
fn test_part1() {
    assert_eq!(part1(SAMPLE), 2);
    assert_eq!(part1(INPUT), 507);
}

#[test]
fn test_part2() {
    assert_eq!(part2(SAMPLE), 4);
    assert_eq!(part2(INPUT), 897);
}