use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;
use std::time::Instant;
use std::vec::Vec;
use ya_advent_lib::coords::Coord2D;
use ya_advent_lib::grid::Grid;
use ya_advent_lib::read::read_input;

#[derive(Clone)]
struct Robot {
    pos: Coord2D,
    vel: Coord2D,
}

impl FromStr for Robot {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        let px = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let py = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let vx = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let vy = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();
        let pos = Coord2D::new(px, py);
        let vel = Coord2D::new(vx, vy);
        Ok(Robot { pos, vel })
    }
}

fn part1(input: &[Robot], is_example: bool) -> i64 {
    let area = if is_example {
        Coord2D::new(11, 7)
    } else {
        Coord2D::new(101, 103)
    };
    let mid = Coord2D::new(area.x / 2, area.y / 2);
    let mut q1 = 0i64;
    let mut q2 = 0i64;
    let mut q3 = 0i64;
    let mut q4 = 0i64;
    input
        .iter()
        .map(|r| {
            Coord2D::new(
                (r.pos.x + 100 * r.vel.x).rem_euclid(area.x),
                (r.pos.y + 100 * r.vel.y).rem_euclid(area.y),
            )
        })
        .for_each(|p| {
            if p.x < mid.x && p.y < mid.y {
                q1 += 1
            };
            if p.x < mid.x && p.y > mid.y {
                q2 += 1
            };
            if p.x > mid.x && p.y < mid.y {
                q3 += 1
            };
            if p.x > mid.x && p.y > mid.y {
                q4 += 1
            };
        });
    q1 * q2 * q3 * q4
}

#[allow(dead_code)]
fn print_bots(robots: &[Robot]) {
    let mut grid: Grid<char> = Grid::new(0, 0, 100, 102, '.');
    robots.iter().for_each(|r| grid.set_c(r.pos, '#'));
    grid.print();
}

fn part2(input: &[Robot]) -> usize {
    let area = Coord2D::new(101, 103);

    // If more than half of the robots concentrate in the center
    // ninth of the area, that's probably what we're looking for
    let center_range = 33..68;
    let center_count_thresh = input.len() / 2;

    let mut step = 0;
    let mut robots = Vec::from(input);
    //print!("\x1b[H\x1b[2J");
    //print_bots(&robots);
    loop {
        step += 1;
        for r in robots.iter_mut() {
            r.pos.x = (r.pos.x + r.vel.x).rem_euclid(area.x);
            r.pos.y = (r.pos.y + r.vel.y).rem_euclid(area.y);
        }
        let center_count = robots
            .iter()
            .filter(|r| center_range.contains(&r.pos.x) && center_range.contains(&r.pos.y))
            .count();
        if center_count > center_count_thresh {
            //print_bots(&robots);
            return step;
        }
    }
}

fn main() {
    let input: Vec<Robot> = read_input();
    let start = Instant::now();
    let part1 = part1(&input, false);
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
    fn day14_test() {
        let input: Vec<Robot> = test_input(
            "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
",
        );
        assert_eq!(part1(&input, true), 12);
    }
}
