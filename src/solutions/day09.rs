use std::collections::HashSet;

pub fn solve(part: u8, input: &String) -> String {
    if part == 1 {
        let w = input.lines().next().unwrap().chars().count();
        let h = input.lines().count();

        let mut grid = Vec::new();
        grid.resize(w * h, 0u8);
        let idx_of = |x, y| x + w * y;

        let mut idx = 0;
        for line in input.lines() {
            for c in line.chars() {
                grid[idx] = c.to_digit(10).unwrap() as u8;
                idx += 1;
            }
        }
        assert_eq!(idx, w * h);
        let grid = grid;

        let mut risks: u64 = 0;
        for row in 0..h {
            for col in 0..w {
                let v = grid[idx_of(col, row)];
                // first memory locality checks, so left & right
                if col > 0 && grid[idx_of(col - 1, row)] <= v {
                    continue;
                }
                if col < w - 1 && grid[idx_of(col + 1, row)] <= v {
                    continue;
                }
                // then far away, so up & down
                if row > 0 && grid[idx_of(col, row - 1)] <= v {
                    continue;
                }
                if row < h - 1 && grid[idx_of(col, row + 1)] <= v {
                    continue;
                }
                risks += (v + 1) as u64;
            }
        }

        format!("{}", risks)
    } else {
        let w = input.lines().next().unwrap().chars().count();
        let h = input.lines().count();

        let mut grid = Vec::new();
        let sz = w * h;
        grid.resize(sz, 0u8);
        let idx_of = |x, y| x + w * y;

        let mut idx = 0;
        for line in input.lines() {
            for c in line.chars() {
                let d = c.to_digit(10).unwrap() as u8;
                grid[idx] = if d == 9 { 255u8 } else { d };
                idx += 1;
            }
        }
        assert_eq!(idx, sz);

        // find lowest points
        let mut lowests = HashSet::new();
        for row in 0..h {
            for col in 0..w {
                let v = grid[idx_of(col, row)];
                // first memory locality checks, so left & right
                if col > 0 && grid[idx_of(col - 1, row)] <= v {
                    continue;
                }
                if col < w - 1 && grid[idx_of(col + 1, row)] <= v {
                    continue;
                }
                // then far away, so up & down
                if row > 0 && grid[idx_of(col, row - 1)] <= v {
                    continue;
                }
                if row < h - 1 && grid[idx_of(col, row + 1)] <= v {
                    continue;
                }
                lowests.insert(idx_of(col, row));
            }
        }

        // clear
        for idx in 0..sz {
            if grid[idx] != 255 {
                grid[idx] = 0;
            }
        }

        // flag basins
        assert!(lowests.len() < 256 - 2); // u8 minus 2 flags
        for (lowid, lowest) in lowests.iter().enumerate() {
            grid[*lowest] = (lowid + 1) as u8;
        }

        // extend
        loop {
            let mut modified = 0;
            for row in 0..h {
                for col in 0..w {
                    let v = grid[idx_of(col, row)];
                    if v != 0 {
                        continue;
                    }
                    let mut bassin = 0;
                    if col > 0 {
                        let v = grid[idx_of(col - 1, row)];
                        if v != 255 && v != 0 {
                            bassin = v;
                        }
                    }
                    if bassin == 0 && col < w - 1 {
                        let v = grid[idx_of(col + 1, row)];
                        if v != 255 && v != 0 {
                            bassin = v;
                        }
                    }
                    if bassin == 0 && row > 0 {
                        let v = grid[idx_of(col, row - 1)];
                        if v != 255 && v != 0 {
                            bassin = v;
                        }
                    }
                    if bassin == 0 && row < h - 1 {
                        let v = grid[idx_of(col, row + 1)];
                        if v != 255 && v != 0 {
                            bassin = v;
                        }
                    }
                    if bassin != 0 {
                        grid[idx_of(col, row)] = bassin;
                        modified += 1;
                    }
                }
            }
            if modified == 0 {
                break;
            }
        }

        let display = false;
        if display {
            for row in 0..h {
                for col in 0..w {
                    let v = grid[idx_of(col, row)];
                    if v == 255 {
                        // print!(" XXX");
                    } else {
                        // print!(" {:3}", v);
                    }
                }
                // println!("");
            }
        }

        // count
        let mut acc = Vec::new();
        acc.resize(lowests.len() + 1, 0usize);
        for idx in 0..sz {
            let v = grid[idx];
            assert!(v != 0);
            if v != 255 {
                acc[v as usize] += 1;
            }
        }
        assert_eq!(acc[0], 0);
        acc.sort();

        let n = acc.len();
        assert!(n > 3 + 1);
        let product_largest = acc[n - 1] * acc[n - 2] * acc[n - 3];

        format!("{}", product_largest)
    }
}
