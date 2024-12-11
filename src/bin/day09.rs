use itertools::Itertools;
use std::collections::HashMap;
use std::iter::repeat_n;
use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

type FileId = usize;

#[derive(Clone, Copy, Debug)]
enum Blocks {
    File(FileId, usize),
    Empty(usize),
}

#[derive(Clone, Debug)]
struct DiskMap(Vec<Blocks>);

impl FromStr for DiskMap {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pad = if s.len() & 1 == 1 {
            vec![0usize]
        } else {
            vec![]
        };
        let dmap = s
            .chars()
            .map(|c| c as usize - '0' as usize)
            .chain(pad)
            .tuples::<(_, _)>()
            .enumerate()
            .flat_map(|(idx, (d, e))| [Blocks::File(idx, d), Blocks::Empty(e)])
            .filter(|b| !matches!(b, Blocks::Empty(0)))
            .collect();
        Ok(DiskMap(dmap))
    }
}

impl DiskMap {
    fn checksum(&self) -> usize {
        self.0
            .iter()
            .flat_map(|b| match b {
                Blocks::File(id, n) => repeat_n(Blocks::File(*id, 1), *n),
                Blocks::Empty(n) => repeat_n(Blocks::Empty(1), *n),
            })
            .enumerate()
            .filter_map(|(idx, b)| match b {
                Blocks::File(bid, _) => Some((idx, bid)),
                _ => None,
            })
            .map(|(idx, bid)| idx * bid)
            .sum()
    }
    fn compact(&mut self) {
        let mut out = Vec::new();
        let mut fwd_idx = 0usize;
        let mut rev_idx = self.0.len() - 1;
        let mut by_id: HashMap<FileId, usize> = HashMap::new();
        while matches!(self.0[rev_idx], Blocks::Empty(_)) {
            rev_idx -= 1;
        }
        while let Blocks::File(bid, sz) = self.0[fwd_idx] {
            out.push(self.0[fwd_idx]);
            by_id.insert(bid, sz);
            fwd_idx += 1;
        }
        let Blocks::Empty(mut cur_empty_space) = self.0[fwd_idx] else {
            panic!();
        };
        let Blocks::File(mut end_blk_id, mut end_blk_len) = self.0[rev_idx] else {
            panic!();
        };
        'outer: while fwd_idx < rev_idx {
            if end_blk_len <= cur_empty_space {
                out.push(Blocks::File(end_blk_id, end_blk_len));
                by_id
                    .entry(end_blk_id)
                    .and_modify(|l| *l += end_blk_len)
                    .or_insert(end_blk_len);
                cur_empty_space -= end_blk_len;
                rev_idx -= 1;
                while matches!(self.0[rev_idx], Blocks::Empty(_)) {
                    rev_idx -= 1;
                }
                let Blocks::File(nxt_blk_id, nxt_blk_len) = self.0[rev_idx] else {
                    panic!();
                };
                end_blk_id = nxt_blk_id;
                end_blk_len = nxt_blk_len;
            } else {
                end_blk_len -= cur_empty_space;
                out.push(Blocks::File(end_blk_id, cur_empty_space));
                by_id
                    .entry(end_blk_id)
                    .and_modify(|l| *l += cur_empty_space)
                    .or_insert(cur_empty_space);
                cur_empty_space = 0;
            }
            while cur_empty_space == 0 {
                fwd_idx += 1;
                if fwd_idx >= rev_idx {
                    break 'outer;
                }
                while let Blocks::File(bid, sz) = self.0[fwd_idx] {
                    out.push(self.0[fwd_idx]);
                    by_id.entry(bid).and_modify(|l| *l += sz).or_insert(sz);
                    fwd_idx += 1;
                    if fwd_idx >= rev_idx {
                        break 'outer;
                    }
                }
                let Blocks::Empty(next_empty_space) = self.0[fwd_idx] else {
                    panic!();
                };
                cur_empty_space = next_empty_space;
            }
        }
        let Blocks::File(_, bsz) = self.0[rev_idx] else {
            panic!();
        };
        if end_blk_len > 0 && (!by_id.contains_key(&end_blk_id) || by_id[&end_blk_id] < bsz) {
            let len = by_id.get(&end_blk_id).copied().unwrap_or(0);
            out.push(Blocks::File(end_blk_id, bsz - len));
        }

        self.0 = out;
    }

    fn compact2(&mut self) {
        let mut file_id = self
            .0
            .iter()
            .rev()
            .flat_map(|b| match b {
                Blocks::File(id, _) => Some(*id),
                _ => None,
            })
            .next()
            .unwrap();
        while file_id > 0 {
            let Some(b_idx) = self
                .0
                .iter()
                .position(|b| matches!(b, Blocks::File(id, _) if *id == file_id))
            else {
                panic!();
            };
            let Blocks::File(_, bsz) = self.0[b_idx] else {
                panic!();
            };

            if let Some(e_idx) = self
                .0
                .iter()
                .position(|b| matches!(b, Blocks::Empty(n) if *n >= bsz))
            {
                if e_idx < b_idx {
                    self.0[b_idx] = Blocks::Empty(bsz);
                    let mut r = vec![Blocks::File(file_id, bsz)];
                    let Blocks::Empty(esz) = self.0[e_idx] else {
                        panic!();
                    };
                    if bsz < esz {
                        r.push(Blocks::Empty(esz - bsz));
                    }
                    self.0.splice(e_idx..e_idx + 1, r);
                }
            }

            file_id -= 1;
        }
    }
}

fn part1(input: &[DiskMap]) -> usize {
    let mut dmap = input[0].clone();
    dmap.compact();
    dmap.checksum()
}

fn part2(input: &[DiskMap]) -> usize {
    let mut dmap = input[0].clone();
    dmap.compact2();
    dmap.checksum()
}

fn main() {
    let input: Vec<DiskMap> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day09_test() {
        let input: Vec<DiskMap> = test_input("2333133121414131402");
        assert_eq!(part1(&input), 1928);
        assert_eq!(part2(&input), 2858);
    }
}
