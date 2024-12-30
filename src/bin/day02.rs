use itertools::Itertools;
use std::time::Instant;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

fn test_safe(vals: &[i64]) -> bool {
    let diffs: Vec<i64> = vals.iter().tuple_windows().map(|(a, b)| b - a).collect();
    if diffs[0] == 0 {
        false
    } else {
        let s = if diffs[0] < 0 { -1 } else { 1 };
        diffs.iter().map(|n| n * s).all(|n| n > 0 && n <= 3)
    }
}

fn test_safe2(vals: &[i64]) -> bool {
    if test_safe(vals) {
        return true;
    }
    for idx in 0..vals.len() {
        let mut v = Vec::from(vals);
        v.remove(idx);
        if test_safe(&v) {
            return true;
        }
    }
    false
}

fn part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|ss| ss.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .filter(|v| test_safe(v))
        .count()
}

fn part2(input: &[String]) -> usize {
    input
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|ss| ss.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .filter(|v| test_safe2(v))
        .count()
}

fn main() {
    let input: Vec<String> = read_input();
    let start = Instant::now();
    let part1 = part1(&input);
    let duration = start.elapsed();
    println!("Part 1: {part1} ({duration:?})");
    let start = Instant::now();
    let part2 = part2(&input);
    let duration = start.elapsed();
    println!("Part 2: {part2} ({duration:?})");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day02_test() {
        let input: Vec<String> = test_input(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
",
        );
        assert_eq!(part1(&input), 2);
        assert_eq!(part2(&input), 4);
    }
}
