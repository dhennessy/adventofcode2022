static INPUT: &str = include_str!("../../input.txt");

type Map = Vec<Vec<char>>;

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let trees = parse_map(input);
    let mut visible = trees.clone();
    let dim = trees.len();
    let mut paths: Vec<Vec<(i64,i64)>> = Vec::new();
    for i in 0..dim {
        paths.push(compute_path((i as i64,0),(0,1),dim as i64));
        paths.push(compute_path((i as i64,dim as i64-1),(0,-1),dim as i64));
        paths.push(compute_path((0,i as i64),(1,0),dim as i64));
        paths.push(compute_path((dim as i64-1,i as i64),(-1,0),dim as i64));
    }
    for path in paths {
        let mut h = '.';
        for pt in path {
            let x = pt.0 as usize;
            let y = pt.1 as usize;
            if trees[y][x] > h {
                visible[y][x] = '*';
                h = trees[y][x];
            }
        }
    }
    count_visible(&visible)
}

fn part2(input: &str) -> usize {
    let trees = parse_map(input);
    let mut max_score = 0;
    let dim = trees.len();
    for y in 0..dim {
        for x in 0..dim {
            let s = scenic_score(&trees, (x as i64,y as i64));
            if s > max_score {
                max_score = s;
            }
        }
    }
    max_score
}

fn scenic_score(trees: &Map, pt: (i64, i64)) -> usize {
    let mut score = 1;
    let dirs = vec![(0,1), (0,-1), (1,0), (-1,0)];
    for dir in dirs {
        let path = compute_path(pt, dir, trees.len() as i64);
        let mut h = '.';
        let mut dist: i64 = -1;
        for pt in path.clone() {
            let x = pt.0 as usize;
            let y = pt.1 as usize;
            dist += 1;
            if dist == 0 {
                h = trees[y][x];
            } else {
                if trees[y][x] >= h {
                    break;
                }
            }
        }
        score *= dist;
    }
    score as usize
}

fn compute_path(start: (i64, i64), dir: (i64, i64), dim: i64) -> Vec<(i64,i64)> {
    let mut path = Vec::new();
    let mut pt = start;
    loop {
        path.push(pt.clone());
        pt.0 += dir.0;
        pt.1 += dir.1;
        if pt.0 < 0 || pt.0 >= dim || pt.1 < 0 || pt.1 >= dim {
            break;
        }
    }
    path
}

fn count_visible(map: &Map) -> usize {
    let mut count = 0;
    for row in map {
        count += row.iter().filter(|c| **c == '*').count();
    }
    count
}

fn parse_map(input: &str) -> Map {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

#[cfg(test)]
static SAMPLE: &str = include_str!("../../sample.txt");

#[test]
fn test_scenic_score() {
    let trees = parse_map(SAMPLE);    
    assert_eq!(scenic_score(&trees, (2,1)), 4);
}

#[test]
fn test_compute_path() {
    let path = compute_path((0,1), (1,0), 3);
    assert_eq!(path, vec![(0,1), (1,1), (2,1)]);
}

#[test]
fn test_parse_map() {
    let map = parse_map(SAMPLE);
    assert_eq!(map.len(), 5);
    assert_eq!(map[0].len(), 5);
}

#[test]
fn test_count_visible() {
    let map = vec![vec!['1', '*', '*'], vec!['1', '3', '*'], vec!['1', '*', '4']];
    assert_eq!(count_visible(&map), 4);
}

#[test]
fn test_part1() {
    assert_eq!(part1(SAMPLE), 21);
    assert_eq!(part1(INPUT), 1560);
}

#[test]
fn test_part2() {
    assert_eq!(part2(SAMPLE), 8);
    assert_eq!(part2(INPUT), 252000);
}