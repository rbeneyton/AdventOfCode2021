use curl::easy::Easy;
use clap::Parser;

pub type Day = i8;

#[derive(Parser)]
#[clap(version = "1.0")]
pub struct Options {
    /// day
    #[clap(short, long, default_value = "1")]
    day: Day,
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

fn solve(day: Day, input: String) -> String {
    match day {
        1 => {
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
            return format!("{}", res);
        },
        _ => {},
    }

    String::from("")
}

fn main() {
    let options = Options::parse();

    let data = get_data(options.day, options.session);
    println!("day {} input: {}", options.day, data);

    let res = solve(options.day, data);
    println!("day {} solve: {}", options.day, res)
}
