use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::iter;
use std::vec::Vec;
use ya_advent_lib::coords::Coord2D;
use ya_advent_lib::read::read_input;

fn find_paths(a: Coord2D, b: Coord2D, gap: Coord2D) -> Vec<String> {
    let mut q = VecDeque::from([(a, String::new())]);
    let mut res = vec![];
    while let Some((loc, mut path)) = q.pop_front() {
        if loc == b {
            path.push('A');
            res.push(path);
            continue;
        }
        // left
        if b.y < loc.y && !(gap.x == loc.x && gap.y < loc.y && gap.y >= b.y) {
            let mut new_path = path.clone();
            new_path.extend(iter::repeat('<').take((loc.y - b.y) as usize));
            q.push_back((Coord2D::new(loc.x, b.y), new_path));
        }
        // up
        if b.x < loc.x && !(gap.y == loc.y && gap.x < loc.x && gap.x >= b.x) {
            let mut new_path = path.clone();
            new_path.extend(iter::repeat('^').take((loc.x - b.x) as usize));
            q.push_back((Coord2D::new(b.x, loc.y), new_path));
        }
        // down
        if b.x > loc.x && !(gap.y == loc.y && gap.x > loc.x && gap.x <= b.x) {
            let mut new_path = path.clone();
            new_path.extend(iter::repeat('v').take((b.x - loc.x) as usize));
            q.push_back((Coord2D::new(b.x, loc.y), new_path));
        }
        // right
        if b.y > loc.y && !(gap.x == loc.x && gap.y > loc.y && gap.y <= b.y) {
            let mut new_path = path.clone();
            new_path.extend(iter::repeat('>').take((b.y - loc.y) as usize));
            q.push_back((Coord2D::new(loc.x, b.y), new_path));
        }
    }
    res
}

struct Keypad {
    keymap: HashMap<char, Coord2D>,
    gap: Coord2D,
}

impl Keypad {
    fn numeric() -> Self {
        let keymap = HashMap::from([
            ('7', Coord2D::new(0, 0)),
            ('8', Coord2D::new(0, 1)),
            ('9', Coord2D::new(0, 2)),
            ('4', Coord2D::new(1, 0)),
            ('5', Coord2D::new(1, 1)),
            ('6', Coord2D::new(1, 2)),
            ('1', Coord2D::new(2, 0)),
            ('2', Coord2D::new(2, 1)),
            ('3', Coord2D::new(2, 2)),
            ('0', Coord2D::new(3, 1)),
            ('A', Coord2D::new(3, 2)),
        ]);
        let gap = Coord2D::new(3, 0);
        Keypad { keymap, gap }
    }

    fn directional() -> Self {
        let keymap = HashMap::from([
            ('^', Coord2D::new(0, 1)),
            ('A', Coord2D::new(0, 2)),
            ('<', Coord2D::new(1, 0)),
            ('v', Coord2D::new(1, 1)),
            ('>', Coord2D::new(1, 2)),
        ]);
        let gap = Coord2D::new(0, 0);
        Keypad { keymap, gap }
    }

    fn paths(&self, a: char, b: char) -> Vec<String> {
        find_paths(self.keymap[&a], self.keymap[&b], self.gap)
    }
}

fn shortest_len(
    np: &Keypad,
    dp: &Keypad,
    code: String,
    depth: usize,
    max_depth: usize,
    cache: &mut HashMap<(usize, String), usize>,
) -> usize {
    if let Some(&cached) = cache.get(&(depth, code.clone())) {
        return cached;
    }

    let kp = if depth == 0 { np } else { dp };
    let res = iter::once('A')
        .chain(code.chars())
        .tuple_windows()
        .map(|(a, b)| {
            let paths = kp.paths(a, b);
            if depth == max_depth {
                paths.iter().map(String::len).min().unwrap()
            } else {
                paths
                    .into_iter()
                    .map(|path| shortest_len(np, dp, path, depth + 1, max_depth, cache))
                    .min()
                    .unwrap()
            }
        })
        .sum();

    cache.insert((depth, code), res);
    res
}

fn dothething(input: &[String], max_depth: usize) -> usize {
    let np = Keypad::numeric();
    let dp = Keypad::directional();
    let mut cache = HashMap::new();
    input
        .iter()
        .map(|code| {
            shortest_len(&np, &dp, code.clone(), 0, max_depth, &mut cache)
                * code[0..3].parse::<usize>().unwrap()
        })
        .sum()
}

fn part1(input: &[String]) -> usize {
    dothething(input, 2)
}

fn part2(input: &[String]) -> usize {
    dothething(input, 25)
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
    fn day21_test() {
        let input: Vec<String> = test_input(
            "029A
980A
179A
456A
379A
",
        );
        assert_eq!(part1(&input), 126384);
    }
}
