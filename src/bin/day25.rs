use itertools::Itertools;
use std::vec::Vec;
use ya_advent_lib::read::read_grouped_input;

enum KL {
    Key([u8; 5]),
    Lock([u8; 5]),
}

impl KL {
    fn from_input(input: &[String]) -> Self {
        let p = input
            .iter()
            .map(|i| {
                i.chars()
                    .map(|c| if c == '#' { 1u8 } else { 0u8 })
                    .collect::<Vec<_>>()
            })
            .reduce(|acc, v| {
                acc.into_iter()
                    .zip(v)
                    .map(|(a, b)| a + b)
                    .collect::<Vec<u8>>()
            })
            .map(|v| [v[0], v[1], v[2], v[3], v[4]])
            .unwrap();

        if input[0] == "#####" {
            KL::Lock(p)
        } else {
            KL::Key(p)
        }
    }

    fn fits(&self, other: &Self) -> bool {
        let a = match self {
            KL::Key(x) => x,
            KL::Lock(x) => x,
        };
        let b = match other {
            KL::Key(x) => x,
            KL::Lock(x) => x,
        };
        a.iter().zip(b).all(|(x, y)| *x + *y <= 7)
    }
}

fn part1(input: &[Vec<String>]) -> usize {
    let kls = input.iter().map(|i| KL::from_input(i)).collect::<Vec<_>>();
    let keys = kls
        .iter()
        .filter(|kl| matches!(kl, KL::Key(_)))
        .collect::<Vec<_>>();
    let locks = kls
        .iter()
        .filter(|kl| matches!(kl, KL::Lock(_)))
        .collect::<Vec<_>>();
    keys.iter()
        .cartesian_product(locks)
        .filter(|(k, l)| k.fits(l))
        .count()
}

fn main() {
    let input: Vec<Vec<String>> = read_grouped_input();
    println!("Part 1: {}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::grouped_test_input;

    #[test]
    fn day25_test() {
        let input: Vec<Vec<String>> = grouped_test_input(include_str!("day25.testinput"));
        assert_eq!(part1(&input), 3);
    }
}
