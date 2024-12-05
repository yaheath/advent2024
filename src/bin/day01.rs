use std::collections::HashMap;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

fn mk_lists(input: &[String]) -> (Vec<i64>, Vec<i64>) {
    let mut list1: Vec<i64> = Vec::with_capacity(input.len());
    let mut list2: Vec<i64> = Vec::with_capacity(input.len());
    input.iter().for_each(|s| {
        let v: Vec<i64> = s
            .split_whitespace()
            .map(|ss| ss.parse::<i64>().unwrap())
            .collect();
        list1.push(v[0]);
        list2.push(v[1]);
    });
    (list1, list2)
}

fn part1(input: &[String]) -> i64 {
    let (mut list1, mut list2) = mk_lists(input);
    list1.sort();
    list2.sort();
    list1
        .iter()
        .zip(list2)
        .map(|(i1, i2)| (i1 - i2).abs())
        .sum()
}

fn part2(input: &[String]) -> i64 {
    let (list1, list2) = mk_lists(input);
    let mut hist2: HashMap<i64, i64> = HashMap::new();
    list2.iter().for_each(|i| {
        hist2.entry(*i).and_modify(|c| *c += 1).or_insert(1);
    });
    list1
        .iter()
        .map(|i| {
            if hist2.contains_key(i) {
                hist2[i] * i
            } else {
                0
            }
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
    fn day01_test() {
        let input: Vec<String> = test_input(
            "3   4
4   3
2   5
1   3
3   9
3   3
",
        );
        assert_eq!(part1(&input), 11);
        assert_eq!(part2(&input), 31);
    }
}
