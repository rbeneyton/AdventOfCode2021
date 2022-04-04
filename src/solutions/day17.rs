use itertools::Itertools;
use std::cmp;

pub fn solve(part: u8, input: &String) -> String {
    if part == 1 {
        assert_eq!(input.lines().count(), 1);
        assert!(input.lines().next().unwrap().starts_with("target area: "));
        let input = input.lines().next().unwrap().chars().collect::<String>();
        let input = input.replace("target area: x=", "");
        let input = input.replace("..", ",");
        let input = input.replace(", y=", ",");
        let (x1, x2, y1, y2) = input
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        println!("{}/{} {}/{}", x1, x2, y1, y2);

        let launch = |x, y| {
            let (mut x, mut y) = (x, y);
            let mut pos = (0, 0);
            let mut high = 0;
            loop {
                pos.0 += x;
                pos.1 += y;
                if pos.1 > high {
                    high = pos.1;
                }

                if pos.0 >= x1 && pos.0 <= x2 && pos.1 >= y1 && pos.1 <= y2 {
                    break (true, high);
                }
                if pos.0 > x2 || pos.1 < cmp::min(0, y2) {
                    break (false, high);
                }

                if x > 0 {
                    x -= 1;
                }
                // if x < 0 { x += 1; }
                y -= 1;
                if x == 0 && pos.0 < x1 {
                    break (false, high);
                } // vertical fall
            }
        };

        let mut max_high = (0, 0, 0);
        for x in 1..x2 {
            for y in y1..10000 {
                let (hit, high) = launch(x, y);
                if hit && high > max_high.0 {
                    max_high = (high, x, y);
                }
            }
        }

        dbg!(max_high);
        format!("{}", max_high.0)
    } else {
        assert_eq!(input.lines().count(), 1);
        assert!(input.lines().next().unwrap().starts_with("target area: "));
        let input = input.lines().next().unwrap().chars().collect::<String>();
        let input = input.replace("target area: x=", "");
        let input = input.replace("..", ",");
        let input = input.replace(", y=", ",");
        let (x1, x2, y1, y2) = input
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        println!("{}/{} {}/{}", x1, x2, y1, y2);

        let launch = |mut vx, mut vy| {
            let (mut x, mut y) = (0, 0);
            loop {
                x += vx;
                y += vy;

                if vx > 0 {
                    vx -= 1;
                }
                if vx < 0 {
                    vx += 1;
                }
                vy -= 1;

                if x >= x1 && x <= x2 && y >= y1 && y <= y2 {
                    break 1;
                }
                if (x < x1 && vx == 0) || (y < y1 && vy < 0) {
                    break 0;
                }
            }
        };

        let mut n = 0;
        for x in 0..=x2 {
            for y in y1..10000 {
                n += launch(x, y);
            }
        }

        format!("{}", n)
    }
}
