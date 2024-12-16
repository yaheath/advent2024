use itertools::Itertools;
use std::vec::Vec;
use ya_advent_lib::coords::{CDir, Coord2D};
use ya_advent_lib::read::read_grouped_input;
use ya_advent_lib::grid::Grid;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Cell {
    Wall,
    Box,
    BoxL,
    BoxR,
    Empty,
    Robot,
}

impl From<char> for Cell {
    fn from(c: char) -> Cell {
        match c {
            '#' => Cell::Wall,
            'O' => Cell::Box,
            '[' => Cell::BoxL,
            ']' => Cell::BoxR,
            '@' => Cell::Robot,
            _ => Cell::Empty,
        }
    }
}

trait Day15Grid {
    fn scaled_from_input(input: &[String]) -> Self;
    fn find_robot(&self) -> Coord2D;
    fn move_robot(&mut self, robot: Coord2D, dir: CDir) -> Coord2D;
    fn sum_gps(&self) -> i64;
    fn push_big_box(&mut self, b: Coord2D, dir: CDir, test: bool) -> bool;
}

impl Day15Grid for Grid<Cell> {
    fn scaled_from_input(input: &[String]) -> Self {
        let input2 = input.iter()
            .map(|line| line.chars().map(|c|
                match c {
                    '@' => "@.",
                    '.' => "..",
                    '#' => "##",
                    'O' => "[]",
                    _ => panic!(),
                }
            ).join(""))
            .collect::<Vec<_>>();
        Self::from_input(&input2, Cell::Empty, 0)
    }

    fn find_robot(&self) -> Coord2D {
        self.find(|c, _, _| c == Cell::Robot).unwrap().into()
    }

    fn move_robot(&mut self, robot: Coord2D, dir: CDir) -> Coord2D {
        let dest = robot + dir;
        match self.get_c(dest) {
            Cell::Empty => {
                self.set_c(dest, Cell::Robot);
                self.set_c(robot, Cell::Empty);
                return dest;
            },
            Cell::Wall => {
                return robot;
            },
            Cell::Box => {
                let mut next = dest + dir;
                while self.get_c(next) == Cell::Box {
                    next += dir;
                }
                match self.get_c(next) {
                    Cell::Wall => {
                        return robot;
                    },
                    Cell::Empty => {
                        self.set_c(next, Cell::Box);
                        self.set_c(dest, Cell::Robot);
                        self.set_c(robot, Cell::Empty);
                        return dest;
                    },
                    _ => panic!(),
                }
            },
            Cell::BoxL | Cell::BoxR => {
                if self.push_big_box(dest, dir, false) {
                    self.set_c(dest, Cell::Robot);
                    self.set_c(robot, Cell::Empty);
                    return dest;
                }
                return robot;
            }
            _ => panic!(),
        }
    }

    fn push_big_box(&mut self, b: Coord2D, dir: CDir, test: bool) -> bool {
        let c = self.get_c(b);
        match dir {
            CDir::E | CDir::W => {
                assert!(dir == CDir::E && c == Cell::BoxL || dir == CDir::W && c == Cell::BoxR);
                assert!(self.get_c(b + dir) == if dir == CDir::E { Cell::BoxR } else {Cell::BoxL});
                let c2 = self.get_c(b + dir);
                let next = self.get_c(b + dir + dir);
                match next {
                    Cell::Wall => {
                        false
                    },
                    Cell::Empty => {
            if !test {
                        self.set_c(b + dir + dir, c2);
                        self.set_c(b + dir, c);
                        self.set_c(b, Cell::Empty);
            }
                        true
                    },
                    Cell::BoxL | Cell::BoxR => {
                        if self.push_big_box(b + dir + dir, dir, test) {
            if !test {
                            self.set_c(b + dir + dir, c2);
                            self.set_c(b + dir, c);
                            self.set_c(b, Cell::Empty);
                }
                            true
                        }
                        else {
                            false
                        }
                    },
                    _ => panic!(),
                }
            },
            CDir::N | CDir::S => {
                let (bl, br) = if c == Cell::BoxL {
                    (b, b + CDir::E)
                } else {
                    (b + CDir::W, b)
                };
                match (self.get_c(bl + dir), self.get_c(br + dir)) {
                    (Cell::Wall, _) | (_, Cell::Wall) => {
                        false
                    },
                    (Cell::Empty, Cell::Empty) => {
    if !test {
                        self.set_c(bl + dir, Cell::BoxL);
                        self.set_c(br + dir, Cell::BoxR);
                        self.set_c(bl, Cell::Empty);
                        self.set_c(br, Cell::Empty);
    }
                        true
                    },
                    (Cell::BoxL, Cell::BoxR) => {
                        if self.push_big_box(bl + dir, dir, test) {
        if !test {
                            self.set_c(bl + dir, Cell::BoxL);
                            self.set_c(br + dir, Cell::BoxR);
                            self.set_c(bl, Cell::Empty);
                            self.set_c(br, Cell::Empty);
        }
                            true
                        } else {
                            false
                        }
                    },
                    (Cell::BoxR, Cell::BoxL) => {
                        if self.push_big_box(bl + dir, dir, true) && self.push_big_box(br + dir, dir, true) {
                            if !test {
                                self.push_big_box(bl + dir, dir, false);
                                self.push_big_box(br + dir, dir, false);
                                self.set_c(bl + dir, Cell::BoxL);
                                self.set_c(br + dir, Cell::BoxR);
                                self.set_c(bl, Cell::Empty);
                                self.set_c(br, Cell::Empty);
                            }
                            true
                        } else {
                            false
                        }
                    }
                    (Cell::BoxR, Cell::Empty) | (Cell::Empty, Cell::BoxL) => {
                        let bb = if self.get_c(bl + dir) == Cell::Empty { br } else { bl };
                        if self.push_big_box(bb + dir, dir, test) {
                            if !test {
                                self.set_c(bl + dir, Cell::BoxL);
                                self.set_c(br + dir, Cell::BoxR);
                                self.set_c(bl, Cell::Empty);
                                self.set_c(br, Cell::Empty);
                            }
                            true
                        } else {
                            false
                        }
                    },
                    _ => panic!(),
                }
            },
        }
    }

    fn sum_gps(&self) -> i64 {
        self.iter_with_coord()
            .filter_map(|(c, x, y)| match c {
                Cell::Box | Cell::BoxL => Some(100 * y + x),
                _ => None,
            })
            .sum()
    }
}

fn to_dirs(input: &[String]) -> impl Iterator<Item=CDir> + '_ {
    input.iter()
        .flat_map(|i| i.chars().map(|c| match c {
            '^' => CDir::N,
            'v' => CDir::S,
            '>' => CDir::E,
            '<' => CDir::W,
            _ => panic!(),
        }))
}

fn part1(input: &[Vec<String>]) -> i64 {
    let mut grid: Grid<Cell> = Grid::from_input(&input[0], Cell::Empty, 0);
    let mut robot = grid.find_robot();
    for dir in to_dirs(&input[1]) {
        robot = grid.move_robot(robot, dir);
    }
    grid.sum_gps()
}

fn part2(input: &[Vec<String>]) -> i64 {
    let mut grid: Grid<Cell> = Grid::scaled_from_input(&input[0]);
    let mut robot = grid.find_robot();
    for dir in to_dirs(&input[1]) {
        robot = grid.move_robot(robot, dir);
    }
    grid.sum_gps()
}

fn main() {
    let input: Vec<Vec<String>> = read_grouped_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::grouped_test_input;

    #[test]
    fn day15_test() {
        let input: Vec<Vec<String>> = grouped_test_input(include_str!("day15.testinput"));
        assert_eq!(part1(&input), 10092);
        assert_eq!(part2(&input), 9021);
    }
}
