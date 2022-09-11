pub fn solve(part: u8, input: &String) -> String {
    if part == 1 {
        const N: usize = 9;
        type Fishs = [usize; N]; // 0..=8

        let mut fishs = Fishs::default();
        for line in input.lines() {
            for tok in line.split(',').map(|x| x.parse::<usize>().unwrap()) {
                assert!(tok < N);
                fishs[tok] += 1;
            }
        }

        for _day in 0..80 {
            let mut new_fishs = Fishs::default();

            for age in 0..N {
                if age == 0 {
                    new_fishs[6] = fishs[age]; // return to age 6
                    new_fishs[8] = fishs[age]; // spawn babies
                } else {
                    new_fishs[age - 1] += fishs[age]; // decrease age counter
                }
            }
            fishs = new_fishs;
        }

        let sum: usize = fishs.iter().sum();

        format!("{}", sum)
    } else {
        const N: usize = 9;
        type Fishs = [usize; N]; // 0..=8

        let mut fishs = Fishs::default();
        for line in input.lines() {
            for tok in line.split(',').map(|x| x.parse::<usize>().unwrap()) {
                assert!(tok < N);
                fishs[tok] += 1;
            }
        }

        for _day in 0..256 {
            let mut new_fishs = Fishs::default();

            for age in 0..N {
                if age == 0 {
                    new_fishs[6] = fishs[age]; // return to age 6
                    new_fishs[8] = fishs[age]; // spawn babies
                } else {
                    new_fishs[age - 1] += fishs[age]; // decrease age counter
                }
            }
            fishs = new_fishs;
        }

        let sum: usize = fishs.iter().sum();

        format!("{}", sum)
    }
}
