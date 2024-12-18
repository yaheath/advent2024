use itertools::Itertools;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

#[derive(Clone)]
struct VM {
    pc: usize,
    mem: Vec<u8>,
    a: u64,
    b: u64,
    c: u64,
}

impl VM {
    fn from_input(input: &[String]) -> Self {
        let a = input[0].split_once(": ").unwrap().1.parse::<u64>().unwrap();
        let b = input[1].split_once(": ").unwrap().1.parse::<u64>().unwrap();
        let c = input[2].split_once(": ").unwrap().1.parse::<u64>().unwrap();
        let mem = input[4]
            .split_once(": ")
            .unwrap()
            .1
            .split(',')
            .map(|s| s.parse::<u8>().unwrap())
            .collect::<Vec<_>>();
        Self {
            pc: 0,
            mem,
            a,
            b,
            c,
        }
    }

    fn combo(&self, op: u8) -> u64 {
        match op {
            0..4 => op as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!(),
        }
    }
    fn run(&mut self) -> Vec<u8> {
        let mut out = Vec::new();
        while self.pc < self.mem.len() - 1 {
            let opcode = self.mem[self.pc];
            let operand = self.mem[self.pc + 1];
            match opcode {
                // adv
                0 => {
                    let num = self.a;
                    let dem = 2u64.pow(self.combo(operand) as u32);
                    self.a = num / dem;
                }
                // bxl
                1 => {
                    self.b ^= operand as u64;
                }
                // bst
                2 => {
                    self.b = self.combo(operand) & 0x7;
                }
                // jnz
                3 => {
                    if self.a != 0 {
                        self.pc = operand as usize;
                        continue;
                    }
                }
                // bxc
                4 => {
                    self.b ^= self.c;
                }
                // out
                5 => {
                    out.push((self.combo(operand) & 0x7) as u8);
                }
                // bdv
                6 => {
                    let num = self.a;
                    let dem = 2u64.pow(self.combo(operand) as u32);
                    self.b = num / dem;
                }
                // cdv
                7 => {
                    let num = self.a;
                    let dem = 2u64.pow(self.combo(operand) as u32);
                    self.c = num / dem;
                }
                _ => panic!(),
            }
            self.pc += 2;
        }
        out
    }
}

fn part1(input: &[String]) -> String {
    let mut vm = VM::from_input(input);
    let out = vm.run();
    out.iter().join(",")
}

fn search(vm: &VM, tgt: &[u8], a: u64) -> Option<u64> {
    let val = tgt[0];
    for b in 0..8 {
        let mut nvm = vm.clone();
        nvm.a = a | b;
        let res = nvm.run();
        if !res.is_empty() && res[0] == val {
            if tgt.len() == 1 {
                return Some(a | b);
            }
            let r = search(vm, &tgt[1..], (a | b) << 3);
            if r.is_some() {
                return r;
            }
        }
    }
    None
}

fn part2(input: &[String]) -> u64 {
    let vm = VM::from_input(input);
    let mut tgt: Vec<u8> = vm.mem.clone();
    tgt.reverse();
    search(&vm, &tgt, 0).unwrap()
}

fn main() {
    let input: Vec<String> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day17_test() {
        let input: Vec<String> = test_input(
            "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
",
        );
        assert_eq!(part1(&input), "4,6,3,5,6,3,5,2,1,0");

        let input: Vec<String> = test_input(
            "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
",
        );
        assert_eq!(part2(&input), 117440);
    }
}
