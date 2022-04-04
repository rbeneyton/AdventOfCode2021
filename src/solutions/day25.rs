pub fn solve(_part: u8, input: &String) -> String {
    let w = input.lines().next().unwrap().chars().count();
    let h = input.lines().count();

    #[repr(u8)]
    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    enum SeaCucumber {
        None = 0,
        East,
        South,
    }

    let mut grid = Vec::new();
    grid.resize(w * h, SeaCucumber::None);
    let idx_of = |x, y| (x % w) + w * (y % h);

    let mut idx = 0;
    for line in input.lines() {
        for c in line.chars() {
            grid[idx] = match c {
                '.' => SeaCucumber::None,
                '>' => SeaCucumber::East,
                'v' => SeaCucumber::South,
                _ => panic!(""),
            };
            idx += 1;
        }
    }
    assert_eq!(idx, w * h);

    let mut ngrid = Vec::new();

    let mut step = 0;
    let [neast, nsouth] =
        [SeaCucumber::East, SeaCucumber::South].map(|t| grid.iter().filter(|x| **x == t).count());

    loop {
        ngrid.clear();
        ngrid.resize(w * h, SeaCucumber::None);

        assert_eq!(
            neast,
            grid.iter().filter(|x| **x == SeaCucumber::East).count()
        );
        assert_eq!(
            nsouth,
            grid.iter().filter(|x| **x == SeaCucumber::South).count()
        );

        let mut moves = 0;
        for row in 0..h {
            for col in 0..w {
                if grid[idx_of(col, row)] == SeaCucumber::East {
                    if grid[idx_of(col + 1, row)] == SeaCucumber::None {
                        ngrid[idx_of(col + 1, row)] = SeaCucumber::East;
                        moves += 1;
                    } else {
                        ngrid[idx_of(col, row)] = SeaCucumber::East;
                    }
                }
            }
        }
        for row in 0..h {
            for col in 0..w {
                if grid[idx_of(col, row)] == SeaCucumber::South {
                    if grid[idx_of(col, row + 1)] != SeaCucumber::South
                        && ngrid[idx_of(col, row + 1)] == SeaCucumber::None
                    {
                        ngrid[idx_of(col, row + 1)] = SeaCucumber::South;
                        moves += 1;
                    } else {
                        ngrid[idx_of(col, row)] = SeaCucumber::South;
                    }
                }
            }
        }
        step += 1;
        // println!("step:{} moves:{}", step, moves);

        if moves == 0 {
            break;
        }
        std::mem::swap(&mut ngrid, &mut grid);
    }

    format!("{}", step)
}
