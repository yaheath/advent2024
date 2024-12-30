use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::time::Instant;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

type Node = [char; 2];

#[derive(Clone, Copy)]
struct Input {
    node1: Node,
    node2: Node,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut c = s.chars();
        let node1 = [c.next().unwrap(), c.next().unwrap()];
        c.next();
        let node2 = [c.next().unwrap(), c.next().unwrap()];
        Ok(Input { node1, node2 })
    }
}

struct Network {
    graph: HashMap<Node, HashSet<Node>>,
    pairs: HashSet<(Node, Node)>,
    allnodes: HashSet<Node>,
}

impl Network {
    fn from_input(input: &[Input]) -> Self {
        let mut allnodes = HashSet::new();
        let mut pairs = HashSet::new();
        let mut graph = HashMap::new();
        input.iter().copied().for_each(|i| {
            allnodes.insert(i.node1);
            allnodes.insert(i.node2);
            if i.node1 < i.node2 {
                pairs.insert((i.node1, i.node2));
            } else {
                pairs.insert((i.node2, i.node1));
            }
            graph
                .entry(i.node1)
                .and_modify(|e: &mut HashSet<Node>| {
                    e.insert(i.node2);
                })
                .or_insert(HashSet::from_iter([i.node2]));
            graph
                .entry(i.node2)
                .and_modify(|e: &mut HashSet<Node>| {
                    e.insert(i.node1);
                })
                .or_insert(HashSet::from_iter([i.node1]));
        });
        Self {
            graph,
            allnodes,
            pairs,
        }
    }

    fn groups_of_three_t(&self) -> impl Iterator<Item = (Node, Node, Node)> + use<'_> {
        self.allnodes
            .iter()
            .combinations(3)
            .filter(|v| v[0][0] == 't' || v[1][0] == 't' || v[2][0] == 't')
            .map(|mut v| {
                v.sort();
                v
            })
            .filter(|v| {
                self.pairs.contains(&(*v[0], *v[1]))
                    && self.pairs.contains(&(*v[1], *v[2]))
                    && self.pairs.contains(&(*v[0], *v[2]))
            })
            .map(|v| (*v[0], *v[1], *v[2]))
    }

    fn subnet(&self, node: &Node) -> HashSet<Node> {
        let mut candidates = HashSet::from_iter(self.graph[node].iter().copied());
        candidates.insert(*node);

        while candidates.len() > 1 {
            let ordered: Vec<(Node, usize)> = candidates
                .iter()
                .map(|n| {
                    (
                        n,
                        self.graph[n]
                            .iter()
                            .filter(|x| candidates.contains(*x))
                            .count(),
                    )
                })
                .sorted_by_key(|(_, count)| *count)
                .map(|(n, c)| (*n, c))
                .collect();
            //println!("{ordered:?}");
            if ordered.iter().all(|(_, s)| *s == ordered[0].1) {
                break;
            } else {
                candidates.remove(&ordered[0].0);
            }
        }
        candidates
    }
}

fn part1(input: &[Input]) -> usize {
    let network = Network::from_input(input);
    network.groups_of_three_t().count()
}

fn part2(input: &[Input]) -> String {
    let network = Network::from_input(input);
    network
        .allnodes
        .iter()
        .map(|n| network.subnet(n))
        .max_by_key(|v| v.len())
        .map(|v| {
            v.iter()
                .sorted_unstable()
                .map(String::from_iter)
                .join(",")
        })
        .unwrap()
}

fn main() {
    let input: Vec<Input> = read_input();
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
    fn day23_test() {
        let input: Vec<Input> = test_input(include_str!("day23.testinput"));
        assert_eq!(part1(&input), 7);
        assert_eq!(part2(&input), "co,de,ka,ta".to_string());
    }
}
