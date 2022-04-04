use itertools::Itertools;
use std::cmp;

pub fn solve(part: u8, input: &String) -> String {
    if part == 1 {
        let points = input
            .lines()
            .filter_map(|x| {
                if x.contains(',') {
                    Some(
                        x.split(',')
                            .map(|x| x.parse::<usize>().unwrap())
                            .collect_tuple::<(_, _)>()
                            .unwrap(),
                    )
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let (w, h) = points.iter().fold((0, 0), |(w, h), (col, row)| {
            (cmp::max(w, *col), cmp::max(h, *row))
        });
        let (w, h) = (w + 1, h + 1);

        let mut grid = Vec::new();
        grid.resize(w * h, 0u8);
        let idx_of = |x, y| x + w * y;
        let pos_of = |idx| (idx % w, idx / w);
        for (col, row) in points {
            grid[idx_of(col, row)] = 1;
        }

        let (what, off) = input
            .lines()
            .find_map(|x| {
                if x.starts_with("fold along ") {
                    let what = x.chars().skip(11).next().unwrap();
                    let off = x
                        .split('=')
                        .skip(1)
                        .map(|x| x.parse::<usize>().unwrap())
                        .next()
                        .unwrap();
                    Some((what, off))
                } else {
                    None
                }
            })
            .unwrap();

        let (mut visible_w, mut visible_h) = (w, h);
        match what {
            'x' => {
                for row in 0..h {
                    for col in (off + 1)..w {
                        let proj = off - (col - off);
                        grid[idx_of(proj, row)] += grid[idx_of(col, row)];
                        if proj == 0 {
                            break;
                        }
                    }
                }
                visible_w = off;
            }
            'y' => {
                for row in (off + 1)..h {
                    let proj = off - (row - off);
                    for col in 0..w {
                        grid[idx_of(col, proj)] += grid[idx_of(col, row)];
                    }
                    if proj == 0 {
                        break;
                    }
                }
                visible_h = off;
            }
            _ => panic!("invalid axis"),
        }

        let visible_dots = grid
            .iter()
            .enumerate()
            .map(|(idx, v)| {
                let (col, row) = pos_of(idx);
                (col, row, v)
            })
            .filter(|(col, row, v)| *col < visible_w && *row < visible_h && *v > &0)
            .count();

        format!("{}", visible_dots)
    } else {
        let points = input
            .lines()
            .filter_map(|x| {
                if x.contains(',') {
                    Some(
                        x.split(',')
                            .map(|x| x.parse::<usize>().unwrap())
                            .collect_tuple::<(_, _)>()
                            .unwrap(),
                    )
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let (w, h) = points.iter().fold((0, 0), |(w, h), (col, row)| {
            (cmp::max(w, *col), cmp::max(h, *row))
        });
        let (w, h) = (w + 1, h + 1);

        let mut grid = Vec::new();
        grid.resize(w * h, 0u8);
        let idx_of = |x, y| x + w * y;
        for (col, row) in points {
            grid[idx_of(col, row)] = 1;
        }

        let (mut visible_w, mut visible_h) = (w, h);
        for (what, off) in input.lines().filter_map(|x| {
            if x.starts_with("fold along ") {
                let what = x.chars().skip(11).next().unwrap();
                let off = x
                    .split('=')
                    .skip(1)
                    .map(|x| x.parse::<usize>().unwrap())
                    .next()
                    .unwrap();
                Some((what, off))
            } else {
                None
            }
        }) {
            match what {
                'x' => {
                    for row in 0..visible_h {
                        for col in (off + 1)..visible_w {
                            let proj = off - (col - off);
                            grid[idx_of(proj, row)] += grid[idx_of(col, row)];
                            if proj == 0 {
                                break;
                            }
                        }
                    }
                    visible_w = off;
                }
                'y' => {
                    for row in (off + 1)..visible_h {
                        let proj = off - (row - off);
                        for col in 0..visible_w {
                            grid[idx_of(col, proj)] += grid[idx_of(col, row)];
                        }
                        if proj == 0 {
                            break;
                        }
                    }
                    visible_h = off;
                }
                _ => panic!("invalid axis"),
            }
        }

        let mut res = String::from("\n");
        for row in 0..visible_h {
            for col in 0..visible_w {
                if grid[idx_of(col, row)] > 0 {
                    res.push('X');
                } else {
                    res.push(' ');
                }
            }
            res.push('\n');
        }

        res
    }
}
