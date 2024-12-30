use std::collections::HashSet;
use std::time::Instant;
use std::vec::Vec;
use ya_advent_lib::algorithm::a_star_ex;
use ya_advent_lib::coords::Coord2D;
use ya_advent_lib::read::read_input;

fn part1(input: &[Coord2D], is_test: bool) -> i64 {
    let n_coords = if is_test { 12 } else { 1024 };
    let dest = if is_test {
        Coord2D::new(6, 6)
    } else {
        Coord2D::new(70, 70)
    };
    search(input, n_coords, dest).unwrap().0
}

fn search(input: &[Coord2D], n_coords: usize, dest: Coord2D) -> Option<(i64, HashSet<Coord2D>)> {
    let bad_coords: HashSet<Coord2D> = HashSet::from_iter(input.iter().take(n_coords).copied());
    a_star_ex(
        Coord2D::new(0, 0),
        |c| *c == dest,
        |c| {
            c.neighbors4()
                .iter()
                .filter(|n| !bad_coords.contains(*n))
                .filter(|n| n.x >= 0 && n.x <= dest.x && n.y >= 0 && n.y <= dest.y)
                .map(|n| (*n, 1))
                .collect::<Vec<_>>()
        },
        |c| c.mdist_to(&dest),
        false,
    )
    .map(|(cost, path)| {
        let mut set = HashSet::new();
        let mut tail = dest;
        while let Some((_, next)) = path.get(&tail) {
            set.insert(tail);
            tail = *next.iter().next().unwrap();
        }
        (cost, set)
    })
}

fn part2(input: &[Coord2D], is_test: bool) -> Coord2D {
    let dest = if is_test {
        Coord2D::new(6, 6)
    } else {
        Coord2D::new(70, 70)
    };
    let start_n = if is_test { 12 } else { 1024 };
    let (_, mut path) = search(input, start_n, dest).unwrap();
    for n in start_n.. {
        if path.contains(&input[n]) {
            let next = search(input, n + 1, dest);
            if let Some((_, nextpath)) = next {
                path = nextpath;
            } else {
                return input[n];
            }
        }
    }
    panic!();
}

fn main() {
    let input: Vec<Coord2D> = read_input();

    let start = Instant::now();
    let part1 = part1(&input, false);
    let duration = start.elapsed();
    println!("Part 1: {part1} ({duration:?})");
    let start = Instant::now();
    let part2 = part2(&input, false);
    let duration = start.elapsed();
    println!("Part 2: {},{} ({duration:?})", part2.x, part2.y);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day18_test() {
        let input: Vec<Coord2D> = test_input(include_str!("day18.testinput"));
        assert_eq!(part1(&input, true), 22);
        assert_eq!(part2(&input, true), Coord2D::new(6, 1));
    }
}
