static INPUT: &str = include_str!("../../input.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    find_marker(input, 4)
}

fn part2(input: &str) -> usize {
    find_marker(input, 14)
}

fn find_marker(input: &str, size: usize) -> usize {
    for start in 0..(input.len()-size) {
        if is_distinct(&input[start..start+size]) {
            return start + size;
        }
    }
    0
}

fn is_distinct(s: &str) -> bool {
    let mut chars: Vec<_> = s.chars().collect();
    chars.sort();
    chars.dedup();
    chars.len() == s.len()
}

#[cfg(test)]
static SAMPLE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

#[test]
fn test_find_marker() {
    assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
}

#[test]
fn test_part1() {
    assert_eq!(part1(SAMPLE), 7);
    assert_eq!(part1(INPUT), 1282);
}

#[test]
fn test_part2() {
    assert_eq!(part2(SAMPLE), 19);
    assert_eq!(part2(INPUT), 3513);
}