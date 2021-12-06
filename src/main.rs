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
            String::from("")
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
