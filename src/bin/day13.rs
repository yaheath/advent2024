use lazy_static::lazy_static;
use regex::Regex;
use std::vec::Vec;
use ya_advent_lib::coords::Coord2D;
use ya_advent_lib::read::read_grouped_input;

struct Game {
    button_a: Coord2D,
    button_b: Coord2D,
    prize: Coord2D,
}

impl Game {
    fn from_input(input: &[String]) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"X[+=](\d+), Y[+=](\d+)").unwrap();
        }
        let caps = RE.captures(&input[0]).unwrap();
        let button_a = Coord2D::new(
            caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
        );
        let caps = RE.captures(&input[1]).unwrap();
        let button_b = Coord2D::new(
            caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
        );
        let caps = RE.captures(&input[2]).unwrap();
        let prize = Coord2D::new(
            caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
        );
        Game {
            button_a,
            button_b,
            prize,
        }
    }

    fn win_cost(&self, offset: i64) -> Option<i64> {
        // use Cramer's Rule to solve:
        //   a * button_a.x + b * button_b.x = prize.x
        //   a * button_a.y + b * button_b.y = prize.y
        // for a and b

        let prize = self.prize + Coord2D::new(offset, offset);
        let det = self.button_a.x * self.button_b.y - self.button_b.x * self.button_a.y;
        assert!(det != 0);
        let a = (prize.x * self.button_b.y - prize.y * self.button_b.x) / det;
        let b = (self.button_a.x * prize.y - self.button_a.y * prize.x) / det;
        if self.button_a * a + self.button_b * b == prize {
            Some(a * 3 + b)
        } else {
            None
        }
    }
}

fn part1(input: &[Game]) -> i64 {
    input.iter().filter_map(|g| g.win_cost(0)).sum()
}

fn part2(input: &[Game]) -> i64 {
    input
        .iter()
        .filter_map(|g| g.win_cost(10000000000000))
        .sum()
}

fn main() {
    let input: Vec<Game> = read_grouped_input()
        .into_iter()
        .map(|i| Game::from_input(&i))
        .collect();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::grouped_test_input;

    #[test]
    fn day13_test() {
        let input: Vec<Game> = grouped_test_input(
            "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
",
        )
        .into_iter()
        .map(|i| Game::from_input(&i))
        .collect();

        assert_eq!(part1(&input), 480);
        assert_eq!(part2(&input), 875318608908);
    }
}
