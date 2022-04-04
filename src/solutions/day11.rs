pub fn solve(part: u8, input: &String) -> String {
    if part == 1 {
        let w = input.lines().next().unwrap().chars().count();
        let h = input.lines().count();
        assert_eq!(w, h);
        let n = w;
        let n_isize = n as isize;
        let sz = n * n;

        let mut grid = Vec::new();
        grid.resize(sz, 0i8);
        let idx_of = |x, y| x + n * y;

        let mut idx = 0;
        for line in input.lines() {
            for c in line.chars() {
                grid[idx] = c.to_digit(10).unwrap() as i8;
                idx += 1;
            }
        }
        assert_eq!(idx, sz);

        let mut flashes = 0u64;
        for _step in 0..100 {
            for idx in 0..sz {
                grid[idx] += 1;
            }

            loop {
                let flashes_before = flashes;
                for row in 0..n {
                    for col in 0..n {
                        let v = grid[idx_of(col, row)];
                        if v > 9 {
                            flashes += 1;
                            grid[idx_of(col, row)] = -1; // off
                                                         // flash, so increase neighbors
                            let (row, col) = (row as isize, col as isize);
                            for arow in (row - 1)..=(row + 1) {
                                if arow < 0 || arow >= n_isize {
                                    continue;
                                }
                                for acol in (col - 1)..=(col + 1) {
                                    if acol < 0 || acol >= n_isize {
                                        continue;
                                    }
                                    if grid[idx_of(acol as usize, arow as usize)] >= 0 {
                                        grid[idx_of(acol as usize, arow as usize)] += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                if flashes_before == flashes {
                    break;
                }
            }
            for idx in 0..sz {
                if grid[idx] < 0 {
                    grid[idx] = 0;
                }
            }
        }

        format!("{}", flashes)
    } else {
        let w = input.lines().next().unwrap().chars().count();
        let h = input.lines().count();
        assert_eq!(w, h);
        let n = w;
        let n_isize = n as isize;
        let sz = n * n;

        let mut grid = Vec::new();
        grid.resize(sz, 0i8);
        let idx_of = |x, y| x + n * y;

        let mut idx = 0;
        for line in input.lines() {
            for c in line.chars() {
                grid[idx] = c.to_digit(10).unwrap() as i8;
                idx += 1;
            }
        }
        assert_eq!(idx, sz);

        let mut step = 0;
        loop {
            for idx in 0..sz {
                grid[idx] += 1;
            }

            let mut flashes = 0usize;
            loop {
                let flashes_before = flashes;
                for row in 0..n {
                    for col in 0..n {
                        let v = grid[idx_of(col, row)];
                        if v > 9 {
                            flashes += 1;
                            grid[idx_of(col, row)] = -1; // off
                                                         // flash, so increase neighbors
                            let (row, col) = (row as isize, col as isize);
                            for arow in (row - 1)..=(row + 1) {
                                if arow < 0 || arow >= n_isize {
                                    continue;
                                }
                                for acol in (col - 1)..=(col + 1) {
                                    if acol < 0 || acol >= n_isize {
                                        continue;
                                    }
                                    if grid[idx_of(acol as usize, arow as usize)] >= 0 {
                                        grid[idx_of(acol as usize, arow as usize)] += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                if flashes_before == flashes {
                    break;
                }
            }
            for idx in 0..sz {
                if grid[idx] < 0 {
                    grid[idx] = 0;
                }
            }
            step += 1;
            if flashes == sz {
                break;
            }
        }

        format!("{}", step)
    }
}
