use itertools::Itertools;
use std::collections::HashSet;
use std::vec::Vec;
use ya_advent_lib::read::read_input;
use ya_advent_lib::coords::{CDir, Coord2D};
use ya_advent_lib::grid::Grid;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
    Empty,
    Wall,
    Start,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '#' => Cell::Wall,
            '^' => Cell::Start,
            _ => Cell::Empty,
        }
    }
}

fn part1(input: &[String]) -> usize {
    let grid = Grid::from_input(input, Cell::Empty, 0);
    let mut pos: Coord2D = grid.find(|c, _, _| c == Cell::Start).unwrap().into();
    let mut dir = CDir::N;
    let mut stepped: HashSet<Coord2D> = HashSet::new();
    loop {
        stepped.insert(pos);
        let next = pos + dir;
        if !grid.contains_coord(next) {
            break;
        }
        match grid.get_c(next) {
            Cell::Wall => {
                dir = dir.right();
            },
            _ => {
                pos = next;
            }
        }
    }
    stepped.len()
}

trait GuardExits {
    fn guard_exits(&self, start: Coord2D, obstacle: Coord2D) -> bool;
}

impl GuardExits for Grid<Cell> {
    fn guard_exits(&self, start: Coord2D, obstacle: Coord2D) -> bool {
        let mut dir = CDir::N;
        let mut pos = start;
        let mut turns: HashSet<(Coord2D, CDir)> = HashSet::new();
        loop {
            let next = pos + dir;
            if !self.contains_coord(next) {
                return true;
            }
            let next_cell = if next == obstacle {
                Cell::Wall
            } else {
                self.get_c(next)
            };
            match next_cell {
                Cell::Wall => {
                    if turns.contains(&(pos, dir)) {
                        return false;
                    }
                    turns.insert((pos, dir));
                    dir = dir.right();
                },
                _ => {
                    pos = next;
                }
            }
        }
    }
}

fn part2(input: &[String]) -> usize {
    let grid = Grid::from_input(input, Cell::Empty, 0);
    let start: Coord2D = grid.find(|c, _, _| c == Cell::Start).unwrap().into();
    grid.x_bounds()
        .cartesian_product(grid.y_bounds())
        .filter(|(x, y)| grid.get(*x, *y) == Cell::Empty)
        .filter(|(x, y)| !grid.guard_exits(start, (*x, *y).into()))
        .count()
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
    fn day06_test() {
        let input: Vec<String> = test_input(
"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
");
        assert_eq!(part1(&input), 41);
        assert_eq!(part2(&input), 6);
    }
}
