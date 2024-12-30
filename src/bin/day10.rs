use std::collections::{HashSet, VecDeque};
use std::time::Instant;
use std::vec::Vec;
use ya_advent_lib::coords::Coord2D;
use ya_advent_lib::grid::Grid;
use ya_advent_lib::read::read_input;

#[derive(Clone, Copy)]
struct Elev(u8);

impl From<char> for Elev {
    fn from(c: char) -> Elev {
        match c {
            '0'..='9' => Elev(c as u8 - b'0'),
            _ => panic!(),
        }
    }
}

trait Day10 {
    fn find_score(&self, start: Coord2D) -> usize;
    fn find_rating(&self, start: Coord2D) -> usize;
}

impl Day10 for Grid<Elev> {
    fn find_score(&self, start: Coord2D) -> usize {
        let mut found_ends: HashSet<Coord2D> = HashSet::new();
        let mut stepped: HashSet<Coord2D> = HashSet::new();
        let mut queue: VecDeque<Coord2D> = VecDeque::new();
        stepped.insert(start);
        queue.push_back(start);
        while let Some(pos) = queue.pop_front() {
            let elev = self.get_c(pos).0;
            if elev == 9 {
                found_ends.insert(pos);
            } else {
                pos.neighbors4()
                    .iter()
                    .filter(|c| self.get_c(**c).0 == elev + 1)
                    .for_each(|c| {
                        if !stepped.contains(c) {
                            stepped.insert(*c);
                            queue.push_back(*c);
                        }
                    });
            }
        }
        found_ends.len()
    }

    fn find_rating(&self, start: Coord2D) -> usize {
        let mut found_paths: HashSet<Vec<Coord2D>> = HashSet::new();
        let mut queue: VecDeque<Vec<Coord2D>> = VecDeque::new();
        queue.push_back(vec![start]);
        while let Some(path) = queue.pop_front() {
            let end = path[path.len() - 1];
            let elev = self.get_c(end).0;
            if elev == 9 {
                found_paths.insert(path);
            } else {
                end.neighbors4()
                    .iter()
                    .filter(|c| self.get_c(**c).0 == elev + 1)
                    .filter(|c| !path.contains(c))
                    .for_each(|c| {
                        let mut next = path.clone();
                        next.push(*c);
                        queue.push_back(next);
                    });
            }
        }
        found_paths.len()
    }
}

fn part1(input: &[String]) -> usize {
    let grid: Grid<Elev> = Grid::from_input(input, Elev(255), 1);
    grid.iter_with_coord()
        .filter(|(c, _, _)| c.0 == 0)
        .map(|(_, x, y)| (x, y).into())
        .map(|start| grid.find_score(start))
        .sum()
}

fn part2(input: &[String]) -> usize {
    let grid: Grid<Elev> = Grid::from_input(input, Elev(255), 1);
    grid.iter_with_coord()
        .filter(|(c, _, _)| c.0 == 0)
        .map(|(_, x, y)| (x, y).into())
        .map(|start| grid.find_rating(start))
        .sum()
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
    fn day10_test() {
        let input: Vec<String> = test_input(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
",
        );
        assert_eq!(part1(&input), 36);
        assert_eq!(part2(&input), 81);
    }
}
