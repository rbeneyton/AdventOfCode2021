pub fn solve(part: u8, input: &String) -> String {
    if part == 1 {
        pub type T = u16;
        let mut line = input.lines();
        let algo = line
            .next()
            .unwrap()
            .chars()
            .map(|c| if c == '#' { 1 } else { 0 })
            .collect::<Vec<T>>();
        // dbg!(algo.len());

        let w = input.lines().skip(2).next().unwrap().chars().count();
        let h = input.lines().skip(2).count();
        // dbg!(w);
        // dbg!(h);
        assert_eq!(w, h);
        let n = w;
        let sz = n * n;

        let mut grid = Vec::new();
        grid.resize(sz, 0 as T);
        let idx_of = |x, y| x + n * y;

        let mut idx = 0;
        for line in input.lines().skip(2) {
            for c in line.chars() {
                grid[idx] = if c == '#' { 1 as T } else { 0 };
                idx += 1;
            }
        }
        assert_eq!(idx, sz);

        // extend
        let mut egrid = Vec::new();
        let extra = 10;
        let en = n + 2 * extra;
        let esz = en * en;
        egrid.resize(esz, 0 as T);
        let eidx_of = |x, y| x + en * y;
        for col in 0..n {
            for row in 0..n {
                egrid[eidx_of(extra + col, extra + row)] = grid[idx_of(col, row)];
            }
        }
        drop(grid);
        let n = en;
        let sz = esz;
        let idx_of = |x, y| x + n * y;
        let grid = egrid;

        // algo
        assert_eq!(algo[0], 1); // 9*0 became 1
        assert_eq!(algo[511], 0); // 9*1 became 0, so infinite is 2 algo stable
        let mut agrid = Vec::new();
        agrid.resize(sz, algo[0] as T); // initial infinite 0
        for col in 1..(n - 1) {
            for row in 1..(n - 1) {
                let mut sum = 0;
                sum += grid[idx_of(col - 1, row - 1)] << 8;
                sum += grid[idx_of(col - 0, row - 1)] << 7;
                sum += grid[idx_of(col + 1, row - 1)] << 6;
                sum += grid[idx_of(col - 1, row + 0)] << 5;
                sum += grid[idx_of(col - 0, row + 0)] << 4;
                sum += grid[idx_of(col + 1, row + 0)] << 3;
                sum += grid[idx_of(col - 1, row + 1)] << 2;
                sum += grid[idx_of(col - 0, row + 1)] << 1;
                sum += grid[idx_of(col + 1, row + 1)] << 0;
                assert!(sum < 512);
                agrid[idx_of(col, row)] = algo[sum as usize];
            }
        }

        let mut res = 0;
        for col in 1..(n - 1) {
            for row in 1..(n - 1) {
                let mut sum = 0;
                sum += agrid[idx_of(col - 1, row - 1)] << 8;
                sum += agrid[idx_of(col - 0, row - 1)] << 7;
                sum += agrid[idx_of(col + 1, row - 1)] << 6;
                sum += agrid[idx_of(col - 1, row + 0)] << 5;
                sum += agrid[idx_of(col - 0, row + 0)] << 4;
                sum += agrid[idx_of(col + 1, row + 0)] << 3;
                sum += agrid[idx_of(col - 1, row + 1)] << 2;
                sum += agrid[idx_of(col - 0, row + 1)] << 1;
                sum += agrid[idx_of(col + 1, row + 1)] << 0;
                assert!(sum < 512);
                res += algo[sum as usize];
            }
        }

        // 5231
        format!("{}", res)
    } else {
        pub type T = u16;
        let mut line = input.lines();
        let algo = line
            .next()
            .unwrap()
            .chars()
            .map(|c| if c == '#' { 1 } else { 0 })
            .collect::<Vec<T>>();
        // dbg!(algo.len());

        let w = input.lines().skip(2).next().unwrap().chars().count();
        let h = input.lines().skip(2).count();
        // dbg!(w);
        // dbg!(h);
        assert_eq!(w, h);
        let n = w;
        let sz = n * n;

        let mut grid = Vec::new();
        grid.resize(sz, 0 as T);
        let idx_of = |x, y| x + n * y;

        let mut idx = 0;
        for line in input.lines().skip(2) {
            for c in line.chars() {
                grid[idx] = if c == '#' { 1 as T } else { 0 };
                idx += 1;
            }
        }
        assert_eq!(idx, sz);

        // extend
        let mut egrid = Vec::new();
        let extra = 60;
        let en = n + 2 * extra;
        let esz = en * en;
        egrid.resize(esz, 0 as T);
        let eidx_of = |x, y| x + en * y;
        for col in 0..n {
            for row in 0..n {
                egrid[eidx_of(extra + col, extra + row)] = grid[idx_of(col, row)];
            }
        }
        drop(grid);
        let n = en;
        let sz = esz;
        let idx_of = |x, y| x + n * y;
        let mut grid = egrid;
        let mut agrid = Vec::new();
        let mut res = 0;
        for step in 0..50 {
            agrid.resize(sz, algo[step % 2] as T); // initial infinite 0
            res = 0;
            for col in 1..(n - 1) {
                for row in 1..(n - 1) {
                    let mut sum = 0;
                    sum += grid[idx_of(col - 1, row - 1)] << 8;
                    sum += grid[idx_of(col - 0, row - 1)] << 7;
                    sum += grid[idx_of(col + 1, row - 1)] << 6;
                    sum += grid[idx_of(col - 1, row + 0)] << 5;
                    sum += grid[idx_of(col - 0, row + 0)] << 4;
                    sum += grid[idx_of(col + 1, row + 0)] << 3;
                    sum += grid[idx_of(col - 1, row + 1)] << 2;
                    sum += grid[idx_of(col - 0, row + 1)] << 1;
                    sum += grid[idx_of(col + 1, row + 1)] << 0;
                    assert!(sum < 512);
                    agrid[idx_of(col, row)] = algo[sum as usize];
                    res += algo[sum as usize]; // valid only on pair step
                }
            }
            // println!("step:{} res:{}", step, res);
            std::mem::swap(&mut agrid, &mut grid);
        }

        format!("{}", res)
    }
}
