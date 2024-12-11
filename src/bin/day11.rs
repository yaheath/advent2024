use std::collections::HashMap;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

fn blink(stones: &[usize], n: usize) -> usize {
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    stones
        .iter()
        .map(|st| blink_one_stone(*st, n, &mut cache))
        .sum()
}

fn blink_one_stone(
    st: usize,
    n_blinks: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(result) = cache.get(&(st, n_blinks)) {
        return *result;
    }
    let mut stone = st;
    let mut total = 1;
    for i in 0..n_blinks {
        let as_str = format!("{stone}");
        let len = as_str.len();
        stone = match stone {
            0 => 1,
            _ if len & 1 == 0 => {
                let (a, b) = as_str.split_at(len / 2);
                let a = a.parse::<usize>().unwrap();
                let b = b.parse::<usize>().unwrap();
                total += blink_one_stone(b, n_blinks - i - 1, cache);
                a
            }
            n => n * 2024,
        };
    }
    cache.insert((st, n_blinks), total);
    total
}

fn part1(input: &[String]) -> usize {
    let stones = input[0]
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    blink(&stones, 25)
}

fn part2(input: &[String]) -> usize {
    let stones = input[0]
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    blink(&stones, 75)
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
    fn day11_test() {
        let input: Vec<String> = test_input("125 17");
        assert_eq!(part1(&input), 55312);
        //assert_eq!(part2(&input), 0);
    }
}
