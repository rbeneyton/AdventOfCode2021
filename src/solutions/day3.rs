pub fn solve(part: u8, input: &String) -> String {
    if part == 1 {
        const N: usize = 12;
        type Acc = [usize; N];

        let (acc0, acc1) =
            input
                .lines()
                .fold((Acc::default(), Acc::default()), |(acc0, acc1), line| {
                    let mut acc0 = acc0;
                    let mut acc1 = acc1;
                    for (idx, c) in line.chars().enumerate() {
                        assert!(idx < N);
                        if c == '0' {
                            acc0[idx] += 1;
                        }
                        if c == '1' {
                            acc1[idx] += 1;
                        }
                    }
                    (acc0, acc1)
                });
        for i in 0..N {
            assert_eq!(acc0[i] + acc1[i], acc0[0] + acc1[0]);
        }
        let (most_common, second_common) = (0..N).fold((0, 0), |(most, second), i| {
            let abs_idx = N - 1 - i;
            let incr = 1 << abs_idx;
            if acc1[i] > acc0[i] {
                (most + incr, second)
            } else {
                (most, second + incr)
            }
        });
        assert_eq!(most_common + second_common, (1 << N) - 1);

        format!("{}", most_common * second_common)
    } else {
        const N: usize = 12;

        let get_rating = |input: &str, most: bool| {
            let n = input.lines().count();
            let mut skip = Vec::new();
            skip.resize(n, false);
            for i in 0..N {
                let mut valid_n = 0;
                let mut valid = "invalid";

                let (acc0, acc1) = input
                    .lines()
                    .enumerate()
                    .filter_map(|(idx, line)| if skip[idx] { None } else { Some(line) })
                    .fold((0, 0), |(mut acc0, mut acc1), line| {
                        let c = line.chars().skip(i).next().unwrap();
                        if c == '0' {
                            acc0 += 1;
                        }
                        if c == '1' {
                            acc1 += 1;
                        }
                        (acc0, acc1)
                    });
                let most_common = if acc1 >= acc0 { '1' } else { '0' };
                let least_common = if acc1 < acc0 { '1' } else { '0' };
                let common = if most { most_common } else { least_common };

                for (idx, line) in input.lines().enumerate() {
                    if skip[idx] {
                        continue;
                    }
                    let c = line.chars().skip(i).next().unwrap();
                    if c != common {
                        skip[idx] = true;
                        continue;
                    }
                    valid_n += 1;
                    valid = line;
                }
                if valid_n == 1 {
                    return valid.to_string();
                }
            }
            unreachable!();
        };
        let oxygen = get_rating(&input, true);
        // println!("oxygen: {}", oxygen);
        let co2 = get_rating(&input, false);
        // println!("co2: {}", co2);

        let to_number = |input: &str| {
            let mut res = 0;
            let mut c = input.chars();
            for i in 0..N {
                if c.next().unwrap() == '1' {
                    let abs_idx = N - 1 - i;
                    res += 1 << abs_idx;
                }
            }
            res
        };
        let oxygen = to_number(&oxygen);
        let co2 = to_number(&co2);

        format!("{}", oxygen * co2)
    }
}
