use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::time::Instant;
use std::vec::Vec;
use ya_advent_lib::read::read_sectioned_input;

struct Initial {
    signal: String,
    value: bool,
}

impl FromStr for Initial {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\w+): (\d+)").unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            let signal: String = caps.get(1).unwrap().as_str().into();
            let value: bool = caps.get(2).unwrap().as_str().starts_with('1');
            Ok(Initial { signal, value })
        } else {
            Err(())
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Operator {
    And,
    Xor,
    Or,
}

impl Operator {
    fn operate(&self, a: bool, b: bool) -> bool {
        match self {
            Operator::And => a && b,
            Operator::Or => a || b,
            Operator::Xor => a != b,
        }
    }
}

impl FromStr for Operator {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
struct Gate {
    op: Operator,
    a: String,
    b: String,
    out: String,
}

impl FromStr for Gate {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut itr = s.split_whitespace();
        let a = itr.next().unwrap().to_string();
        let op = itr.next().unwrap().parse::<Operator>().unwrap();
        let b = itr.next().unwrap().to_string();
        itr.next();
        let out = itr.next().unwrap().to_string();
        Ok(Self { op, a, b, out })
    }
}

struct GateNet {
    gates: Vec<Gate>,
    signals: HashMap<String, Option<bool>>,
    signal_map: HashMap<String, HashSet<usize>>,
}

impl GateNet {
    fn new(initial: &[Initial], gates: &[Gate]) -> Self {
        let gates = gates.to_vec();
        let mut signal_map = HashMap::new();
        let mut signals = HashMap::from_iter(initial.iter().map(|i| {
            signal_map.insert(i.signal.clone(), HashSet::new());
            (i.signal.clone(), Some(i.value))
        }));
        gates.iter().for_each(|g| {
            signals.entry(g.a.clone()).or_insert(None);
            signals.entry(g.b.clone()).or_insert(None);
            signals.entry(g.out.clone()).or_insert(None);
            signal_map
                .entry(g.out.clone())
                .and_modify(|_| {
                    panic!();
                })
                .or_insert(HashSet::new());
        });
        gates.iter().enumerate().for_each(|(idx, g)| {
            signal_map.entry(g.a.clone()).and_modify(|e| {
                e.insert(idx);
            });
            signal_map.entry(g.b.clone()).and_modify(|e| {
                e.insert(idx);
            });
        });
        Self {
            gates,
            signals,
            signal_map,
        }
    }

    fn resolve(&mut self) {
        loop {
            let mut changed = false;
            self.gates.iter().for_each(|g| {
                if self.signals[&g.out].is_none() {
                    let a = self.signals[&g.a];
                    let b = self.signals[&g.b];
                    if a.is_some() && b.is_some() {
                        self.signals.insert(
                            g.out.clone(),
                            Some(g.op.operate(*a.as_ref().unwrap(), *b.as_ref().unwrap())),
                        );
                        changed = true;
                    }
                }
            });
            if !changed {
                return;
            }
        }
    }

    fn z_val(&mut self) -> usize {
        let mut out = 0;
        for i in 0.. {
            let Some(v) = self.signals.get(&format!("z{i:02}")) else {
                break;
            };
            if v.unwrap() {
                out |= 1 << i;
            }
        }
        out
    }

    fn find_bad_connections(&self) -> Vec<String> {
        let mut bad_connections = Vec::new();

        for g in self.gates.iter() {
            let is_output = g.out.starts_with('z');

            let is_a_input = g.a.starts_with('x') || g.a.starts_with('y');
            let is_input_a_0 = is_a_input && g.a.ends_with("00");
            let is_b_input = g.b.starts_with('x') || g.b.starts_with('y');
            let is_input_b_0 = is_b_input && g.b.ends_with("00");
            let has_an_input = is_a_input || is_b_input;
            let has_input_0 = is_input_a_0 || is_input_b_0;

            let (output_to_xor, output_to_and, output_to_or) =
                if let Some(next) = self.signal_map.get(&g.out) {
                    (
                        next.iter()
                            .any(|node| self.gates[*node].op == Operator::Xor),
                        next.iter()
                            .any(|node| self.gates[*node].op == Operator::And),
                        next.iter().any(|node| self.gates[*node].op == Operator::Or),
                    )
                } else {
                    (false, false, false)
                };

            // Look for connections that don't fit the expected pattern (by gate type)
            // Note: assumes the bad connections are always to different gate types,
            // which appears to be true for my input.
            if !match g.op {
                Operator::Xor => {
                    !has_an_input && is_output
                        || has_an_input && output_to_xor
                        || has_input_0 && is_output
                }
                Operator::Or => g.out == "z45" || (output_to_xor && output_to_and),
                Operator::And => output_to_or || has_input_0,
            } {
                bad_connections.push(g.out.clone());
            }
        }

        bad_connections.sort();
        bad_connections
    }
}

fn part1(initial: &[Initial], gates: &[Gate]) -> usize {
    let mut net = GateNet::new(initial, gates);
    net.resolve();
    net.z_val()
}

fn part2(initial: &[Initial], gates: &[Gate]) -> String {
    let net = GateNet::new(initial, gates);
    net.find_bad_connections().join(",")
}

fn main() {
    let input: (Vec<Initial>, Vec<Gate>) = read_sectioned_input();
    let start = Instant::now();
    let part1 = part1(&input.0, &input.1);
    let duration = start.elapsed();
    println!("Part 1: {part1} ({duration:?})");
    let start = Instant::now();
    let part2 = part2(&input.0, &input.1);
    let duration = start.elapsed();
    println!("Part 2: {part2} ({duration:?})");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::sectioned_test_input;

    #[test]
    fn day24_test() {
        let input: (Vec<Initial>, Vec<Gate>) = sectioned_test_input(
            "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
",
        );
        assert_eq!(part1(&input.0, &input.1), 4);

        let input: (Vec<Initial>, Vec<Gate>) =
            sectioned_test_input(include_str!("day24.testinput"));
        assert_eq!(part1(&input.0, &input.1), 2024);
    }
}
