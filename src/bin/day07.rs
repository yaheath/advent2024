use itertools::{repeat_n, Itertools};
use std::str::FromStr;
use std::time::Instant;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

struct Input {
    lhs: u64,
    rhs: Vec<u64>,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lhs_s, rhs_s) = s.split_once(": ").unwrap();
        let lhs = lhs_s.parse::<u64>().unwrap();
        let rhs = rhs_s.split(' ').flat_map(|ss| ss.parse::<u64>()).collect();
        Ok(Input { lhs, rhs })
    }
}

#[derive(Debug, Copy, Clone)]
enum Oper {
    Add,
    Mul,
    Cat,
}

impl Input {
    fn is_valid(&self, forpart2: bool) -> bool {
        let ops = if forpart2 {
            vec![Oper::Add, Oper::Mul, Oper::Cat]
        } else {
            vec![Oper::Add, Oper::Mul]
        };
        for ops in repeat_n(ops, self.rhs.len() - 1).multi_cartesian_product() {
            let mut ops_iter = ops.iter();
            let e = self
                .rhs
                .iter()
                .copied()
                .reduce(|acc, n| match ops_iter.next() {
                    Some(Oper::Add) => acc + n,
                    Some(Oper::Mul) => acc * n,
                    Some(Oper::Cat) => format!("{acc}{n}").parse::<u64>().unwrap(),
                    _ => panic!(),
                })
                .unwrap();
            if e == self.lhs {
                return true;
            }
        }
        false
    }
}

fn part1(input: &[Input]) -> u64 {
    input
        .iter()
        .filter(|i| i.is_valid(false))
        .map(|i| i.lhs)
        .sum()
}

fn part2(input: &[Input]) -> u64 {
    input
        .iter()
        .filter(|i| i.is_valid(true))
        .map(|i| i.lhs)
        .sum()
}

fn main() {
    let input: Vec<Input> = read_input();
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
    fn day07_test() {
        let input: Vec<Input> = test_input(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
",
        );
        assert_eq!(part1(&input), 3749);
        assert_eq!(part2(&input), 11387);
    }
}
