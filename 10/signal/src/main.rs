static INPUT: &str = include_str!("../../input.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: \n{}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let signal = decode_signal(input);
    let mut strength = 0;
    let mut idx = 20;
    while idx < signal.len() {
        strength += idx as i64 * signal[idx-1];
        idx += 40;
    }
    strength as usize
}

fn part2(input: &str) -> String {
    let mut screen = String::new();
    let signal = decode_signal(input);
    for i in 0i64..240i64 {
        let x = signal[i as usize] % 40;
        let scan_x = i % 40;
        if x-1<=scan_x && x+1 >= scan_x {
            screen.push('#');
        } else {
            screen.push('.');
        }
        if (i+1)%40 == 0 {
            screen.push('\n');
        }
    }
    screen
}

fn decode_signal(input: &str) -> Vec<i64> {
    let mut x = 1;
    input
        .lines()
        .map(|line| 
            if line == "noop" {
                (1, 0)
            } else {
                (2, line.split_once(" ").unwrap().1.parse::<i64>().ok().unwrap())
            }
        )
        // .map(|x| [1])
        .map(|(cnt,dx)| if cnt == 1 { vec![x] } else { let px = x; x += dx; vec![px, px] })
        .flatten()
        .collect()
}

#[cfg(test)]
static SAMPLE0: &str = include_str!("../../sample0.txt");
#[cfg(test)]
static SAMPLE: &str = include_str!("../../sample.txt");

#[test]
fn test_decode_signal() {
    assert_eq!(decode_signal(SAMPLE0), vec![1, 1, 1, 4, 4])
}

#[test]
fn test_part1() {
    assert_eq!(part1(SAMPLE), 13140);
    assert_eq!(part1(INPUT), 13860);
}

#[test]
fn test_part2() {
    // println!("{}", part2(SAMPLE));
    assert_eq!(part2(SAMPLE), "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
".trim_start());
    assert_eq!(part2(INPUT), "
###..####.#..#.####..##....##..##..###..
#..#....#.#..#.#....#..#....#.#..#.#..#.
#..#...#..####.###..#.......#.#....###..
###...#...#..#.#....#.##....#.#....#..#.
#.#..#....#..#.#....#..#.#..#.#..#.#..#.
#..#.####.#..#.#.....###..##...##..###..
".trim_start());
}