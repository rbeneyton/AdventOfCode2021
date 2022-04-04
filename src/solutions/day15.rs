use std::cmp;
use std::collections::BinaryHeap;

pub fn solve(part: u8, input: &String) -> String {
    if part == 1 {
        let w = input.lines().next().unwrap().chars().count();
        let h = input.lines().count();

        assert_eq!(w, h);
        let n = h;

        let mut grid = Vec::new();
        let sz = n * n;
        grid.resize(sz, 0u32);
        let idx_of = |(x, y)| x + w * y;

        let mut idx = 0;
        for line in input.lines() {
            for c in line.chars() {
                let v = c.to_digit(10).unwrap() as u32;
                assert!(v > 0);
                grid[idx] = v;
                idx += 1;
            }
        }
        assert_eq!(idx, sz);

        let mut dist = Vec::new();
        dist.resize(sz, 0);
        let aim = (n - 1, n - 1);
        dist[idx_of(aim)] = grid[idx_of(aim)];
        'dist: loop {
            for row in (0..n).rev() {
                if let Some(col) = (0..n).rev().find(|x| dist[idx_of((*x, row))] == 0) {
                    dist[idx_of((col, row))] = grid[idx_of((col, row))]
                        + if col == n - 1 {
                            dist[idx_of((col, row + 1))]
                        } else if row == n - 1 {
                            dist[idx_of((col + 1, row))]
                        } else {
                            cmp::min(dist[idx_of((col, row + 1))], dist[idx_of((col + 1, row))])
                        };
                    if col == n - 1 {
                        break;
                    }
                } else if row == 0 {
                    break 'dist;
                }
            }
        }

        let res = dist[idx_of((0, 0))] - grid[idx_of((0, 0))];
        format!("{}", res)
    } else {
        let w = input.lines().next().unwrap().chars().count();
        let h = input.lines().count();
        assert_eq!(w, h);
        let n = h;

        type Dist = u32;

        let mut grid = Vec::new();
        let sz = n * n;
        grid.resize(sz, Dist::default());
        let mut idx = 0;
        for line in input.lines() {
            for c in line.chars() {
                let v = c.to_digit(10).unwrap() as Dist;
                assert!(v > 0);
                grid[idx] = v;
                idx += 1;
            }
        }
        assert_eq!(idx, sz);

        let idx_of = |x, y| x + n * y;
        let grid = grid;
        let tgrid = |pos: Pos| -> Dist {
            let (x, y) = (pos.col, pos.row);
            let idx = idx_of(x % n, y % n);
            let v = grid[idx];
            let pad_x = (x / n) as Dist;
            let pad_y = (y / n) as Dist;
            let v = v + pad_x + pad_y;
            if v > 9 {
                v - 9
            } else {
                v
            }
        };
        drop(idx_of);

        const EXPAND: usize = 5;
        let tn = EXPAND * n;
        let tsz = tn * tn;
        let mut dist2 = Vec::new();
        dist2.resize(tsz, Dist::default());
        let idx_of = |pos: Pos| pos.col + tn * pos.row;

        // 2 moves only distances
        dist2[tsz - 1] = tgrid(Pos::new(tn - 1, tn - 1));
        for idx in (0..(tsz - 1)).rev() {
            let (col, row) = (idx % tn, idx / tn);
            dist2[idx] = tgrid(Pos::new(col, row))
                + if col == tn - 1 {
                    dist2[idx + tn] // (col, row + 1)
                } else if row == tn - 1 {
                    dist2[idx + 1] // (col + 1, row)
                } else {
                    cmp::min(
                        dist2[idx + tn], // (col, row + 1)
                        dist2[idx + 1],
                    ) // (col + 1, row)
                };
        }
        let dist2 = dist2;
        // println!("2 move minimal distance: {}", dist2[0] - grid[0]);

        // {{{ Pos
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pos {
            col: usize,
            row: usize,
        }
        impl Pos {
            pub fn new(col: usize, row: usize) -> Self {
                Self { col, row }
            }
            pub fn neighbors(self, tn: usize) -> PosS {
                PosS { pos: self, tn }
            }
        }
        impl Default for Pos {
            fn default() -> Self {
                Self { col: 0, row: 0 }
            }
        }
        impl Ord for Pos {
            fn cmp(&self, other: &Pos) -> cmp::Ordering {
                self.col
                    .cmp(&other.col)
                    .then_with(|| self.row.cmp(&self.row))
            }
        }
        impl PartialOrd for Pos {
            fn partial_cmp(&self, other: &Pos) -> Option<cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        // }}}
        // {{{ Pos Iterator
        #[derive(Copy, Clone)]
        pub struct PosS {
            pos: Pos,
            tn: usize,
        }
        pub struct IntoIteratorPosS {
            pos: Pos,
            tn: usize,
            iter: u8,
        }
        impl IntoIterator for PosS {
            type Item = Pos;
            type IntoIter = IntoIteratorPosS;

            fn into_iter(self) -> Self::IntoIter {
                IntoIteratorPosS {
                    pos: self.pos,
                    tn: self.tn,
                    iter: 0u8,
                }
            }
        }
        impl Iterator for IntoIteratorPosS {
            type Item = Pos;

            fn next(&mut self) -> Option<Self::Item> {
                let (col, row) = (self.pos.col, self.pos.row);
                let tn = self.tn;
                self.iter += 1;
                match self.iter {
                    1 => {
                        if col > 0 {
                            Some(Pos::new(col - 1, row))
                        } else {
                            self.next()
                        }
                    }
                    2 => {
                        if row > 0 {
                            Some(Pos::new(col, row - 1))
                        } else {
                            self.next()
                        }
                    }
                    3 => {
                        if col < tn - 1 {
                            Some(Pos::new(col + 1, row))
                        } else {
                            self.next()
                        }
                    }
                    4 => {
                        if row < tn - 1 {
                            Some(Pos::new(col, row + 1))
                        } else {
                            self.next()
                        }
                    }
                    _ => None,
                }
            }
        }
        // }}}
        // {{{ Cand
        #[derive(Copy, Clone, Eq, PartialEq)]
        struct Cand {
            pos: Pos,
            distance: Dist,
            score: Dist,
        }
        impl Ord for Cand {
            fn cmp(&self, other: &Cand) -> cmp::Ordering {
                other.score.cmp(&self.score)
            }
        }
        impl PartialOrd for Cand {
            fn partial_cmp(&self, other: &Cand) -> Option<cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        // }}}

        // should underestimate to get optimal path
        let heuristic = |pos| dist2[idx_of(pos)] * 98 / 100;

        // TODO perf: use dict2 as there is no overlap od data (negative number)
        let mut dist = Vec::new();
        dist.resize(tsz, Dist::default());

        // TODO perf: fixed 'drop if exceed' BinaryHeap
        let mut heap = BinaryHeap::with_capacity(128);

        let pos = Pos::default();
        dist[0] = tgrid(pos);
        heap.push(Cand {
            pos,
            distance: dist[0],
            score: dist[0] + heuristic(pos),
        });

        'path: loop {
            let cand = heap.pop().unwrap();

            if dist[idx_of(cand.pos)] > cand.distance {
                continue;
            }

            for pos in cand.pos.neighbors(tn).into_iter() {
                let (col, row) = (pos.col, pos.row);
                let h = heuristic(pos);
                let distance = cand.distance + tgrid(pos);

                if dist[idx_of(pos)] == 0 || dist[idx_of(pos)] > distance {
                    dist[idx_of(pos)] = distance;
                    if col == tn - 1 && row == tn - 1 {
                        break 'path;
                    }
                    heap.push(Cand {
                        pos,
                        distance,
                        score: distance + h,
                    });
                }
            }
        }

        if false {
            println!("tiled_grid corner");
            let w = 20;
            for row in 0..w {
                for col in 0..w {
                    print!("{:1}", tgrid(Pos::new(col, row)));
                }
                println!("");
            }
            println!("dist corner start");
            for row in 0..w {
                for col in 0..w {
                    print!("{:5}", dist[col + tn * row]);
                }
                println!("");
            }
            println!("dist corner end");
            for row in (tn - w)..tn {
                for col in (tn - w)..tn {
                    print!("{:5}", dist[col + tn * row]);
                }
                println!("");
            }
        }

        let res = dist[idx_of(Pos::new(tn - 1, tn - 1))] - grid[0];
        format!("{}", res)
    }
}
