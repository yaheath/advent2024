use lazy_static::lazy_static;
use regex::Regex;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

lazy_static! {
    static ref RE: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    static ref DD_RE: Regex = Regex::new(r"don't\(\).*?do\(\)").unwrap();
}

fn part1(input: &[String]) -> i64 {
    input
        .iter()
        .map(|line| {
            RE.captures_iter(line)
                .map(|cap| {
                    cap.get(1).unwrap().as_str().parse::<i64>().unwrap()
                        * cap.get(2).unwrap().as_str().parse::<i64>().unwrap()
                })
                .sum::<i64>()
        })
        .sum()
}

fn part2(input: &[String]) -> i64 {
    let joined = input.join(" ");
    let line = DD_RE.replace_all(&joined, "X");
    RE.captures_iter(&line)
        .map(|cap| {
            cap.get(1).unwrap().as_str().parse::<i64>().unwrap()
                * cap.get(2).unwrap().as_str().parse::<i64>().unwrap()
        })
        .sum()
}

fn main() {
    let input: Vec<String> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day03_test() {
        let input: Vec<String> =
            test_input("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(part1(&input), 161);
        let input: Vec<String> =
            test_input("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(part2(&input), 48);
    }
}
