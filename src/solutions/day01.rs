use itertools::Itertools;

pub fn solve(part: u8, input: &String) -> String {
    if part == 1 {
        let mut res = 0;
        let mut prev = None;

        for line in input.lines() {
            if let Ok(value) = line.parse::<i64>() {
                if let Some(prev) = prev {
                    if value > prev {
                        res += 1;
                    }
                }
                prev = Some(value);
            }
        }
        format!("{}", res)
    } else {
        let (increased, _) = input
            .lines()
            .filter_map(|x| x.parse::<i64>().ok())
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .fold((0, None), |(acc, prev), sum| {
                let inc = if let Some(prev) = prev {
                    if sum > prev {
                        1
                    } else {
                        0
                    }
                } else {
                    0
                };
                (acc + inc, Some(sum))
            });

        format!("{}", increased)
    }
}
