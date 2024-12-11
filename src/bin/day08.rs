use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;
use ya_advent_lib::coords::Coord2D;
use ya_advent_lib::grid::Grid;
use ya_advent_lib::read::read_input;

#[derive(Copy, Clone)]
enum Cell {
    Empty,
    Antenna(char),
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            cc if cc.is_ascii_alphanumeric() => Cell::Antenna(cc),
            _ => Cell::Empty,
        }
    }
}

fn bothparts(input: &[String]) -> (usize, usize) {
    let grid: Grid<Cell> = Grid::from_input(input, Cell::Empty, 0);
    let mut antinodes: HashSet<Coord2D> = HashSet::new();
    let mut antinodes2: HashSet<Coord2D> = HashSet::new();
    let mut by_freq: HashMap<char, Vec<Coord2D>> = HashMap::new();
    grid.iter_with_coord()
        .filter_map(|(cell, x, y)| match cell {
            Cell::Antenna(c) => Some((c, x, y)),
            _ => None,
        })
        .for_each(|(c, x, y)| {
            by_freq
                .entry(c)
                .and_modify(|e| e.push((x, y).into()))
                .or_insert(Vec::from_iter([(x, y).into()]));
        });
    by_freq
        .into_values()
        .filter(|lst| lst.len() > 1)
        .for_each(|lst| {
            lst.iter().tuple_combinations().for_each(|(&a, &b)| {
                antinodes2.insert(a);
                antinodes2.insert(b);
                let diff = b - a;
                let mut n1 = a - diff;
                let mut first = true;
                while grid.contains_coord(n1) {
                    if first {
                        antinodes.insert(n1);
                        first = false;
                    }
                    antinodes2.insert(n1);
                    n1 -= diff;
                }
                let mut n2 = b + diff;
                first = true;
                while grid.contains_coord(n2) {
                    if first {
                        antinodes.insert(n2);
                        first = false;
                    }
                    antinodes2.insert(n2);
                    n2 += diff;
                }
            });
        });
    (antinodes.len(), antinodes2.len())
}

fn main() {
    let input: Vec<String> = read_input();
    let (part1, part2) = bothparts(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day08_test() {
        let input: Vec<String> = test_input(
            "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
",
        );
        let (part1, part2) = bothparts(&input);
        assert_eq!(part1, 14);
        assert_eq!(part2, 34);
    }
}
