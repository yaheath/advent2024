use std::collections::HashSet;
use std::str::FromStr;
use std::time::Instant;
use std::vec::Vec;
use topological_sort::TopologicalSort;
use ya_advent_lib::read::read_sectioned_input;

struct OrderRule {
    earlier: usize,
    later: usize,
}

impl FromStr for OrderRule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('|').unwrap();
        let earlier = a.parse::<usize>().unwrap();
        let later = b.parse::<usize>().unwrap();
        Ok(OrderRule { earlier, later })
    }
}

struct PageList(Vec<usize>);

impl FromStr for PageList {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PageList(
            s.split(',').map(|n| n.parse::<usize>().unwrap()).collect(),
        ))
    }
}

struct Rules {
    rules: HashSet<(usize, usize)>,
}

impl Rules {
    fn from_orders(orders: &[OrderRule]) -> Self {
        let rules: HashSet<(usize, usize)> = orders.iter().map(|o| (o.earlier, o.later)).collect();
        Self { rules }
    }

    fn sort_nodes(&self, nodes: &[usize]) -> Vec<usize> {
        let nodeset: HashSet<usize> = HashSet::from_iter(nodes.iter().copied());
        let mut topo: TopologicalSort<usize> = TopologicalSort::new();
        self.rules
            .iter()
            .filter(|o| nodeset.contains(&o.0) && nodeset.contains(&o.1))
            .for_each(|o| topo.add_dependency(o.0, o.1));
        topo.collect()
    }
}

fn bothparts(orders: &[OrderRule], pages: &[PageList]) -> (usize, usize) {
    let rules = Rules::from_orders(orders);
    pages
        .iter()
        .map(|lst| {
            let sorted = rules.sort_nodes(&lst.0);
            if sorted == lst.0 {
                (lst.0[lst.0.len() / 2], 0)
            } else {
                (0, sorted[sorted.len() / 2])
            }
        })
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1))
}

fn main() {
    let input: (Vec<OrderRule>, Vec<PageList>) = read_sectioned_input();
    let start = Instant::now();
    let (part1, part2) = bothparts(&input.0, &input.1);
    let duration = start.elapsed();
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    println!("({duration:?})");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::sectioned_test_input;

    #[test]
    fn day05_test() {
        let input: (Vec<OrderRule>, Vec<PageList>) =
            sectioned_test_input(include_str!("day05.testinput"));
        let (part1, part2) = bothparts(&input.0, &input.1);
        assert_eq!(part1, 143);
        assert_eq!(part2, 123);
    }
}
