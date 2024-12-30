use std::collections::HashSet;
use std::time::Instant;
use std::vec::Vec;
use ya_advent_lib::algorithm::a_star_ex;
use ya_advent_lib::coords::Coord2D;
use ya_advent_lib::grid::Grid;
use ya_advent_lib::read::read_input;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Cell {
    Wall,
    Empty,
    Start,
    End,
}

impl From<char> for Cell {
    fn from(c: char) -> Cell {
        match c {
            '#' => Cell::Wall,
            'S' => Cell::Start,
            'E' => Cell::End,
            _ => Cell::Empty,
        }
    }
}

trait Day20Grid {
    fn find_cheats(&self, radius: i64) -> Vec<usize>;
}

impl Day20Grid for Grid<Cell> {
    fn find_cheats(&self, radius: i64) -> Vec<usize> {
        let start: Coord2D = self.find(|c, _, _| c == Cell::Start).unwrap().into();
        let end: Coord2D = self.find(|c, _, _| c == Cell::End).unwrap().into();
        let mut out = Vec::new();
        let Some((_, mut rpath)) = a_star_ex(
            start,
            |c| *c == end,
            |c| {
                c.neighbors4()
                    .into_iter()
                    .filter(|cc| self.get_c(*cc) != Cell::Wall)
                    .map(|cc| (cc, 1))
                    .collect()
            },
            |c| c.mdist_to(&end) as usize,
            false,
        ) else {
            panic!();
        };
        rpath.insert(start, (0, HashSet::from_iter([Coord2D::new(-1, -1)])));

        let mut pos = end;
        while let Some(node) = rpath.get(&pos) {
            pos.mdist_radius(radius)
                .filter_map(|n| rpath.get(&n).map(|d| (n, d)))
                .filter(|(n, d)| d.0 + (n.mdist_to(&pos) as usize) < node.0)
                .for_each(|(n, d)| {
                    out.push(node.0 - d.0 - (n.mdist_to(&pos) as usize));
                });
            pos = *node.1.iter().next().unwrap();
        }
        out
    }
}

fn part1(input: &[String]) -> usize {
    let grid: Grid<Cell> = Grid::from_input(input, Cell::Empty, 0);
    grid.find_cheats(2).iter().filter(|n| **n >= 100).count()
}

fn part2(input: &[String]) -> usize {
    let grid: Grid<Cell> = Grid::from_input(input, Cell::Empty, 0);
    grid.find_cheats(20).iter().filter(|n| **n >= 100).count()
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
    fn day20_test() {
        let input: Vec<String> = test_input(
            "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
",
        );
        let grid: Grid<Cell> = Grid::from_input(&input, Cell::Empty, 0);
        let v = grid.find_cheats(2).iter().filter(|n| **n >= 20).count();
        assert_eq!(v, 5);
        let v = grid.find_cheats(20).iter().filter(|n| **n >= 50).count();
        assert_eq!(v, 285);
    }
}
