use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_sectioned_input;

struct Towels(Vec<String>);

impl FromStr for Towels {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let t = s.split(", ").map(|st| st.to_owned()).collect();
        Ok(Towels(t))
    }
}

fn search(towels: &Towels, pattern: &str) -> usize {
    let mut cache = HashMap::new();
    search_c(towels, pattern, &mut cache)
}

fn search_c<'a>(towels: &Towels, pattern: &'a str, cache: &mut HashMap<&'a str, usize>) -> usize {
    if let Some(&c) = cache.get(pattern) {
        return c;
    }
    let mut sum = 0;
    for t in towels.0.iter() {
        if pattern.starts_with(t) {
            if pattern == t {
                cache.insert(pattern, 1);
                sum += 1;
            } else {
                let n = search_c(towels, &pattern[t.len()..], cache);
                cache.insert(&pattern[t.len()..], n);
                sum += n;
            }
        }
    }
    sum
}

fn both_parts(towels: &Towels, patterns: &[String]) -> (usize, usize) {
    let r = patterns
        .iter()
        .map(|p| search(towels, p))
        .collect::<Vec<_>>();
    (r.iter().filter(|v| **v > 0).count(), r.iter().sum())
}

fn main() {
    let input: (Vec<Towels>, Vec<String>) = read_sectioned_input();
    let (part1, part2) = both_parts(&input.0[0], &input.1);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::sectioned_test_input;

    #[test]
    fn day19_test() {
        let input: (Vec<Towels>, Vec<String>) = sectioned_test_input(
            "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
",
        );
        let (part1, part2) = both_parts(&input.0[0], &input.1);
        assert_eq!(part1, 6);
        assert_eq!(part2, 16);
    }
}
