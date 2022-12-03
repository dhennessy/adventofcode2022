use itertools::Itertools;

static INPUT: &str = include_str!("../../input.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> u64 {
    input.lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len()/2);
            let common = find_common_letters(left, right);
            priority(common.chars().next().unwrap())
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let mut sum = 0;
    for mut group in &input.lines().chunks(3) {
        let mut common: String = String::from(group.next().unwrap());
        common = find_common_letters(&common, &String::from(group.next().unwrap()));
        common = find_common_letters(&common, &String::from(group.next().unwrap()));
        sum += priority(common.chars().next().unwrap());
    }
    sum
}

fn find_common_letters(s1: &str, s2: &str) -> String {
    let mut common = String::new();
    for char in s1.chars() {
        if s2.find(char).is_some() {
            common.push(char);
        }
    }
    common
}

fn priority(ch: char) -> u64 {
    match ch {
        'a'..='z' => ch as u64 - 'a' as u64 + 1,
        'A'..='Z' => ch as u64 - 'A' as u64 + 27,
        _ => 0
    }
}

#[cfg(test)]
static SAMPLE: &str = include_str!("../../sample.txt");

#[test]
fn test_priority() {
    assert_eq!(priority('a'), 1);
    assert_eq!(priority('z'), 26);
    assert_eq!(priority('A'), 27);
    assert_eq!(priority('Z'), 52);
}

#[test]
fn test_find_common() {
    assert_eq!(find_common_letters("vJrwpWtwJgWr", "hcsFMMfFFhFp"), "p");
}

#[test]
fn test_part1() {
    assert_eq!(part1(SAMPLE), 157);
    assert_eq!(part1(INPUT), 8240);
}

#[test]
fn test_part2() {
    assert_eq!(part2(SAMPLE), 70);
    assert_eq!(part2(INPUT), 2587);
}