use std::collections::{HashSet, VecDeque};
use std::vec::Vec;
use ya_advent_lib::algorithm::dijkstra_ex;
use ya_advent_lib::coords::{CDir, Coord2D};
use ya_advent_lib::read::read_input;
use ya_advent_lib::grid::Grid;

#[allow(dead_code)]
#[derive(Clone, Copy, Eq, PartialEq)]
enum Cell {
    Wall,
    Empty,
    Start,
    End,
    Path,
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

impl From<Cell> for char {
    fn from(c: Cell) -> char {
        match c {
            Cell::Wall => '#',
            Cell::Start => 'S',
            Cell::End => 'E',
            Cell::Empty => '.',
            Cell::Path => 'O',
        }
    }
}

fn both_parts(input: &[String]) -> (usize, usize, String) {
    let mut grid: Grid<Cell> = Grid::from_input(input, Cell::Empty, 0);
    let start: Coord2D = grid.find(|c, _, _| c == Cell::Start).unwrap().into();
    let end: Coord2D = grid.find(|c, _, _| c == Cell::End).unwrap().into();
    let (cost, prev) = dijkstra_ex(
        (start, CDir::E),
        |(loc, _)| *loc == end,
        |(loc, dir)| {
            let mut n = vec![
                ((*loc, dir.left()), 1000),
                ((*loc, dir.right()), 1000),
            ];
            if grid.get_c(*loc + *dir) != Cell::Wall {
                n.push((((*loc + *dir), *dir), 1));
            }
            return n;
        },
        true,
    ).unwrap();

    let mut seats: HashSet<Coord2D> = HashSet::new();
    let mut queue: VecDeque<(Coord2D, CDir)> = VecDeque::new();
    prev.iter().filter(|(coord, _)| coord.0 == end).for_each(|node| {
        queue.push_back(*node.0);
    });
    while let Some(node) = queue.pop_front() {
        grid.set_c(node.0, Cell::Path);
        seats.insert(node.0);
        if let Some(pnode) = prev.get(&node) {
            pnode.1.iter()
                .for_each(|n| {
                    queue.push_back(*n);
                });
        }
    }

    (cost, seats.len(), grid.format())
}

fn main() {
    let input: Vec<String> = read_input();
    let (part1, part2, _) = both_parts(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day16_test() {
        let input: Vec<String> = test_input(
"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
");
        let (part1, part2, result) = both_parts(&input);
        assert_eq!(part1, 7036);
        assert_eq!(part2, 45);
        assert_eq!(result,
"###############
#.......#....O#
#.#.###.#.###O#
#.....#.#...#O#
#.###.#####.#O#
#.#.#.......#O#
#.#.#####.###O#
#..OOOOOOOOO#O#
###O#O#####O#O#
#OOO#O....#O#O#
#O#O#O###.#O#O#
#OOOOO#...#O#O#
#O###.#.#.#O#O#
#O..#.....#OOO#
###############
");

        let input: Vec<String> = test_input(
"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
");
        let (part1, part2, result) = both_parts(&input);
        assert_eq!(part1, 11048);
        assert_eq!(part2, 64);
        assert_eq!(result,
"#################
#...#...#...#..O#
#.#.#.#.#.#.#.#O#
#.#.#.#...#...#O#
#.#.#.#.###.#.#O#
#OOO#.#.#.....#O#
#O#O#.#.#.#####O#
#O#O..#.#.#OOOOO#
#O#O#####.#O###O#
#O#O#..OOOOO#OOO#
#O#O###O#####O###
#O#O#OOO#..OOO#.#
#O#O#O#####O###.#
#O#O#OOOOOOO..#.#
#O#O#O#########.#
#O#OOO..........#
#################
");

        let input: Vec<String> = test_input(
"###########################
#######################..E#
######################..#.#
#####################..##.#
####################..###.#
###################..##...#
##################..###.###
#################..####...#
################..#######.#
###############..##.......#
##############..###.#######
#############..####.......#
############..###########.#
###########..##...........#
##########..###.###########
#########..####...........#
########..###############.#
#######..##...............#
######..###.###############
#####..####...............#
####..###################.#
###..##...................#
##..###.###################
#..####...................#
#.#######################.#
#S........................#
###########################
");
        let (part1, part2, _) = both_parts(&input);
        assert_eq!(part1, 21148);
        assert_eq!(part2, 149);

        let input: Vec<String> = test_input(
"####################################################
#......................................#..........E#
#......................................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.............................#
#S...................#.............................#
####################################################
");
        let (part1, part2, _) = both_parts(&input);
        assert_eq!(part1, 5078);
        assert_eq!(part2, 413);
    }
}
