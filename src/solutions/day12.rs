use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(part: u8, input: &String) -> String {
    if part == 1 {
        let mut nodes = HashMap::new();
        for line in input.lines() {
            let (from, to) = line.split('-').collect_tuple().unwrap();
            let entry = nodes.entry(from).or_insert(HashSet::new());
            entry.insert(to);
            let entry = nodes.entry(to).or_insert(HashSet::new());
            entry.insert(from);
        }

        fn upper<'a>(node: &'a str) -> bool {
            node.chars().all(char::is_uppercase)
        }
        fn visit<'a>(
            nodes: &HashMap<&'a str, HashSet<&'a str>>,
            from: &'a str,
            path: &Vec<&'a str>,
            paths: &mut Vec<Vec<&'a str>>,
        ) {
            for next in nodes.get(from).unwrap() {
                if !upper(next) && path.contains(next) {
                    continue;
                }
                let mut new = path.clone();
                new.push(next);
                if *next == "end" {
                    paths.push(new);
                } else {
                    visit(nodes, next, &new, paths);
                }
            }
        }

        let mut paths = Vec::new();
        visit(&nodes, "start", &vec!["start"], &mut paths);

        format!("{}", paths.len())
    } else {
        let mut nodes = HashMap::new();
        for line in input.lines() {
            let (from, to) = line.split('-').collect_tuple().unwrap();
            let entry = nodes.entry(from).or_insert(HashSet::new());
            entry.insert(to);
            let entry = nodes.entry(to).or_insert(HashSet::new());
            entry.insert(from);
        }
        let nodes = nodes;

        fn upper<'a>(node: &'a str) -> bool {
            node.chars().all(char::is_uppercase)
        }

        let n = nodes.len();
        // technically only lower nodes indices + one extra flag are required, so 63 lower
        // nodes can be used using u64 (or use u128 with 127 lower nodes).
        // we code naive method here: all indices (lower + upper) + extra indices, so only 32
        assert!(n < 32);
        let idx_of = |needle: &str| {
            nodes
                .iter()
                .enumerate()
                .find_map(|(i, (f, _))| if *f == needle { Some(i) } else { None })
                .unwrap()
        };

        type Path = usize;
        let mut nnodes = Vec::with_capacity(n);
        nnodes.resize(n, Path::default());
        let mut upper_mask = Path::default();
        let mut lower_mask = Path::default();
        for (idx, (from, tos)) in nodes.iter().enumerate() {
            if upper(from) {
                upper_mask |= 1 << idx;
                upper_mask |= 1 << (idx + 32);
            } else {
                lower_mask |= 1 << idx;
                lower_mask |= 1 << (idx + 32);
            }
            for to in tos {
                nnodes[idx] |= 1 << idx_of(to);
            }
        }

        fn count_dup_small(path: Path, lower_mask: usize) -> u32 {
            ((path & lower_mask) & 0xFFFFFFFF00000000).count_ones()
        }
        fn visit(
            nnodes: &Vec<Path>,
            from_idx: usize,
            start_idx: usize,
            end_idx: usize,
            upper_mask: usize,
            lower_mask: usize,
            path: Path,
            paths: &mut usize,
        ) {
            for next_idx in 0..32 {
                let next_flag = 1 << next_idx;
                if nnodes[from_idx] & next_flag != 0 {
                    if next_idx == start_idx {
                        continue;
                    }
                    let mut new_path = path;
                    if next_flag & upper_mask == 0 {
                        if new_path & next_flag != 0 {
                            if new_path & next_flag << 32 != 0 {
                                continue;
                            }
                            new_path |= next_flag << 32;
                        } else {
                            new_path |= next_flag;
                        }
                        if count_dup_small(new_path, lower_mask) > 1 {
                            continue;
                        }
                    } else {
                        new_path |= next_flag;
                    }
                    if next_idx == end_idx {
                        *paths += 1;
                    } else {
                        visit(
                            nnodes, next_idx, start_idx, end_idx, upper_mask, lower_mask, new_path,
                            paths,
                        );
                    }
                }
            }
        }

        let start_idx = idx_of("start");
        let end_idx = idx_of("end");
        let mut paths = 0;
        visit(
            &nnodes,
            start_idx,
            start_idx,
            end_idx,
            upper_mask,
            lower_mask,
            1 << start_idx,
            &mut paths,
        );

        format!("{}", paths)
    }
}
