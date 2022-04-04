use itertools::Itertools;
use std::cmp;

pub fn solve(part: u8, input: &String) -> String {
    if part == 1 {
        let sz: usize = input
            .lines()
            .map(|x| {
                let mut tok = x.split_whitespace();
                let (x1, y1) = tok.next().unwrap().split(',').collect_tuple().unwrap();
                tok.next(); // ->
                let (x2, y2) = tok.next().unwrap().split(',').collect_tuple().unwrap();
                [x1, y1, x2, y2]
                    .iter()
                    .map(|x| x.parse::<usize>().unwrap())
                    .max()
            })
            .max()
            .unwrap()
            .unwrap()
            + 1;

        // println!("sz: {}", sz);

        let mut grid = Vec::new();
        grid.resize(sz * sz, 0u8);
        let idx_of = |x, y| x + sz * y;

        for line in input.lines() {
            let mut tok = line.split_whitespace();
            let (x1, y1) = tok.next().unwrap().split(',').collect_tuple().unwrap();
            tok.next(); // ->
            let (x2, y2) = tok.next().unwrap().split(',').collect_tuple().unwrap();
            let (x1, y1, x2, y2) = [x1, y1, x2, y2]
                .iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();

            if x1 == x2 {
                let (start, stop) = (cmp::min(y1, y2), cmp::max(y1, y2));
                for y in start..=stop {
                    grid[idx_of(x1, y)] += 1;
                }
            } else if y1 == y2 {
                let (start, stop) = (cmp::min(x1, x2), cmp::max(x1, x2));
                for x in start..=stop {
                    grid[idx_of(x, y1)] += 1;
                }
            } else {
                continue;
            }
        }

        let sum_overlap = grid.iter().filter(|x| **x >= 2).count();

        format!("{}", sum_overlap)
    } else {
        let sz: usize = input
            .lines()
            .map(|x| {
                let mut tok = x.split_whitespace();
                let (x1, y1) = tok.next().unwrap().split(',').collect_tuple().unwrap();
                tok.next(); // ->
                let (x2, y2) = tok.next().unwrap().split(',').collect_tuple().unwrap();
                [x1, y1, x2, y2]
                    .iter()
                    .map(|x| x.parse::<usize>().unwrap())
                    .max()
            })
            .max()
            .unwrap()
            .unwrap()
            + 1;

        // println!("sz: {}", sz);

        let mut grid = Vec::new();
        grid.resize(sz * sz, 0u16);
        let idx_of = |x, y| x + sz * y;

        for line in input.lines() {
            let mut tok = line.split_whitespace();
            let (x1, y1) = tok.next().unwrap().split(',').collect_tuple().unwrap();
            tok.next(); // ->
            let (x2, y2) = tok.next().unwrap().split(',').collect_tuple().unwrap();
            let (x1, y1, x2, y2) = [x1, y1, x2, y2]
                .iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();

            if x1 == x2 {
                let (start, stop) = (cmp::min(y1, y2), cmp::max(y1, y2));
                for y in start..=stop {
                    grid[idx_of(x1, y)] += 1;
                }
            } else if y1 == y2 {
                let (start, stop) = (cmp::min(x1, x2), cmp::max(x1, x2));
                for x in start..=stop {
                    grid[idx_of(x, y1)] += 1;
                }
            } else if (x2 > x1 && y2 > y1) || (x2 < x1 && y2 < y1) {
                // goes down right
                let (x1, y1, x2, y2) = (
                    cmp::min(x1, x2),
                    cmp::min(y1, y2),
                    cmp::max(x1, x2),
                    cmp::max(y1, y2),
                );
                assert_eq!(x2 - x1, y2 - y1);
                let depth = x2 - x1;
                for step in 0..=depth {
                    grid[idx_of(x1 + step, y1 + step)] += 1;
                }
            } else if (x2 > x1 && y2 < y1) || (x2 < x1 && y2 > y1) {
                // goes down left
                let (x1, y1, x2, y2) = (
                    cmp::max(x1, x2),
                    cmp::min(y1, y2),
                    cmp::min(x1, x2),
                    cmp::max(y1, y2),
                );
                assert_eq!(x1 - x2, y2 - y1);
                let depth = x1 - x2;
                for step in 0..=depth {
                    grid[idx_of(x1 - step, y1 + step)] += 1;
                }
            }
        }

        let sum_overlap = grid.iter().filter(|x| **x >= 2).count();

        format!("{}", sum_overlap)
    }
}
