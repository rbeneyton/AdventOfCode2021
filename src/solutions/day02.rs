pub fn solve(part: u8, input: &String) -> String {
    if part == 1 {
        let (horizontal, depth) = input.lines().fold((0, 0), |(hor, depth), line| {
            let (mut inc_hor, mut inc_depth) = (0, 0);
            let words: Vec<_> = line.split_whitespace().collect();
            if words.len() == 2 {
                let step = words[1].parse::<i64>().unwrap();
                if words[0] == "forward" {
                    inc_hor += step;
                }
                if words[0] == "down" {
                    inc_depth += step;
                }
                if words[0] == "up" {
                    inc_depth -= step;
                }
            }
            (hor + inc_hor, depth + inc_depth)
        });

        format!("{}", horizontal * depth)
    } else {
        let (horizontal, depth, _aim) = input.lines().fold((0, 0, 0), |(hor, depth, aim), line| {
            let (mut inc_hor, mut inc_depth, mut inc_aim) = (0, 0, 0);
            let words: Vec<_> = line.split_whitespace().collect();
            if words.len() == 2 {
                let step = words[1].parse::<i64>().unwrap();
                if words[0] == "forward" {
                    inc_hor += step;
                    inc_depth += aim * step;
                }
                if words[0] == "down" {
                    inc_aim += step;
                }
                if words[0] == "up" {
                    inc_aim -= step;
                }
            }
            (hor + inc_hor, depth + inc_depth, aim + inc_aim)
        });

        format!("{}", horizontal * depth)
    }
}
