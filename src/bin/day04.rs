use itertools::Itertools;
use std::time::Instant;
use std::vec::Vec;
use ya_advent_lib::coords::Coord2D;
use ya_advent_lib::grid::Grid;
use ya_advent_lib::read::read_input;

fn mkgrid(inp: &[String]) -> Grid<char> {
    Grid::from_input(inp, '.', 1)
}

const WORD: [char; 4] = ['X', 'M', 'A', 'S'];

fn num_words_at(x: i64, y: i64, grid: &Grid<char>) -> usize {
    let center = Coord2D::new(x, y);
    Coord2D::new(0, 0)
        .neighbors8()
        .iter()
        .filter(|c| {
            WORD.iter().enumerate().all(|(n, letter)| {
                let loc = center + **c * (n as i64);
                grid.get_c(loc) == *letter
            })
        })
        .count()
}

fn part1(input: &[String]) -> usize {
    let grid = mkgrid(input);
    grid.x_bounds_orig()
        .cartesian_product(grid.y_bounds_orig())
        .map(|(x, y)| num_words_at(x, y, &grid))
        .sum()
}

fn x_mas_at(x: i64, y: i64, grid: &Grid<char>) -> bool {
    let center = Coord2D::new(x, y);
    if grid.get_c(center) != 'A' {
        return false;
    }
    let nw = center + Coord2D::new(1, 1);
    let se = center + Coord2D::new(-1, -1);
    let ne = center + Coord2D::new(1, -1);
    let sw = center + Coord2D::new(-1, 1);
    let nw = grid.get_c(nw);
    let ne = grid.get_c(ne);
    let sw = grid.get_c(sw);
    let se = grid.get_c(se);
    (nw == 'M' && se == 'S' || nw == 'S' && se == 'M')
        && (ne == 'M' && sw == 'S' || ne == 'S' && sw == 'M')
}

fn part2(input: &[String]) -> usize {
    let grid = mkgrid(input);
    grid.x_bounds_orig()
        .cartesian_product(grid.y_bounds_orig())
        .filter(|(x, y)| x_mas_at(*x, *y, &grid))
        .count()
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
    fn day04_test() {
        let input: Vec<String> = test_input(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
",
        );
        assert_eq!(part1(&input), 18);
        assert_eq!(part2(&input), 9);
    }
}
