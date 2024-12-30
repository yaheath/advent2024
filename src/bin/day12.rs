use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;
use std::vec::Vec;
use ya_advent_lib::coords::Coord2D;
use ya_advent_lib::grid::Grid;
use ya_advent_lib::read::read_input;

fn bothparts(input: &[String]) -> (usize, usize) {
    let grid: Grid<char> = Grid::from_input(input, '.', 1);
    let mut total = 0usize;
    let mut total2 = 0usize;
    let mut mapped: HashSet<Coord2D> = HashSet::new();
    grid.x_bounds_orig()
        .cartesian_product(grid.y_bounds_orig())
        .map(|(x, y)| Coord2D::new(x, y))
        .for_each(|cell| {
            if !mapped.contains(&cell) {
                let mut area = 0usize;
                let mut perim = 0usize;
                let typ = grid.get_c(cell);
                let mut queue: VecDeque<Coord2D> = VecDeque::new();
                let mut h_edges: HashMap<(i64, i64), HashSet<i64>> = HashMap::new();
                let mut v_edges: HashMap<(i64, i64), HashSet<i64>> = HashMap::new();
                mapped.insert(cell);
                queue.push_back(cell);
                while let Some(c) = queue.pop_front() {
                    area += 1;
                    for n in c.neighbors4() {
                        if grid.get_c(n) != typ {
                            perim += 1;
                            if n.x == c.x {
                                h_edges
                                    .entry((c.y, n.y))
                                    .and_modify(|edg| {
                                        edg.insert(c.x);
                                    })
                                    .or_insert(HashSet::from_iter([c.x]));
                            } else {
                                v_edges
                                    .entry((c.x, n.x))
                                    .and_modify(|edg| {
                                        edg.insert(c.y);
                                    })
                                    .or_insert(HashSet::from_iter([c.y]));
                            }
                        } else if !mapped.contains(&n) {
                            mapped.insert(n);
                            queue.push_back(n);
                        }
                    }
                }
                total += area * perim;

                let h_perim: usize = h_edges
                    .into_values()
                    .map(|set| {
                        set.into_iter()
                            .sorted_unstable()
                            .coalesce(|a, b| if a + 1 == b { Ok(b) } else { Err((a, b)) })
                            .count()
                    })
                    .sum();
                let v_perim: usize = v_edges
                    .into_values()
                    .map(|set| {
                        set.into_iter()
                            .sorted_unstable()
                            .coalesce(|a, b| if a + 1 == b { Ok(b) } else { Err((a, b)) })
                            .count()
                    })
                    .sum();
                total2 += area * (h_perim + v_perim);
            }
        });
    (total, total2)
}

fn main() {
    let input: Vec<String> = read_input();
    let start = Instant::now();
    let (part1, part2) = bothparts(&input);
    let duration = start.elapsed();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("({duration:?})");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day12_test() {
        let input: Vec<String> = test_input(
            "AAAA
BBCD
BBCC
EEEC
",
        );
        let (part1, part2) = bothparts(&input);
        assert_eq!(part1, 140);
        assert_eq!(part2, 80);

        let input: Vec<String> = test_input(
            "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
",
        );
        let (part1, part2) = bothparts(&input);
        assert_eq!(part1, 772);
        assert_eq!(part2, 436);

        let input: Vec<String> = test_input(
            "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
",
        );
        let (part1, part2) = bothparts(&input);
        assert_eq!(part1, 1930);
        assert_eq!(part2, 1206);

        let input: Vec<String> = test_input(
            "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
",
        );
        let (_, part2) = bothparts(&input);
        assert_eq!(part2, 236);

        let input: Vec<String> = test_input(
            "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
",
        );
        let (_, part2) = bothparts(&input);
        assert_eq!(part2, 368);
    }
}
