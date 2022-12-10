use std::collections::HashSet;

static INPUT: &str = include_str!("../../input.txt");

type Point = (i64, i64);    // x, y

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    simulate_rope(input, 2)   
}

fn part2(input: &str) -> usize {
    simulate_rope(input, 10)   
}

fn simulate_rope(input: &str, knot_count: usize) -> usize {
    let mut visits: HashSet<Point> = HashSet::new();
    let mut knots: Vec<Point> = Vec::new();
    for _ in 0..knot_count {
        knots.push((0,0));
    }
    for line in input.lines() {
        for step in expand_moves(line) {
            let mut head = knots[0];
            match step {
                'R' => head = (head.0+1, head.1),
                'L' => head = (head.0-1, head.1),
                'U' => head = (head.0, head.1+1),
                'D' => head = (head.0, head.1-1),
                _ => todo!()                
            }
            knots[0] = head;
            for i in 0..knot_count-1 {
                knots[i+1] = move_tail(knots[i], knots[i+1]);
            }
            visits.insert(knots[knot_count-1]);
        }
        // println!("== {} ==", line);
        // dump_rope(&knots, &visits, 16, 16);
    }
    // dump_rope(&knots, &visits, 6, 5);
    visits.len()
}

fn move_tail(head: Point, tail: Point) -> Point {
    let dist = manhattan_dist(head, tail);
    if dist == 2 && (head.0 == tail.0 || head.1 == tail.1) {
        return try_move(head, tail, vec![(0,1),(0,-1),(1,0),(-1,0)]);
    } else if dist >= 3 {
        return try_move(head, tail, vec![(1,1),(-1,-1),(1,-1),(-1,1)]);
    }
    tail
}

fn try_move(head: Point, tail: Point, steps: Vec<(i64, i64)>) -> Point {
    let mut best_dist = 999;
    let mut best_move = tail;
    for step in steps {
        let p = (tail.0+step.0,tail.1+step.1);
        let dist = manhattan_dist(head, p);
        if dist < best_dist {
            best_dist = dist;
            best_move = p;
        }
    }
    best_move
}

fn manhattan_dist(a: Point, b: Point) -> usize {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as usize
}

fn expand_moves(line: &str) -> Vec<char> {
    let (dir, c) = line.split_once(" ").unwrap();
    let ch = dir.chars().next().unwrap();
    let count = c.parse::<usize>().ok().unwrap();
    let mut moves = Vec::new();
    for _ in 0..count {
        moves.push(ch);
    }
    moves
}

fn dump_rope(rope: &Vec<Point>, visits: &HashSet<Point>, width: i64, height: i64) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = width-1;
    let mut max_y = height-1;
    for visit in visits {
        if visit.0 < min_x {
            min_x = visit.0;
        }
        if visit.0 > max_x {
            max_x = visit.0;
        }
        if visit.1 < min_y {
            min_y = visit.1;
        }
        if visit.1 > max_y {
            max_y = visit.1;
        }
    }
    for rev_y in min_y..=max_y {
        let y = max_y-rev_y;
        'x: for x in min_x..=max_x {
            for i in 0..rope.len() {
                if rope[i].0 == x && rope[i].1 == y {
                    print!("{}", i);
                    continue 'x;
                }
            }
            for visit in visits {
                if visit.0 == x && visit.1 == y {
                    print!(".");
                    continue 'x;
                }
            }
            print!(".");
        }
        println!("");
    }
}

#[cfg(test)]
static SAMPLE: &str = include_str!("../../sample.txt");

#[cfg(test)]
static SAMPLE2: &str = include_str!("../../sample2.txt");

#[test]
fn test_simulate_rope() {
    assert_eq!(simulate_rope(SAMPLE, 2), 13);
}

#[test]
fn test_expand() {
    assert_eq!(expand_moves("R 3"), vec!['R','R','R']);
}

#[test]
fn test_move_tail() {
    assert_eq!(move_tail((3,1), (1,1)), (2,1));
    assert_eq!(move_tail((1,3), (1,1)), (1,2));
    assert_eq!(move_tail((2,1), (1,3)), (2,2));
    assert_eq!(move_tail((3,2), (1,3)), (2,2));
    assert_eq!(move_tail((2,2), (0,0)), (1,1));
}

#[test]
fn test_part1() {
    assert_eq!(part1(SAMPLE), 13);
    assert_eq!(part1(INPUT), 6266);
}

#[test]
fn test_part2() {
    assert_eq!(part2(SAMPLE2), 36);
    assert_eq!(part2(INPUT), 2369);
}