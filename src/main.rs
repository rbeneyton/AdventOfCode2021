use curl::easy::Easy;
use clap::Parser;
use itertools::Itertools;

pub type Day = i8;

#[derive(Parser)]
#[clap(version = "1.0")]
pub struct Options {
    /// day
    #[clap(short, long, default_value = "1")]
    day: Day,
    /// part
    #[clap(short, long, default_value = "1")]
    part: u8,
    /// session cookie
    #[clap(short, long, default_value = "unset")]
    session: String,
}

fn get_data(day: Day, session: String) -> String
{
    let mut res = Vec::new();
    let mut easy = Easy::new();

    let url = format!("https://adventofcode.com/2021/day/{}/input", day);
    easy.url(&url).unwrap();
    let cookie = format!("session={}", session);
    easy.cookie(&cookie).unwrap();

    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        res.extend_from_slice(data);
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();
    drop(transfer);

    String::from_utf8_lossy(&res).to_string()
}

fn solve(day: Day, part: u8, input: String) -> String {
    match day {
        1 => if part == 1 {
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
            let (increased, _) = input.lines()
                .filter_map(|x| x.parse::<i64>().ok())
                .tuple_windows()
                .map(|(a, b, c)| a + b + c)
                .fold((0, None), |(acc, prev), sum| {
                    let inc = if let Some(prev) = prev {
                        if sum > prev { 1 } else { 0 }
                    } else { 0 };
                    (acc + inc, Some(sum)) });

            format!("{}", increased)
        },
        2 => if part == 1 {
            let (horizontal, depth) = input.lines()
                .fold((0, 0), |(hor, depth), line| {
                    let (mut inc_hor, mut inc_depth) = (0, 0);
                    let words : Vec<_> = line.split_whitespace().collect();
                    if words.len() == 2 {
                        let step = words[1].parse::<i64>().unwrap();
                        if words[0] == "forward" { inc_hor += step; }
                        if words[0] == "down" { inc_depth += step; }
                        if words[0] == "up" { inc_depth -= step; }
                    }
                    (hor + inc_hor, depth + inc_depth) });

            format!("{}", horizontal * depth)
        } else {
            let (horizontal, depth, _aim) = input.lines()
                .fold((0, 0, 0), |(hor, depth, aim), line| {
                    let (mut inc_hor, mut inc_depth, mut inc_aim) = (0, 0, 0);
                    let words : Vec<_> = line.split_whitespace().collect();
                    if words.len() == 2 {
                        let step = words[1].parse::<i64>().unwrap();
                        if words[0] == "forward" { inc_hor += step; inc_depth += aim * step;}
                        if words[0] == "down" { inc_aim += step; }
                        if words[0] == "up" { inc_aim -= step; }
                    }
                    (hor + inc_hor, depth + inc_depth, aim + inc_aim) });

            format!("{}", horizontal * depth)
        },
        3 => if part == 1 {
            const N : usize = 12;
            type Acc = [usize; N];

            let (acc0, acc1) = input.lines()
                .fold((Acc::default(), Acc::default()), |(acc0, acc1), line| {
                    let mut acc0 = acc0;
                    let mut acc1 = acc1;
                    for (idx, c) in line.chars().enumerate() {
                        assert!(idx < N);
                        if c == '0' { acc0[idx] += 1; }
                        if c == '1' { acc1[idx] += 1; }
                    }
                    (acc0, acc1) });
            for i in 0..N {
                assert_eq!(acc0[i] + acc1[i], acc0[0] + acc1[0]);
            }
            let (most_common, second_common) = (0..N)
                .fold((0, 0), |(most, second), i| {
                    let abs_idx = N - 1 - i;
                    let incr = 1 << abs_idx;
                    if acc1[i] > acc0[i] {
                        (most + incr, second)
                    } else {
                        (most, second + incr)
                    }});
            assert_eq!(most_common + second_common, (1 << N) - 1);

            format!("{}", most_common * second_common)
        } else {
            const N : usize = 12;

            let get_rating = |input : &str, most : bool| {
                let n = input.lines().count();
                let mut skip = Vec::new();
                skip.resize(n, false);
                for i in 0..N {
                    let mut valid_n = 0;
                    let mut valid = "invalid";

                    let (acc0, acc1) = input.lines().enumerate()
                        .filter_map(|(idx, line)| if skip[idx] { None } else { Some(line) })
                        .fold((0, 0), |(mut acc0, mut acc1), line| {
                            let c = line.chars().skip(i).next().unwrap();
                            if c == '0' { acc0 += 1; }
                            if c == '1' { acc1 += 1; }
                            (acc0, acc1) });
                    let most_common = if acc1 >= acc0 { '1' } else { '0' };
                    let least_common = if acc1 < acc0 { '1' } else { '0' };
                    let common = if most { most_common } else { least_common };

                    for (idx, line) in input.lines().enumerate() {
                        if skip[idx] { continue; }
                        let c = line.chars().skip(i).next().unwrap();
                        if c != common {
                            skip[idx] = true;
                            continue;
                        }
                        valid_n += 1;
                        valid = line;
                    }
                    if dbg!(valid_n) == 1 {
                        return valid.to_string();
                    }
                }
                unreachable!();
            };
            let oxygen = get_rating(&input, true);
            println!("oxygen: {}", oxygen);
            let co2 = get_rating(&input, false);
            println!("co2: {}", co2);

            let to_number = |input : &str| {
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
        },
        _ => String::from(""),
    }
}

fn main() {
    let options = Options::parse();

    let data = get_data(options.day, options.session);
    println!("day {} input: {}", options.day, data);

    let res = solve(options.day, options.part, data);
    println!("day {} part {} solve: {}", options.day, options.part, res)
}
