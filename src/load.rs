use crate::Day;

use curl::easy::Easy;
use std::fs::{read_to_string, write};

pub fn get_data(day: Day, session: &String) -> String
{
    let file = format!("{}/data/{}.input", env!("CARGO_MANIFEST_DIR"), day);
    match read_to_string(&file) {
        Ok(data) => data,
        Err(..) => {
            let data = get_data_server(day, session);
            write(&file, &data).expect("cannot write onto file");
            data
        }
    }
}

fn get_data_server(day: Day, session: &String) -> String
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
