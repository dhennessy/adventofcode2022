static INPUT: &str = include_str!("../../input.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> u64 {
    let mut score = 0;
    for line in input.lines() {
        let round = line
            .replace("X", "A")
            .replace("Y", "B")
            .replace("Z", "C");
        score += score_round(&round);
    }
    score
}

fn part2(input: &str) -> u64 {
    let mut score = 0;
    for line in input.lines() {
        let opponent = &line[0..1];
        let end = line.chars().last().unwrap();
        let choice: &str = match end {
            'X' => match opponent {
                "A" => "C",
                "B" => "A",
                "C" => "B",
                _ => todo!()
            },
            'Y' => opponent,
            'Z' => match opponent {
                "A" => "B",
                "B" => "C",
                "C" => "A",
                _ => todo!()
            },
            _ => todo!()
        };
        let round = format!("{} {}", opponent, choice);
        score += score_round(&round);
    }
    score
}

fn score_round(s: &str) -> u64 {
    match s {
        "A A" => 1 + 3,
        "A B" => 2 + 6,
        "A C" => 3 + 0,
        "B A" => 1 + 0,
        "B B" => 2 + 3,
        "B C" => 3 + 6,
        "C A" => 1 + 6,
        "C B" => 2 + 0,
        "C C" => 3 + 3,
        _ => 0
    }
}

#[cfg(test)]
static SAMPLE: &str = "A Y
B X
C Z";

#[test]
fn test_scoring() {
    assert_eq!(score_round("A B"), 8);
    assert_eq!(score_round("B A"), 1);
    assert_eq!(score_round("C C"), 6);
}

#[test]
fn test_part1() {
    assert_eq!(part1(SAMPLE), 15);
}

#[test]
fn test_part2() {
    assert_eq!(part2(SAMPLE), 12);
}