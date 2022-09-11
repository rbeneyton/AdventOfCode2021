pub fn solve(part: u8, input: &String) -> String {
    if part == 1 {
        let positions: Vec<_> = input
            .lines()
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        // TODO sort + bisect
        let start: i32 = *positions.iter().min().unwrap();
        let stop: i32 = *positions.iter().max().unwrap();
        let steps: i32 = (start..=stop)
            .map(|x| positions.iter().map(|p| (p - x).abs()).sum())
            .min()
            .unwrap();

        format!("{}", steps)
    } else {
        let positions: Vec<_> = input
            .lines()
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        // TODO sort + bisect
        let start: i32 = *positions.iter().min().unwrap();
        let stop: i32 = *positions.iter().max().unwrap();
        let steps: i32 = (start..=stop)
            .map(|x| {
                positions
                    .iter()
                    .map(|p| {
                        let step = (p - x).abs();
                        step * (step + 1) / 2
                    })
                    .sum()
            })
            .min()
            .unwrap();

        format!("{}", steps)
    }
}
