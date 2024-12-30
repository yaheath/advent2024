use itertools::Itertools;
use std::{collections::HashMap, num::ParseIntError, str::FromStr, time::Instant, vec::Vec};
use ya_advent_lib::read::read_input;

#[derive(Copy, Clone)]
struct MonkeyRNG(usize);

impl FromStr for MonkeyRNG {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<usize>().map(Self)
    }
}

impl MonkeyRNG {
    fn next(&mut self) -> usize {
        self.0 = ((self.0 << 6) ^ self.0) & 0xffffff;
        self.0 = (self.0 >> 5) ^ self.0;
        self.0 = ((self.0 << 11) ^ self.0) & 0xffffff;
        self.0
    }
}

impl IntoIterator for MonkeyRNG {
    type Item = usize;
    type IntoIter = MonkeyRNGIterator;
    fn into_iter(self) -> Self::IntoIter {
        MonkeyRNGIterator(self)
    }
}

struct MonkeyRNGIterator(MonkeyRNG);

impl Iterator for MonkeyRNGIterator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.next())
    }
}

fn part1(input: &[MonkeyRNG]) -> usize {
    input
        .iter()
        .copied()
        .map(|m| m.into_iter().nth(1999).unwrap())
        .sum()
}

fn part2(input: &[MonkeyRNG]) -> usize {
    let mut total_set: HashMap<(i8, i8, i8, i8), usize> = HashMap::new();
    input
        .iter()
        .copied()
        .map(|m| {
            let mut set: HashMap<(i8, i8, i8, i8), usize> = HashMap::new();
            m.into_iter()
                .take(2000)
                .map(|v| (v % 10) as i8)
                .tuple_windows()
                .for_each(|(a, b, c, d, e)| {
                    let k = (b - a, c - b, d - c, e - d);
                    set.entry(k).or_insert(e as usize);
                });
            set
        })
        .for_each(|set| {
            set.into_iter().for_each(|(k, v)| {
                total_set.entry(k).and_modify(|tot| *tot += v).or_insert(v);
            });
        });
    total_set.into_values().max().unwrap()
}

fn main() {
    let input: Vec<MonkeyRNG> = read_input();
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
    fn monkeyrng_test() {
        let mut rng = MonkeyRNG(123);
        assert_eq!(rng.next(), 15887950);
        assert_eq!(rng.next(), 16495136);
        assert_eq!(rng.next(), 527345);
        assert_eq!(rng.next(), 704524);
    }

    #[test]
    fn day22_test() {
        let input: Vec<MonkeyRNG> = test_input("1\n10\n100\n2024\n");
        assert_eq!(part1(&input), 37327623);
        let input: Vec<MonkeyRNG> = test_input("1\n2\n3\n2024\n");
        assert_eq!(part2(&input), 23);
    }
}
