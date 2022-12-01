use std::fs;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let elves = gather_food(&input);
    let max = max_sum(&elves);
    let top_three = top_three(&elves);
    println!("The elf with the most is carrying {}", max);
    println!("The best three are carrying {}", top_three);
}

fn gather_food(s: &str) -> Vec<Vec<u64>> {
    let mut elves: Vec<Vec<u64>> = Vec::new();
    let mut food: Vec<u64> = Vec::new();
    for line in s.lines() {
        let calories = u64::from_str(line).unwrap_or_default();
        if calories > 0 {
            food.push(calories);
        } else {
            elves.push(food);
            food = Vec::new();
        }
    }
    if !food.is_empty() {
        elves.push(food);
    }

    elves
}

fn max_sum(elves: &Vec<Vec<u64>>) -> u64 {
    let mut max: u64 = 0;
    for food in elves {
        let sum = food.iter().sum();
        if sum > max {
            max = sum;
        }
    }
    max
}

fn top_three(elves: &Vec<Vec<u64>>) -> u64 {
    let mut sums: Vec<u64> = elves.iter().map(|v| v.iter().sum()).collect();
    sums.sort_by(|a, b| b.partial_cmp(a).unwrap());
    sums[0] + sums[1] + sums[2]
}

#[test]
fn test_sample() {
    let input = fs::read_to_string("../sample.txt").unwrap();
    let elves = gather_food(&input);
    assert_eq!(elves.len(), 5);
    assert_eq!(elves[2], vec![5000, 6000]);
}

#[test]
fn test_sum() {
    let elves = vec![vec![1, 2, 3], vec![4], vec![5,6], vec![7,8,9], vec![10]];
    assert_eq!(max_sum(&elves), 24);
}

#[test]
fn test_top_three() {
    let input = fs::read_to_string("../sample.txt").unwrap();
    let elves = gather_food(&input);
    assert_eq!(top_three(&elves), 45000);
}
