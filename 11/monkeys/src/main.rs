static INPUT: &str = include_str!("../../input.txt");

struct Monkey {
    items: Vec<i64>,
    op: char,
    parm: Option<i64>,
    test_val: i64,
    true_dest: usize,
    false_dest: usize
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|s| parse_monkey(s))
        .collect();
    let mut inspect_counts: Vec<usize> = vec![0; monkeys.len()];
    for r in 0..20 {
        for i in 0..monkeys.len() {
            inspect_counts[i] += monkeys[i].items.len();
            // println!("Monkey {}:", i);
            let adds = act_monkey(&monkeys[i], true);
            monkeys[i].items = vec![];
            for (idx, val) in adds {
                monkeys[idx].items.push(val);
            }
        }
        // println!("=== ROUND {} ===", r+1);
        // for i in 0..monkeys.len() {
        //     println!("Monkey {}: {:?}", i, monkeys[i].items);
        // }
    }
    inspect_counts.sort_by(|a, b| b.partial_cmp(a).unwrap());
    // println!("ACTIVITY: {:?}", inspect_counts);
    inspect_counts[0] * inspect_counts[1]
}

fn part2(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|s| parse_monkey(s))
        .collect();
    let mut inspect_counts: Vec<usize> = vec![0; monkeys.len()];
    for r in 0..30 {
        for i in 0..monkeys.len() {
            inspect_counts[i] += monkeys[i].items.len();
            // println!("Monkey {}:", i);
            let adds = act_monkey(&monkeys[i], false);
            monkeys[i].items = vec![];
            for (idx, val) in adds {
                monkeys[idx].items.push(val);
            }
        }
        // println!("=== ROUND {} ===", r+1);
        // for i in 0..monkeys.len() {
        //     println!("Monkey {}: {:?}", i, monkeys[i].items);
        // }
        println!("ROUND {}: ACTIVITY: {:?}", r, inspect_counts);
    }
    inspect_counts.sort_by(|a, b| b.partial_cmp(a).unwrap());
    println!("ACTIVITY: {:?}", inspect_counts);
    inspect_counts[0] * inspect_counts[1]
}

fn act_monkey(monkey: &Monkey, get_bored: bool) -> Vec<(usize, i64)> {
    let mut adds: Vec<(usize, i64)> = Vec::new();
    let mut items = monkey.items.clone();
    while items.len() > 0 {
        let mut item = items.remove(0);
        // println!("  Monkey inspects an item with a worry level of {}.", item);
        let old = item.clone();
        let parm: i64;
        if let Some(v) = monkey.parm {
            parm = v;
        } else {
            parm = old;
        }
        println!("    Worry level is adjusting ({} {}) to {}.", monkey.op, parm, item);
        item = match monkey.op {
            '+' => old + parm,
            '*' => old * parm,
            _ => todo!()
        };
        // println!("    Worry level is adjusted ({} {}) to {}.", monkey.op, parm, item);
        if get_bored {
            item /= 3;
        } else {
            // item %= monkey.test_val;
        }
        // println!("    Monkey gets bored with item. Worry level is divided by 3 to {}.", item);
        if item % monkey.test_val == 0 {
            // println!("    Current worry level is divisible by {}.", monkey.test_val);
            // println!("    Item with worry level {} is thrown to monkey {}.", item, monkey.true_dest);
            adds.push((monkey.true_dest, item));
        } else {
            // println!("    Current worry level is not divisible by {}.", monkey.test_val);
            // println!("    Item with worry level {} is thrown to monkey {}.", item, monkey.false_dest);
            adds.push((monkey.false_dest, item));
        }
    }
    adds
}

fn parse_monkey(input: &str) -> Monkey {
    let mut lines = input.lines();
    lines.next();
    let items = lines.next().unwrap()
        .split_once(": ").unwrap().1
        .split(", ")
        .map(|v| v.parse::<i64>().ok().unwrap())
        .collect();
    let (op, parm) = lines.next().unwrap()
        .split_once(" new = old ").unwrap().1
        .split_once(" ").unwrap();
    let test_val = lines.next().unwrap()
        .split_once(" divisible by ").unwrap().1
        .parse::<i64>().ok().unwrap();
    let true_dest = lines.next().unwrap()
        .split_once(" throw to monkey ").unwrap().1
        .parse::<i64>().ok().unwrap();
    let false_dest = lines.next().unwrap()
        .split_once(" throw to monkey ").unwrap().1
        .parse::<i64>().ok().unwrap();
    Monkey { 
        items: items, 
        op: op.chars().next().unwrap(), 
        parm: if parm == "old" { None } else { parm.parse::<i64>().ok() }, 
        test_val: test_val, 
        true_dest: true_dest as usize, 
        false_dest: false_dest as usize
    }
}

#[cfg(test)]
static SAMPLE: &str = include_str!("../../sample.txt");

#[test]
fn test_parse_monkey() {
    let s: &str = SAMPLE.split("\n\n").next().unwrap();
    let monkey = parse_monkey(s);
    assert_eq!(monkey.items, vec![79, 98]);
    assert_eq!(monkey.op, '*');
    assert_eq!(monkey.parm, Some(19));
    assert_eq!(monkey.test_val, 23);
    assert_eq!(monkey.true_dest, 2);
    assert_eq!(monkey.false_dest, 3);
}

#[test]
fn test_part1() {
    assert_eq!(part1(SAMPLE), 10605);
    assert_eq!(part1(INPUT), 112815);
}

#[test]
fn test_part2() {
    assert_eq!(part2(SAMPLE), 2713310158);
    // assert_eq!(part2(INPUT), 0);
}