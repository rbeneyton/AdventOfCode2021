use curl::easy::Easy;
use clap::Parser;
use itertools::Itertools;
use std::cmp;
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::fs::{read_to_string, write};
use std::time::Instant;
use std::iter::{Peekable, Sum};
use std::ops::Add;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;


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
    /// bench all until the given day
    #[clap(short, long)]
    bench: bool,
}

fn get_data(day: Day, session: &String) -> String
{
    let file = format!("data/{}.input", day);
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

fn solve(day: Day, part: u8, input: &String) -> String {
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
        4 => if part == 1 {
            let mut line = input.lines();
            let numbers : Vec<_> = line.next().unwrap()
                .split(',')
                .map(|x| x.parse::<u8>().unwrap())
                .collect();

            const N : usize = 5;
            type Board = [[(u8, bool); N]; N];
            let mut boards = Vec::new();
            loop {
                match line.next() {
                    None => break,
                    Some(v) => if v.chars().count() > 0 { panic!("invalid parse pattern"); },
                }
                let mut board = Board::default();
                for row in 0..N {
                    let mut row_it = line.next().unwrap().split_whitespace();
                    for col in 0..N {
                        board[row][col].0 = row_it.next().unwrap().parse::<u8>().unwrap();
                    }
                }
                boards.push(board);
            }

            for number in numbers {
                for board in &mut boards {
                    'place_num: for row in 0..N {
                        for col in 0..N {
                            if board[row][col].0 == number {
                                board[row][col].1 = true;
                                break 'place_num;
                            }
                        }
                    }

                    for scan in 0..N {
                        if (board[scan][0].1
                         && board[scan][1].1
                         && board[scan][2].1
                         && board[scan][3].1
                         && board[scan][4].1)
                         || (board[0][scan].1
                          && board[1][scan].1
                          && board[2][scan].1
                          && board[3][scan].1
                          && board[4][scan].1) {
                            let mut sum = 0i64;
                            for row in 0..N {
                                for col in 0..N {
                                    if board[row][col].1 == false {
                                        sum += board[row][col].0 as i64;
                                    }
                                }
                            }
                            return format!("{}", sum * (number as i64));
                        }
                    }
                }
            }

            String::from("")
        } else {
            let mut line = input.lines();
            let numbers : Vec<_> = line.next().unwrap()
                .split(',')
                .map(|x| x.parse::<u8>().unwrap())
                .collect();

            const N : usize = 5;
            type Board = [[(u8, bool); N]; N];
            let mut boards = Vec::new();
            loop {
                match line.next() {
                    None => break,
                    Some(v) => if v.chars().count() > 0 { panic!("invalid parse pattern"); },
                }
                let mut board = Board::default();
                for row in 0..N {
                    let mut row_it = line.next().unwrap().split_whitespace();
                    for col in 0..N {
                        board[row][col].0 = row_it.next().unwrap().parse::<u8>().unwrap();
                    }
                }
                boards.push((board, false, 0i64)); // board, win?, score
            }

            for number in numbers {
                let mut win_idx = None;
                for (idx, mut board) in boards.iter_mut().enumerate() {
                    if board.1 { continue; } // win
                    'place_num2: for row in 0..N {
                        for col in 0..N {
                            if board.0[row][col].0 == number {
                                board.0[row][col].1 = true;
                                break 'place_num2;
                            }
                        }
                    }

                    for scan in 0..N {
                        if (board.0[scan][0].1
                         && board.0[scan][1].1
                         && board.0[scan][2].1
                         && board.0[scan][3].1
                         && board.0[scan][4].1)
                         || (board.0[0][scan].1
                          && board.0[1][scan].1
                          && board.0[2][scan].1
                          && board.0[3][scan].1
                          && board.0[4][scan].1) {
                            let mut sum = 0i64;
                            for row in 0..N {
                                for col in 0..N {
                                    if board.0[row][col].1 == false {
                                        sum += board.0[row][col].0 as i64;
                                    }
                                }
                            }
                            board.1 = true;
                            board.2 = sum * (number as i64);
                            win_idx = Some(idx);
                        }
                    }
                }
                let remain = boards.iter().filter(|x| !x.1).count();
                if remain == 0 {
                    assert!(win_idx.is_some());
                    return format!("{}", boards[win_idx.unwrap()].2);
                }
            }

            String::from("")
        },
        5 => if part == 1 {
            let sz : usize = input.lines()
                .map(|x| {
                    let mut tok = x.split_whitespace();
                    let (x1, y1) = tok.next().unwrap().split(',').collect_tuple().unwrap();
                    tok.next(); // ->
                    let (x2, y2) = tok.next().unwrap().split(',').collect_tuple().unwrap();
                    [x1, y1, x2, y2].iter()
                        .map(|x| x.parse::<usize>().unwrap())
                        .max()
                })
                .max()
                .unwrap()
                .unwrap() + 1;

            // println!("sz: {}", sz);

            let mut grid = Vec::new();
            grid.resize(sz * sz, 0u8);
            let idx_of = |x, y| x + sz * y;

            for line in input.lines() {
                    let mut tok = line.split_whitespace();
                    let (x1, y1) = tok.next().unwrap().split(',').collect_tuple().unwrap();
                    tok.next(); // ->
                    let (x2, y2) = tok.next().unwrap().split(',').collect_tuple().unwrap();
                    let (x1, y1, x2, y2) = [x1, y1, x2, y2].iter()
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap();

                    if x1 == x2 {
                        let (start, stop) = (cmp::min(y1, y2), cmp::max(y1, y2));
                        for y in start..=stop {
                            grid[idx_of(x1, y)] += 1;
                        }
                    } else
                    if y1 == y2 {
                        let (start, stop) = (cmp::min(x1, x2), cmp::max(x1, x2));
                        for x in start..=stop {
                            grid[idx_of(x, y1)] += 1;
                        }
                    } else {
                        continue;
                    }
            }

            let sum_overlap = grid.iter().filter(|x| **x >= 2).count();

            format!("{}", sum_overlap)
        } else {
            let sz : usize = input.lines()
                .map(|x| {
                    let mut tok = x.split_whitespace();
                    let (x1, y1) = tok.next().unwrap().split(',').collect_tuple().unwrap();
                    tok.next(); // ->
                    let (x2, y2) = tok.next().unwrap().split(',').collect_tuple().unwrap();
                    [x1, y1, x2, y2].iter()
                        .map(|x| x.parse::<usize>().unwrap())
                        .max()
                })
                .max()
                .unwrap()
                .unwrap() + 1;

            // println!("sz: {}", sz);

            let mut grid = Vec::new();
            grid.resize(sz * sz, 0u16);
            let idx_of = |x, y| x + sz * y;

            for line in input.lines() {
                let mut tok = line.split_whitespace();
                let (x1, y1) = tok.next().unwrap().split(',').collect_tuple().unwrap();
                tok.next(); // ->
                let (x2, y2) = tok.next().unwrap().split(',').collect_tuple().unwrap();
                let (x1, y1, x2, y2) = [x1, y1, x2, y2].iter()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap();

                if x1 == x2 {
                    let (start, stop) = (cmp::min(y1, y2), cmp::max(y1, y2));
                    for y in start..=stop {
                        grid[idx_of(x1, y)] += 1;
                    }
                } else
                if y1 == y2 {
                    let (start, stop) = (cmp::min(x1, x2), cmp::max(x1, x2));
                    for x in start..=stop {
                        grid[idx_of(x, y1)] += 1;
                    }
                } else
                if (x2 > x1 && y2 > y1) || (x2 < x1 && y2 < y1) { // goes down right
                    let (x1, y1, x2, y2) = (cmp::min(x1, x2), cmp::min(y1, y2),
                                            cmp::max(x1, x2), cmp::max(y1, y2));
                    assert_eq!(x2 - x1, y2 - y1);
                    let depth = x2 - x1;
                    for step in 0..=depth {
                        grid[idx_of(x1 + step, y1 + step)] += 1;
                    }
                } else
                if (x2 > x1 && y2 < y1) || (x2 < x1 && y2 > y1) { // goes down left
                    let (x1, y1, x2, y2) = (cmp::max(x1, x2), cmp::min(y1, y2),
                                            cmp::min(x1, x2), cmp::max(y1, y2));
                    assert_eq!(x1 - x2, y2 - y1);
                    let depth = x1 - x2;
                    for step in 0..=depth {
                        grid[idx_of(x1 - step, y1 + step)] += 1;
                    }
                }
            }

            let sum_overlap = grid.iter().filter(|x| **x >= 2).count();

            format!("{}", sum_overlap)
        },
        6 => if part == 1 {
            const N : usize = 9;
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

            let sum : usize = fishs.iter().sum();

            format!("{}", sum)
        } else {
            const N : usize = 9;
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

            let sum : usize = fishs.iter().sum();

            format!("{}", sum)
        },
        7 => if part == 1 {
            let positions : Vec<_> = input.lines().next().unwrap()
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            // TODO sort + bisect
            let start : i32 = *positions.iter().min().unwrap();
            let stop : i32 = *positions.iter().max().unwrap();
            let steps : i32 = (start..=stop)
                .map(|x| positions.iter().map(|p| (p - x).abs()).sum())
                .min()
                .unwrap();

            format!("{}", steps)
        } else {
            let positions : Vec<_> = input.lines().next().unwrap()
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            // TODO sort + bisect
            let start : i32 = *positions.iter().min().unwrap();
            let stop : i32 = *positions.iter().max().unwrap();
            let steps : i32 = (start..=stop)
                .map(|x| positions.iter()
                    .map(|p| {
                        let step = (p - x).abs();
                        step * (step + 1) / 2 })
                    .sum())
                .min()
                .unwrap();

            format!("{}", steps)
        },
        8 => if part == 1 {
            let sum : usize = input.lines().map(|line| {
                let (digits, displayed) = line.split('|').collect_tuple().unwrap();
                [1, 4, 7, 8].iter().map(|digit| {
                    let n_wires = match digit {
                        1 => 2,
                        4 => 4,
                        7 => 3,
                        8 => 7,
                        _ => panic!(),
                    };
                    let wires = digits.split_whitespace()
                        .find(|x| x.len() == n_wires)
                        .unwrap()
                        .chars()
                        .sorted()
                        .collect::<String>();
                    let found : usize = displayed.split_whitespace()
                        .filter(|x| x.len() == n_wires)
                        .filter(|x| x.chars().sorted().collect::<String>() == wires)
                        .count();
                    found
                })
                .sum::<usize>()
            })
            .sum();

            format!("{}", sum)
        } else {
            let sum : usize = input.lines().map(|line| {
                let (numbers, displayed) = line.split('|').collect_tuple().unwrap();
                let mut wires_in_digit = HashMap::new();
                for number in numbers.split_whitespace() {
                    let n = number.len();
                    let v = number.chars().sorted().collect::<String>();
                    let entry = wires_in_digit.entry(n).or_insert(Vec::new());
                    entry.push(v);
                }
                assert!(wires_in_digit[&2].len() == 1); // 1
                let num1 = &wires_in_digit[&2][0];
                assert!(wires_in_digit[&3].len() == 1); // 7
                let num7 = &wires_in_digit[&3][0];
                assert!(wires_in_digit[&4].len() == 1); // 4
                let num4 = &wires_in_digit[&4][0];
                assert!(wires_in_digit[&5].len() == 3); // 2 3 5
                assert!(wires_in_digit[&6].len() == 3); // 0 6 9
                assert!(wires_in_digit[&7].len() == 1); // 8
                let num8 = &wires_in_digit[&7][0];

                // rule 1: 'a' wire is the only wire in 7 and not in 1
                let a : char = num7.chars().find(|c| !num1.chars().contains(c)).unwrap();
                // rule 2: for 6 wires number (0, 6, 9), only 6 haven't c&f (the ones in 1)
                assert_eq!(wires_in_digit[&6].iter()
                    .filter(|num| !num1.chars().all(|c| num.chars().contains(&c)))
                    .count(), 1);
                let num6 = &wires_in_digit[&6].iter()
                    .find(|num| !num1.chars().all(|c| num.chars().contains(&c)))
                    .unwrap();
                // rule 3: the missing wire in 6 is c
                assert_eq!(num8.chars().filter(|c| !num6.chars().contains(c)).count(), 1);
                let c : char = num8.chars().find(|c| !num6.chars().contains(c)).unwrap();
                assert!(a != c);
                // rule 4: only c&f in 1
                assert!(num1.chars().contains(&c));
                let f : char = num1.chars().find(|ch| ch != &c).unwrap();
                assert!(f != a);
                assert!(f != c);
                // rule 5: b&d are present in 4, but only b is present in all 6 wires number 0 6 9
                let b : char = num4.chars().filter(|c| !num1.chars().contains(c))
                    .find(|c| wires_in_digit[&6].iter().all(|num| num.chars().contains(c)))
                    .unwrap();
                assert!(b != a);
                assert!(b != c);
                assert!(b != f);
                // rule 6: d is present in 4
                let d : char = num4.chars().find(|ch| (ch != &b) && (ch != &c) && (ch != &f))
                    .unwrap();
                assert!(d != a);
                assert!(d != c);
                assert!(d != f);
                assert!(d != b);
                // rule 7: the only 6 wire number ≠ 6 with d is 9
                let num9 = &wires_in_digit[&6].iter()
                    .filter(|num| num != num6)
                    .find(|num| num.chars().contains(&d))
                    .unwrap();
                // rule 8: the missing wire in 9 is e
                let e : char = num8.chars().find(|c| !num9.chars().contains(c)).unwrap();
                assert!(e != a);
                assert!(e != c);
                assert!(e != f);
                assert!(e != b);
                assert!(e != d);
                // rule 9: g is in 8
                let g : char = num8.chars().find(|ch|
                    (ch != &a) && (ch != &b) && (ch != &c)
                    && (ch != &d) && (ch != &e) && (ch != &f))
                    .unwrap();
                assert!(g != a);
                assert!(g != c);
                assert!(g != f);
                assert!(g != b);
                assert!(g != d);
                assert!(g != e);

                // find numbers
                let num3 = &wires_in_digit[&5].iter()
                    .find(|num| num.chars().contains(&a) && num.chars().contains(&c)
                        && num.chars().contains(&d) && num.chars().contains(&f)
                        && num.chars().contains(&g))
                    .unwrap();
                let num2 = &wires_in_digit[&5].iter()
                    .find(|num| num.chars().contains(&a) && num.chars().contains(&c)
                        && num.chars().contains(&d) && num.chars().contains(&e)
                        && num.chars().contains(&g))
                    .unwrap();

                let mut aim = 0;
                for number in displayed.split_whitespace() {
                    aim *= 10;

                    let v = number.chars().sorted().collect::<String>();
                    match v.len() {
                        2 => aim += 1,
                        3 => aim += 7,
                        4 => aim += 4,
                        5 => {
                            if v == **num2 { aim += 2; }
                            else if v == **num3 { aim += 3; }
                            else { aim += 5; }
                        },
                        6 => {
                            if v == **num6 { aim += 6; }
                            else if v == **num9 { aim += 9; }
                            // 0
                        },
                        7 => aim += 8,
                        _ => panic!(),
                    }
                }

                aim
            })
            .sum();

            format!("{}", sum)
        },
        9 => if part == 1 {
            let w = input.lines().next().unwrap().chars().count();
            let h = input.lines().count();

            let mut grid = Vec::new();
            grid.resize(w * h, 0u8);
            let idx_of = |x, y| x + w * y;

            let mut idx = 0;
            for line in input.lines() {
                for c in line.chars() {
                    grid[idx] = c.to_digit(10).unwrap() as u8;
                    idx += 1;
                }
            }
            assert_eq!(idx, w * h);
            let grid = grid;

            let mut risks : u64 = 0;
            for row in 0..h {
                for col in 0..w {
                    let v = grid[idx_of(col, row)];
                    // first memory locality checks, so left & right
                    if col > 0 && grid[idx_of(col - 1, row)] <= v { continue; }
                    if col < w - 1 && grid[idx_of(col + 1, row)] <= v { continue; }
                    // then far away, so up & down
                    if row > 0 && grid[idx_of(col, row - 1)] <= v { continue; }
                    if row < h - 1 && grid[idx_of(col, row + 1)] <= v { continue; }
                    risks += (v + 1) as u64;
                }
            }

            format!("{}", risks)
        } else {
            let w = input.lines().next().unwrap().chars().count();
            let h = input.lines().count();

            let mut grid = Vec::new();
            let sz = w * h;
            grid.resize(sz, 0u8);
            let idx_of = |x, y| x + w * y;

            let mut idx = 0;
            for line in input.lines() {
                for c in line.chars() {
                    let d = c.to_digit(10).unwrap() as u8;
                    grid[idx] = if d == 9 { 255u8 } else { d };
                    idx += 1;
                }
            }
            assert_eq!(idx, sz);

            // find lowest points
            let mut lowests = HashSet::new();
            for row in 0..h {
                for col in 0..w {
                    let v = grid[idx_of(col, row)];
                    // first memory locality checks, so left & right
                    if col > 0 && grid[idx_of(col - 1, row)] <= v { continue; }
                    if col < w - 1 && grid[idx_of(col + 1, row)] <= v { continue; }
                    // then far away, so up & down
                    if row > 0 && grid[idx_of(col, row - 1)] <= v { continue; }
                    if row < h - 1 && grid[idx_of(col, row + 1)] <= v { continue; }
                    lowests.insert(idx_of(col, row));
                }
            }

            // clear
            for idx in 0..sz {
                if grid[idx] != 255 {
                    grid[idx] = 0;
                }
            }

            // flag basins
            assert!(lowests.len() < 256 - 2); // u8 minus 2 flags
            for (lowid, lowest) in lowests.iter().enumerate() {
                grid[*lowest] = (lowid + 1) as u8;
            }

            // extend
            loop {
                let mut modified = 0;
                for row in 0..h {
                    for col in 0..w {
                        let v = grid[idx_of(col, row)];
                        if v != 0 { continue; }
                        let mut bassin = 0;
                        if col > 0 {
                            let v = grid[idx_of(col - 1, row)];
                            if v != 255 && v != 0 { bassin = v; }
                        }
                        if bassin == 0 && col < w - 1 {
                            let v = grid[idx_of(col + 1, row)];
                            if v != 255 && v != 0 { bassin = v; }
                        }
                        if bassin == 0 && row > 0 {
                            let v = grid[idx_of(col, row - 1)];
                            if v != 255 && v != 0 { bassin = v; }
                        }
                        if bassin == 0 && row < h - 1 {
                            let v = grid[idx_of(col, row + 1)];
                            if v != 255 && v != 0 { bassin = v; }
                        }
                        if bassin != 0 {
                            grid[idx_of(col, row)] = bassin;
                            modified += 1;
                        }
                    }
                }
                if modified == 0 {
                    break;
                }
            }

            let display = false;
            if display {
                for row in 0..h {
                    for col in 0..w {
                        let v = grid[idx_of(col, row)];
                        if v == 255 {
                            // print!(" XXX");
                        } else {
                            // print!(" {:3}", v);
                        }
                    }
                    // println!("");
                }
            }

            // count
            let mut acc = Vec::new();
            acc.resize(lowests.len() + 1, 0usize);
            for idx in 0..sz {
                let v = grid[idx];
                assert!(v != 0);
                if v != 255 {
                    acc[v as usize] += 1;
                }
            }
            assert_eq!(acc[0], 0);
            acc.sort();

            let n = acc.len();
            assert!(n > 3 + 1);
            let product_largest = acc[n - 1] * acc[n - 2] * acc[n - 3];

            format!("{}", product_largest)
        },
        10 => if part == 1 {
            let mut stack = Vec::new();
            let mut score = 0;
            'line: for line in input.lines() {
                stack.clear();
                'scan: for c in line.chars() {
                    for (open, close, point) in [('(', ')', 3), ('[', ']', 57), ('{', '}', 1197), ('<', '>', 25137)] {
                        if c == open {
                            stack.push(c);
                            continue 'scan;
                        }
                        if c == close {
                            let last = stack.pop().expect("empty stack");
                            if last == open {
                                continue 'scan;
                            } else {
                                score += point;
                                continue 'line;
                            }
                        }
                    }
                    panic!("invalid token {}", c);
                }
            }

            format!("{}", score)
        } else {
            let mut stack = Vec::new();
            let mut scores = Vec::new();
            'line2: for line in input.lines() {
                stack.clear();
                'scan2: for c in line.chars() {
                    for (open, close) in [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')] {
                        if c == open {
                            stack.push(c);
                            continue 'scan2;
                        }
                        if c == close {
                            let last = stack.pop().expect("empty stack");
                            if last == open {
                                continue 'scan2;
                            } else {
                                continue 'line2;
                            }
                        }
                    }
                    panic!("invalid token {}", c);
                }
                let mut line_score = 0u64;
                for c in stack.iter().rev() {
                    line_score *= 5;
                    line_score += match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!(""),
                    };
                }
                scores.push(line_score);
            }
            let n = scores.len();
            assert_eq!(n % 2, 1);
            scores.sort();

            format!("{}", scores[n / 2])
        }
        11 => if part == 1 {
            let w = input.lines().next().unwrap().chars().count();
            let h = input.lines().count();
            assert_eq!(w, h);
            let n = w;
            let n_isize = n as isize;
            let sz = n * n;

            let mut grid = Vec::new();
            grid.resize(sz, 0i8);
            let idx_of = |x, y| x + n * y;

            let mut idx = 0;
            for line in input.lines() {
                for c in line.chars() {
                    grid[idx] = c.to_digit(10).unwrap() as i8;
                    idx += 1;
                }
            }
            assert_eq!(idx, sz);

            let mut flashes = 0u64;
            for _step in 0..100 {
                for idx in 0..sz {
                    grid[idx] += 1;
                }

                loop {
                    let flashes_before = flashes;
                    for row in 0..n {
                        for col in 0..n {
                            let v = grid[idx_of(col, row)];
                            if v > 9 {
                                flashes += 1;
                                grid[idx_of(col, row)] = -1; // off
                                // flash, so increase neighbors
                                let (row, col) = (row as isize, col as isize);
                                for arow in (row - 1)..=(row + 1) {
                                    if arow < 0 || arow >= n_isize { continue; }
                                    for acol in (col - 1)..=(col + 1) {
                                        if acol < 0 || acol >= n_isize { continue; }
                                        if grid[idx_of(acol as usize, arow as usize)] >= 0 {
                                            grid[idx_of(acol as usize, arow as usize)] += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if flashes_before == flashes {
                        break;
                    }
                }
                for idx in 0..sz {
                    if grid[idx] < 0 {
                        grid[idx] = 0;
                    }
                }
            }

            format!("{}", flashes)
        } else {
            let w = input.lines().next().unwrap().chars().count();
            let h = input.lines().count();
            assert_eq!(w, h);
            let n = w;
            let n_isize = n as isize;
            let sz = n * n;

            let mut grid = Vec::new();
            grid.resize(sz, 0i8);
            let idx_of = |x, y| x + n * y;

            let mut idx = 0;
            for line in input.lines() {
                for c in line.chars() {
                    grid[idx] = c.to_digit(10).unwrap() as i8;
                    idx += 1;
                }
            }
            assert_eq!(idx, sz);

            let mut step = 0;
            loop {
                for idx in 0..sz {
                    grid[idx] += 1;
                }

                let mut flashes = 0usize;
                loop {
                    let flashes_before = flashes;
                    for row in 0..n {
                        for col in 0..n {
                            let v = grid[idx_of(col, row)];
                            if v > 9 {
                                flashes += 1;
                                grid[idx_of(col, row)] = -1; // off
                                // flash, so increase neighbors
                                let (row, col) = (row as isize, col as isize);
                                for arow in (row - 1)..=(row + 1) {
                                    if arow < 0 || arow >= n_isize { continue; }
                                    for acol in (col - 1)..=(col + 1) {
                                        if acol < 0 || acol >= n_isize { continue; }
                                        if grid[idx_of(acol as usize, arow as usize)] >= 0 {
                                            grid[idx_of(acol as usize, arow as usize)] += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if flashes_before == flashes {
                        break;
                    }
                }
                for idx in 0..sz {
                    if grid[idx] < 0 {
                        grid[idx] = 0;
                    }
                }
                step += 1;
                if flashes == sz {
                    break;
                }
            }

            format!("{}", step)
        }
        12 => if part == 1 {
            let mut nodes = HashMap::new();
            for line in input.lines() {
                let (from, to) = line.split('-').collect_tuple().unwrap();
                let entry = nodes.entry(from).or_insert(HashSet::new());
                entry.insert(to);
                let entry = nodes.entry(to).or_insert(HashSet::new());
                entry.insert(from);
            }

            fn upper<'a>(node : &'a str) -> bool { node.chars().all(char::is_uppercase) }
            fn visit<'a>(nodes: &HashMap<&'a str, HashSet<&'a str>>,
                         from: &'a str,
                         path: &Vec<&'a str>,
                         paths: &mut Vec<Vec<&'a str>>)
            {
                for next in nodes.get(from).unwrap() {
                    if !upper(next) && path.contains(next) { continue; }
                    let mut new = path.clone();
                    new.push(next);
                    if *next == "end" {
                        paths.push(new);
                    } else {
                        visit(nodes, next, &new, paths);
                    }
                }
            }

            let mut paths = Vec::new();
            visit(&nodes, "start", &vec!["start",], &mut paths);

            format!("{}", paths.len())
        } else {
            let mut nodes = HashMap::new();
            for line in input.lines() {
                let (from, to) = line.split('-').collect_tuple().unwrap();
                let entry = nodes.entry(from).or_insert(HashSet::new());
                entry.insert(to);
                let entry = nodes.entry(to).or_insert(HashSet::new());
                entry.insert(from);
            }
            let nodes = nodes;

            fn upper<'a>(node : &'a str) -> bool { node.chars().all(char::is_uppercase) }

            let n = nodes.len();
            // technically only lower nodes indices + one extra flag are required, so 63 lower
            // nodes can be used using u64 (or use u128 with 127 lower nodes).
            // we code naive method here: all indices (lower + upper) + extra indices, so only 32
            assert!(n < 32);
            let idx_of = |needle : &str| {
                nodes.iter()
                    .enumerate()
                    .find_map(|(i, (f, _))| if *f == needle { Some(i) } else { None })
                    .unwrap()
            };

            type Path = usize;
            let mut nnodes = Vec::with_capacity(n);
            nnodes.resize(n, Path::default());
            let mut upper_mask = Path::default();
            let mut lower_mask = Path::default();
            for (idx, (from, tos)) in nodes.iter().enumerate() {
                if upper(from) {
                    upper_mask |= 1 << idx;
                    upper_mask |= 1 << (idx + 32);
                } else {
                    lower_mask |= 1 << idx;
                    lower_mask |= 1 << (idx + 32);
                }
                for to in tos {
                    nnodes[idx] |= 1 << idx_of(to);
                }
            }

            fn count_dup_small(path: Path, lower_mask: usize) -> u32 {
                ((path & lower_mask) & 0xFFFFFFFF00000000).count_ones()
            }
            fn visit(nnodes: &Vec<Path>,
                     from_idx: usize,
                     start_idx: usize,
                     end_idx: usize,
                     upper_mask: usize,
                     lower_mask: usize,
                     path: Path,
                     paths: &mut usize)
            {
                for next_idx in 0..32 {
                    let next_flag = 1 << next_idx;
                    if nnodes[from_idx] & next_flag != 0 {
                        if next_idx == start_idx { continue; }
                        let mut new_path = path;
                        if next_flag & upper_mask == 0 {
                            if new_path & next_flag != 0 {
                                if new_path & next_flag << 32 != 0 { continue; }
                                new_path |= next_flag << 32;
                            } else {
                                new_path |= next_flag;
                            }
                            if count_dup_small(new_path, lower_mask) > 1 { continue; }
                        } else {
                            new_path |= next_flag;
                        }
                        if next_idx == end_idx {
                            *paths += 1;
                        } else {
                            visit(nnodes, next_idx, start_idx, end_idx, upper_mask, lower_mask, new_path, paths);
                        }
                    }
                }
            }

            let start_idx = idx_of("start");
            let end_idx = idx_of("end");
            let mut paths = 0;
            visit(&nnodes, start_idx, start_idx, end_idx, upper_mask, lower_mask, 1 << start_idx, &mut paths);

            format!("{}", paths)
        }
        13 => if part == 1 {
            let points = input.lines()
                .filter_map(|x| if x.contains(',') {
                    Some(x.split(',')
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect_tuple::<(_, _)>()
                        .unwrap())
                } else { None })
                .collect::<Vec<_>>();
            let (w, h) = points.iter()
                .fold((0, 0), |(w, h), (col, row)| (cmp::max(w, *col), cmp::max(h, *row)));
            let (w, h) = (w + 1, h + 1);

            let mut grid = Vec::new();
            grid.resize(w * h, 0u8);
            let idx_of = |x, y| x + w * y;
            let pos_of = |idx| (idx % w, idx / w);
            for (col, row) in points {
                grid[idx_of(col, row)] = 1;
            }

            let (what, off) = input.lines()
                .find_map(|x| if x.starts_with("fold along ") {
                    let what = x.chars().skip(11)
                        .next().unwrap();
                    let off = x.split('=').skip(1).map(|x| x.parse::<usize>().unwrap())
                        .next().unwrap();
                    Some((what, off))
                } else { None })
                .unwrap();

            let (mut visible_w, mut visible_h) = (w, h);
            match what {
                'x' => {
                    for row in 0..h {
                        for col in (off + 1)..w {
                            let proj = off - (col - off);
                            grid[idx_of(proj, row)] += grid[idx_of(col, row)];
                            if proj == 0 { break; }
                        }
                    }
                    visible_w = off;
                },
                'y' => {
                    for row in (off + 1)..h {
                        let proj = off - (row - off);
                        for col in 0..w {
                            grid[idx_of(col, proj)] += grid[idx_of(col, row)];
                        }
                        if proj == 0 { break; }
                    }
                    visible_h = off;
                },
                _ => panic!("invalid axis"),
            }

            let visible_dots = grid.iter()
                .enumerate()
                .map(|(idx, v)| {
                    let (col, row) = pos_of(idx);
                    (col, row, v) })
                .filter(|(col, row, v)| *col < visible_w && *row < visible_h && *v > &0)
                .count();

            format!("{}", visible_dots)
        } else {
            let points = input.lines()
                .filter_map(|x| if x.contains(',') {
                    Some(x.split(',')
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect_tuple::<(_, _)>()
                        .unwrap())
                } else { None })
                .collect::<Vec<_>>();
            let (w, h) = points.iter()
                .fold((0, 0), |(w, h), (col, row)| (cmp::max(w, *col), cmp::max(h, *row)));
            let (w, h) = (w + 1, h + 1);

            let mut grid = Vec::new();
            grid.resize(w * h, 0u8);
            let idx_of = |x, y| x + w * y;
            for (col, row) in points {
                grid[idx_of(col, row)] = 1;
            }

            let (mut visible_w, mut visible_h) = (w, h);
            for (what, off) in input.lines()
                .filter_map(|x| if x.starts_with("fold along ") {
                    let what = x.chars().skip(11)
                        .next().unwrap();
                    let off = x.split('=').skip(1).map(|x| x.parse::<usize>().unwrap())
                        .next().unwrap();
                    Some((what, off))
                } else { None })
            {
                match what {
                    'x' => {
                        for row in 0..visible_h {
                            for col in (off + 1)..visible_w {
                                let proj = off - (col - off);
                                grid[idx_of(proj, row)] += grid[idx_of(col, row)];
                                if proj == 0 { break; }
                            }
                        }
                        visible_w = off;
                    },
                    'y' => {
                        for row in (off + 1)..visible_h {
                            let proj = off - (row - off);
                            for col in 0..visible_w {
                                grid[idx_of(col, proj)] += grid[idx_of(col, row)];
                            }
                            if proj == 0 { break; }
                        }
                        visible_h = off;
                    },
                    _ => panic!("invalid axis"),
                }
            }

            let mut res = String::from("\n");
            for row in 0..visible_h {
                for col in 0..visible_w {
                    if grid[idx_of(col, row)] > 0 {
                        res.push('X');
                    } else {
                        res.push(' ');
                    }
                }
                res.push('\n');
            }

            res
        }
        14 => if part == 1 {
            let polymer = input.lines().next().unwrap();
            let templates = input.lines()
                .filter(|x| x.contains(" -> "))
                .map(|x| x.split(" -> ")
                    .collect_tuple::<(_, _)>()
                    .map(|(pair, insert)|
                        ((pair.chars().next().unwrap(),
                          pair.chars().skip(1).next().unwrap()),
                         insert.chars().next().unwrap()))
                    .unwrap())
                .collect::<HashMap<_, _>>();

            let mut polymer = String::from(polymer);
            for _step in 0..10 {
                let last = polymer.chars().last();
                polymer = polymer.chars()
                    .tuple_windows()
                    .map(|(a, b)| [a, *templates.get(&(a, b)).unwrap()])
                    .flatten()
                    .chain(last)
                    .collect();
            }

            let keys_counts = polymer.chars()
                .fold(HashMap::<char, usize>::new(), |mut h, c| {
                    *h.entry(c).or_default() += 1;
                    h
                })
                .into_values()
                .collect::<Vec<_>>();

            let res = keys_counts.iter().max().unwrap() - keys_counts.iter().min().unwrap();
            format!("{}", res)
        } else {
            let polymer = input.lines().next().unwrap();
            let templates = input.lines()
                .filter(|x| x.contains(" -> "))
                .map(|x| x.split(" -> ")
                    .collect_tuple::<(_, _)>()
                    .map(|(pair, insert)|
                        ((pair.chars().next().unwrap(),
                          pair.chars().skip(1).next().unwrap()),
                         insert.chars().next().unwrap()))
                    .unwrap())
                .collect::<HashMap<_, _>>();

            let mut polymer = polymer.chars()
                    .tuple_windows()
                    .fold(HashMap::<_, usize>::new(), |mut h, (a, b)| {
                        *h.entry((a, b)).or_default() += 1; // XXX double count!
                        h
                    });
            for _step in 0..40 {
                polymer = polymer.iter()
                    .map(|((a, b), n)| {
                        let spawn = *templates.get(&(*a, *b)).unwrap();
                        [((*a, spawn), *n), ((spawn, *b), *n)] })
                    .flatten()
                    .fold(HashMap::<_, usize>::new(), |mut h, ((a, b), n)| {
                        *h.entry((a, b)).or_default() += n;
                        h
                    });
            }

            let keys_counts = polymer.iter()
                .map(|((a, b), n)| [(*a, n), (*b, n)])
                .flatten()
                .fold(HashMap::<char, usize>::new(), |mut h, (c, n)| {
                    *h.entry(c).or_default() += n;
                    h
                })
                .into_values()
                .collect::<Vec<_>>();

            let res = keys_counts.iter().max().unwrap() - keys_counts.iter().min().unwrap();
            format!("{}", res / 2)
        },
        15 => if part == 1 {
            let w = input.lines().next().unwrap().chars().count();
            let h = input.lines().count();

            assert_eq!(w, h);
            let n = h;

            let mut grid = Vec::new();
            let sz = n * n;
            grid.resize(sz, 0u32);
            let idx_of = |(x, y)| x + w * y;

            let mut idx = 0;
            for line in input.lines() {
                for c in line.chars() {
                    let v = c.to_digit(10).unwrap() as u32;
                    assert!(v > 0);
                    grid[idx] = v;
                    idx += 1;
                }
            }
            assert_eq!(idx, sz);

            let mut dist = Vec::new();
            dist.resize(sz, 0);
            let aim = (n - 1, n - 1);
            dist[idx_of(aim)] = grid[idx_of(aim)];
            'dist: loop {
                for row in (0..n).rev() {
                    if let Some(col) = (0..n).rev().find(|x| dist[idx_of((*x, row))] == 0) {
                        dist[idx_of((col, row))] = grid[idx_of((col, row))] +
                            if col == n - 1 {
                                dist[idx_of((col, row + 1))]
                            } else
                            if row == n - 1 {
                                dist[idx_of((col + 1, row))]
                            } else {
                                cmp::min(dist[idx_of((col, row + 1))],
                                         dist[idx_of((col + 1, row))])
                            };
                        if col == n - 1 { break; }
                    } else
                    if row == 0 { break 'dist; }
                }
            }

            let res = dist[idx_of((0, 0))] - grid[idx_of((0, 0))];
            format!("{}", res)
        } else {
            let w = input.lines().next().unwrap().chars().count();
            let h = input.lines().count();
            assert_eq!(w, h);
            let n = h;

            type Dist = u32;

            let mut grid = Vec::new();
            let sz = n * n;
            grid.resize(sz, Dist::default());
            let mut idx = 0;
            for line in input.lines() {
                for c in line.chars() {
                    let v = c.to_digit(10).unwrap() as Dist;
                    assert!(v > 0);
                    grid[idx] = v;
                    idx += 1;
                }
            }
            assert_eq!(idx, sz);

            let idx_of = |x, y| x + n * y;
            let grid = grid;
            let tgrid = |pos : Pos| -> Dist {
                let (x, y) = (pos.col, pos.row);
                let idx = idx_of(x % n, y % n);
                let v = grid[idx];
                let pad_x = (x / n) as Dist;
                let pad_y = (y / n) as Dist;
                let v = v + pad_x + pad_y;
                if v > 9 {
                    v - 9
                } else {
                    v
                }
            };
            drop(idx_of);

            const EXPAND : usize = 5;
            let tn = EXPAND * n;
            let tsz = tn * tn;
            let mut dist2 = Vec::new();
            dist2.resize(tsz, Dist::default());
            let idx_of = |pos : Pos| pos.col + tn * pos.row;

            // 2 moves only distances
            dist2[tsz - 1] = tgrid(Pos::new(tn - 1, tn - 1));
            for idx in (0..(tsz - 1)).rev() {
                let (col, row) = (idx % tn, idx / tn);
                dist2[idx] = tgrid(Pos::new(col, row)) +
                    if col == tn - 1 { dist2[idx + tn] // (col, row + 1)
                    } else
                    if row == tn - 1 { dist2[idx + 1] // (col + 1, row)
                    } else {
                        cmp::min(dist2[idx + tn], // (col, row + 1)
                                 dist2[idx + 1])  // (col + 1, row)
                    };
            }
            let dist2 = dist2;
            // println!("2 move minimal distance: {}", dist2[0] - grid[0]);

            // {{{ Pos
            #[derive(Copy, Clone, Eq, PartialEq)]
            pub struct Pos { col: usize, row: usize }
            impl Pos {
                pub fn new(col: usize, row: usize) -> Self {
                    Self { col, row }
                }
                pub fn neighbors(self, tn: usize) -> PosS {
                    PosS { pos: self, tn }
                }
            }
            impl Default for Pos {
                fn default() -> Self {
                    Self { col: 0, row: 0 }
                }
            }
            impl Ord for Pos {
                fn cmp(&self, other: &Pos) -> cmp::Ordering {
                    self.col.cmp(&other.col).then_with(|| self.row.cmp(&self.row))
                }
            }
            impl PartialOrd for Pos {
                fn partial_cmp(&self, other: &Pos) -> Option<cmp::Ordering> {
                    Some(self.cmp(other))
                }
            }
            // }}}
            // {{{ Pos Iterator
            #[derive(Copy, Clone)]
            pub struct PosS { pos: Pos, tn: usize }
            pub struct IntoIteratorPosS {
                pos: Pos,
                tn: usize,
                iter: u8,
            }
            impl IntoIterator for PosS {
                type Item = Pos;
                type IntoIter = IntoIteratorPosS;

                fn into_iter(self) -> Self::IntoIter {
                    IntoIteratorPosS {
                        pos: self.pos,
                        tn: self.tn,
                        iter: 0u8,
                    }
                }
            }
            impl Iterator for IntoIteratorPosS {
                type Item = Pos;

                fn next(&mut self) -> Option<Self::Item> {
                    let (col, row) = (self.pos.col, self.pos.row);
                    let tn = self.tn;
                    self.iter += 1;
                    match self.iter {
                        1 => if col > 0 { Some(Pos::new(col - 1, row )) } else { self.next() },
                        2 => if row > 0 { Some(Pos::new(col, row - 1 )) } else { self.next() },
                        3 => if col < tn - 1 { Some(Pos::new(col + 1, row)) } else { self.next() },
                        4 => if row < tn - 1 { Some(Pos::new(col, row + 1)) } else { self.next() },
                        _ => None,
                    }
                }
            }
            // }}}
            // {{{ Cand
            #[derive(Copy, Clone, Eq, PartialEq)]
            struct Cand { pos: Pos, distance: Dist, score: Dist }
            impl Ord for Cand {
                fn cmp(&self, other: &Cand) -> cmp::Ordering {
                    other.score.cmp(&self.score)
                }
            }
            impl PartialOrd for Cand {
                fn partial_cmp(&self, other: &Cand) -> Option<cmp::Ordering> {
                    Some(self.cmp(other))
                }
            }
            // }}}

            // should underestimate to get optimal path
            let heuristic = |pos| dist2[idx_of(pos)] * 98 / 100;

            // TODO perf: use dict2 as there is no overlap od data (negative number)
            let mut dist = Vec::new();
            dist.resize(tsz, Dist::default());

            // TODO perf: fixed 'drop if exceed' BinaryHeap
            let mut heap = BinaryHeap::with_capacity(128);

            let pos = Pos::default();
            dist[0] = tgrid(pos);
            heap.push(Cand { pos,
                             distance: dist[0],
                             score: dist[0] + heuristic(pos) });

            'path: loop {
                let cand = heap.pop().unwrap();

                if dist[idx_of(cand.pos)] > cand.distance { continue; }

                for pos in cand.pos.neighbors(tn).into_iter() {
                    let (col, row) = (pos.col, pos.row);
                    let h = heuristic(pos);
                    let distance = cand.distance + tgrid(pos);

                    if dist[idx_of(pos)] == 0
                    || dist[idx_of(pos)] > distance
                    {
                        dist[idx_of(pos)] = distance;
                        if col == tn - 1 && row == tn - 1 { break 'path; }
                        heap.push(Cand { pos,
                                         distance,
                                         score: distance + h });
                    }
                }
            }

            if false {
                println!("tiled_grid corner");
                let w = 20;
                for row in 0..w {
                    for col in 0..w {
                        print!("{:1}", tgrid(Pos::new(col, row)));
                    }
                    println!("");
                }
                println!("dist corner start");
                for row in 0..w {
                    for col in 0..w {
                        print!("{:5}", dist[col + tn * row]);
                    }
                    println!("");
                }
                println!("dist corner end");
                for row in (tn - w)..tn {
                    for col in (tn - w)..tn {
                        print!("{:5}", dist[col + tn * row]);
                    }
                    println!("");
                }
            }

            let res = dist[idx_of(Pos::new(tn - 1, tn - 1))] - grid[0];
            format!("{}", res)
        },
        16 => if part == 1 {
            assert_eq!(input.lines().count(), 1);
            let input : Vec<i64> = input.lines()
                .next().unwrap()
                .chars()
                .map(|c| {
                    let v = i64::from_str_radix(&c.to_string(), 16).unwrap();
                    [v & (1 << 3),
                     v & (1 << 2),
                     v & (1 << 1),
                     v & (1 << 0)] })
                .flatten()
                .map(|x| if x > 0 { 1 } else { 0 })
                .collect();
            let mut cursor = input.iter().peekable();

            fn process_packet<'buf, 'it, T>(cursor: &'it mut Peekable<T>) -> i64
                where T: Iterator<Item = &'buf i64>,
                      T: Itertools

            {
                let mut res = 0i64;
                // version: 3 bit
                let (v1, v2, v3) = cursor.next_tuple().unwrap();
                let version = (v1 << 2) + (v2 << 1) + v3;
                res += version;
                // type_id: 3 bit
                let (t1, t2, t3) = cursor.next_tuple().unwrap();
                match (t1, t2, t3) {
                    // litteral
                    (1, 0, 0) => {
                        let mut value = 0i64;
                        loop {
                            // group
                            let group_mark = cursor.next().unwrap();
                            for i in (0..4).rev() {
                                value += cursor.next().unwrap() << i;
                            }
                            if *group_mark == 0 { break; }
                            value <<= 4;
                        }
                        let _litteral = value;
                    },
                    // operator
                    _ => {
                        let lenght_type_id = cursor.next().unwrap();
                        let lenght_type_id_width = if *lenght_type_id == 1 { 11 } else { 15 };
                        let mut value = 0i64;
                        for i in (0..lenght_type_id_width).rev() {
                            value += cursor.next().unwrap() << i;
                        }
                        if *lenght_type_id == 0 {
                            // value = total width of all subpacket
                            let buf_size = value as isize;
                            let start: *const i64 = *cursor.peek().unwrap();

                            loop {
                                res += process_packet(cursor);

                                let pos: *const i64 = *cursor.peek().unwrap();
                                let sz = unsafe { pos.offset_from(start) };
                                assert!(sz <= buf_size);
                                if sz == buf_size { break; }
                            }
                        } else
                        if *lenght_type_id == 1 {
                            // value = number of subpacket
                            let nb_packet = value;
                            for _ in 0..nb_packet {
                                res += process_packet(cursor);
                            }
                        }
                    },
                }

                res
            }

            let version_sum = process_packet(&mut cursor);
            format!("{}", version_sum)
        } else {
            assert_eq!(input.lines().count(), 1);
            let input : Vec<i64> = input.lines()
                .next().unwrap()
                .chars()
                .map(|c| {
                    let v = i64::from_str_radix(&c.to_string(), 16).unwrap();
                    [v & (1 << 3),
                     v & (1 << 2),
                     v & (1 << 1),
                     v & (1 << 0)] })
                .flatten()
                .map(|x| if x > 0 { 1 } else { 0 })
                .collect();
            let mut cursor = input.iter().peekable();

            fn process_packet<'buf, 'it, T>(cursor: &'it mut Peekable<T>) -> i64
                where T: Iterator<Item = &'buf i64>,
                      T: Itertools

            {
                // version: 3 bit
                let (v1, v2, v3) = cursor.next_tuple().unwrap();
                let _version = (v1 << 2) + (v2 << 1) + v3;

                // type_id: 3 bit
                let (t1, t2, t3) = cursor.next_tuple().unwrap();
                let type_id = (t1 << 2) + (t2 << 1) + t3;
                if type_id == 4 { // litteral
                    let mut litteral = 0i64;
                    loop {
                        // group
                        let group_mark = cursor.next().unwrap();
                        for i in (0..4).rev() {
                            litteral += cursor.next().unwrap() << i;
                        }
                        if *group_mark == 0 { break; }
                        litteral <<= 4;
                    }
                    litteral
                } else { // operator
                    let lenght_type_id = cursor.next().unwrap();
                    let lenght_type_id_width = if *lenght_type_id == 1 { 11 } else { 15 };
                    let mut value = 0i64;
                    for i in (0..lenght_type_id_width).rev() {
                        value += cursor.next().unwrap() << i;
                    }
                    let mut values = Vec::new();
                    if *lenght_type_id == 0 {
                        // value = total width of all subpacket
                        let buf_size = value as isize;
                        let start: *const i64 = *cursor.peek().unwrap();

                        loop {
                            values.push(process_packet(cursor));

                            let pos: *const i64 = *cursor.peek().unwrap();
                            let sz = unsafe { pos.offset_from(start) };
                            assert!(sz <= buf_size);
                            if sz == buf_size { break; }
                        }
                    } else
                    if *lenght_type_id == 1 {
                        // value = number of subpacket
                        let nb_packet = value;
                        for _ in 0..nb_packet {
                            values.push(process_packet(cursor));
                        }
                    }
                    match type_id {
                        0 => values.iter().sum(),
                        1 => values.iter().product(),
                        2 => *values.iter().min().unwrap(),
                        3 => *values.iter().max().unwrap(),
                        5 => {
                            assert_eq!(values.len(), 2);
                            if values[0] > values[1] { 1 } else { 0 }
                        },
                        6 => {
                            assert_eq!(values.len(), 2);
                            if values[0] < values[1] { 1 } else { 0 }
                        },
                        7 => {
                            assert_eq!(values.len(), 2);
                            if values[0] == values[1] { 1 } else { 0 }
                        },
                        _ => panic!(""),
                    }
                }
            }

            let version_sum = process_packet(&mut cursor);
            format!("{}", version_sum)
        }
        17 => if part == 1 {
            assert_eq!(input.lines().count(), 1);
            assert!(input.lines().next().unwrap().starts_with("target area: "));
            let input = input.lines().next().unwrap().chars().collect::<String>();
            let input = input.replace("target area: x=", "");
            let input = input.replace("..", ",");
            let input = input.replace(", y=", ",");
            let (x1, x2, y1, y2) = input.split(",")
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
                    if pos.1 > high { high = pos.1; }

                    if pos.0 >= x1 && pos.0 <= x2
                    && pos.1 >= y1 && pos.1 <= y2 {
                        break (true, high);
                    }
                    if pos.0 > x2 || pos.1 < cmp::min(0, y2) {
                        break (false, high);
                    }

                    if x > 0 { x -= 1; }
                    // if x < 0 { x += 1; }
                    y -= 1;
                    if x == 0 && pos.0 < x1 { break (false, high); } // vertical fall
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
            let (x1, x2, y1, y2) = input.split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();
            println!("{}/{} {}/{}", x1, x2, y1, y2);


            let launch = |mut vx, mut vy| {
                let (mut x, mut y) = (0, 0);
                loop {
                    x += vx;
                    y += vy;

                    if vx > 0 { vx -= 1; }
                    if vx < 0 { vx += 1; }
                    vy -= 1;

                    if x >= x1 && x <= x2
                        && y >= y1 && y <= y2 {
                            break 1;
                    }
                    if (x < x1 && vx == 0)
                        || (y < y1 && vy < 0) {
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
        },
        18 => {
            #[derive(Clone, Copy, Eq)]
            struct Tok{d: u8, v: u32}
            impl PartialEq for Tok {
                fn eq(&self, other: &Tok) -> bool {
                    self.d == other.d && self.v == other.v
                }
            }

            #[derive(Eq, Clone)]
            struct Snail{a: Vec<Tok>}
            impl Snail {
                pub fn new() -> Self { Self { a: Vec::new() } }
                pub fn from_str(input : &'_ str) -> Self {
                    let mut res = Self::new();
                    let mut d = 0u8;
                    for c in input.chars() {
                        match c {
                            '[' => d += 1,
                            ']' => d -= 1,
                            ',' => (),
                            _ => {
                                // XXX use hexa to avoid complex parsing (used in tests)
                                let v = c.to_digit(16).unwrap() as u32;
                                res.a.push(Tok{d, v});
                            },
                        }
                    }
                    res
                }
                pub fn explode(&mut self) -> bool {
                    let n = self.a.len();
                    let deep = self.a.iter().enumerate().find_map(
                        |(idx, t)| if t.d == 5 { Some(idx) } else { None });
                    if deep.is_none() { return false; }
                    let idx = deep.unwrap();
                    assert_eq!(self.a[idx + 1].d, 5);
                    if idx != 0 {
                        self.a[idx - 1].v += self.a[idx].v;
                    }
                    if idx + 1 != n - 1 {
                        self.a[idx + 2].v += self.a[idx + 1].v;
                    }
                    self.a.remove(idx);
                    self.a[idx] = Tok{ d: 4, v: 0 };
                    true
                }
                pub fn split(&mut self) -> bool {
                    let big = self.a.iter().enumerate().find_map(
                        |(idx, t)| if t.v >= 10 { Some(idx) } else { None });
                    if big.is_none() { return false; }
                    let idx = big.unwrap();
                    let d = self.a[idx].d + 1;
                    let l = self.a[idx].v / 2;
                    let r = self.a[idx].v - l;
                    self.a[idx] = Tok{d, v: l};
                    self.a.insert(idx + 1, Tok{d, v: r});
                    true
                }
                pub fn reduce(&mut self) {
                    loop {
                        if self.explode() { continue; }
                        if self.split() { continue; }
                        break;
                    }
                }
                pub fn magnitude(&self) -> u32 {
                    let mut a = self.a.clone();
                    for d in (1..=4).rev() {
                        loop {
                            let mut modified = false;
                            for idx in 0..(a.len() - 1) {
                                if a[idx].d == d && a[idx + 1].d == d {
                                    a[idx] = Tok{d: d - 1,
                                                 v: 3 * a[idx].v + 2 * a[idx + 1].v};
                                    a.remove(idx + 1);
                                    modified = true;
                                    break;
                                }
                            }
                            if !modified { break; }
                        }
                    }
                    assert_eq!(a.len(), 1);
                    a[0].v
                }
                pub fn add_assign_magnitude(&self, other: &Self) -> u32 {
                    let mut w = self.clone();
                    w.a.extend(&other.a);
                    for i in w.a.iter_mut() {
                        i.d += 1;
                    }
                    w.reduce();
                    w.magnitude()
                }
            }
            impl PartialEq for Snail {
                fn eq(&self, other: &Snail) -> bool {
                    if self.a.len() != other.a.len() {
                        return false;
                    } else {
                        for i in 0..self.a.len() {
                            if self.a[i] != other.a[i] {
                                return false;
                            }
                        }
                    }
                    true
                }
            }
            impl Add for Snail {
                type Output = Self;

                fn add(self, other: Self) -> Self {
                    // XXX define neutral element to allow easy sum()
                    if self.a.len() == 0 { return other; }

                    let mut res = Self::new();
                    res.a.reserve(self.a.len() + other.a.len());
                    res.a.extend(self.a);
                    res.a.extend(other.a);

                    for i in res.a.iter_mut() {
                        i.d += 1;
                    }
                    res.reduce();
                    res
                }
            }
            impl Sum for Snail {
                fn sum<I>(iter: I) -> Self
                    where I: Iterator<Item = Self>
                {
                    iter.fold(Self::new(), |acc, x| acc + x)
                }
            }

            fn check_explode(before: &str, after: &str) {
                let mut a = Snail::from_str(before);
                a.explode();
                let b = Snail::from_str(after);
                assert!(a == b);
            }
            check_explode("[[[[[9,8],1],2],3],4]",
                          "[[[[0,9],2],3],4]");
            check_explode("[7,[6,[5,[4,[3,2]]]]]",
                          "[7,[6,[5,[7,0]]]]");
            check_explode("[[6,[5,[4,[3,2]]]],1]",
                          "[[6,[5,[7,0]]],3]");
            check_explode("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                          "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
            check_explode("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                          "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");

            fn check_split(before: &str, after: &str) {
                let mut a = Snail::from_str(before);
                a.split();
                let b = Snail::from_str(after);
                assert!(a == b);
            }

            check_split("[[[[0,7],4],[F,[0,D]]],[1,1]]",
                        "[[[[0,7],4],[[7,8],[0,D]]],[1,1]]");
            check_split("[[[[0,7],4],[[7,8],[0,D]]],[1,1]]",
                        "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");

            assert!(
                Snail::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]")
                + Snail::from_str("[1,1]")
                == Snail::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));

            fn check_sum(a: &[&str], sum: &str) {
                let a : Snail = a.iter().map(|x| Snail::from_str(x)).sum();
                let b = Snail::from_str(sum);
                assert!(a == b);
            }

            check_sum(&["[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]"],
                      "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
            check_sum(&["[1,1]", "[2,2]", "[3,3]", "[4,4]",],
                      "[[[[1,1],[2,2]],[3,3]],[4,4]]");
            check_sum(&["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]",],
                      "[[[[3,0],[5,3]],[4,4]],[5,5]]");
            check_sum(&["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]",],
                      "[[[[5,0],[7,4]],[5,5]],[6,6]]");
            check_sum(&["[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                        "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                        "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                        "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                        "[7,[5,[[3,8],[1,4]]]]",
                        "[[2,[2,2]],[8,[8,1]]]",
                        "[2,9]",
                        "[1,[[[9,3],9],[[9,0],[0,7]]]]",
                        "[[[5,[7,4]],7],1]",
                        "[[[[4,2],2],6],[8,7]]",],
                      "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");

            fn check_magnitude(a: &str, m: u32) {
                let a = Snail::from_str(a);
                assert_eq!(a.magnitude(), m);
            }

            check_magnitude("[[9,1],[1,9]]", 129);
            check_magnitude("[[1,2],[[3,4],5]]", 143);
            check_magnitude("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384);
            check_magnitude("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445);
            check_magnitude("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791);
            check_magnitude("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137);
            check_magnitude("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",3488);

            if part == 1 {
                let magnitude = input.lines()
                    .map(|l| Snail::from_str(l))
                    .sum::<Snail>()
                    .magnitude();

                format!("{}", magnitude)
            } else {
                let snails = input.lines()
                    .map(|l| Snail::from_str(l))
                    .collect::<Vec<Snail>>();

                let mut magnitude = 0;
                for a in 0..snails.len() {
                    for b in (a + 1)..snails.len() {
                        magnitude = cmp::max(
                            snails[a].add_assign_magnitude(&snails[b]),
                            magnitude);
                        magnitude = cmp::max(
                            snails[b].add_assign_magnitude(&snails[a]),
                            magnitude);
                    }
                }

                format!("{}", magnitude)
            }
        }
        19 => {
            // {{{ Pos
            #[derive(Clone, Copy, Eq, Hash, Debug)]
            struct Pos{x: i32, y: i32, z: i32}
            impl PartialEq for Pos {
                fn eq(&self, other: &Pos) -> bool {
                    self.x == other.x && self.y == other.y && self.z == other.z
                }
            }
            impl Pos {
                pub fn new(x: i32, y: i32, z: i32) -> Self { Self { x, y, z } }
                pub fn from_str(input : &'_ str) -> Self {
                    let (x, y, z) = input.split(',')
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    Pos::new(x, y, z)
                }
                pub fn distance(&self, other: &Self) -> i32 {
                    (self.x - other.x) * (self.x - other.x)
                    + (self.y - other.y) * (self.y - other.y)
                    + (self.z - other.z) * (self.z - other.z)
                }
                pub fn manhattan(&self, other: &Self) -> i32 {
                    (self.x - other.x).abs()
                    + (self.y - other.y).abs()
                    + (self.z - other.z).abs()
                }
            }
            impl fmt::Display for Pos {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "{}/{}/{}", self.x, self.y, self.z)
                }
            }
            // }}}
            // {{{ ScannerOriented
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            enum Up { X, Y, Z, XR, YR, ZR }
            #[derive(Clone, Copy, Debug, Eq)]
            struct ScannerOriented{
                offset: Pos, // in '0' referential
                dirx: bool, // means reverted if true
                diry: bool,
                dirz: bool,
                up: Up,
            }
            impl PartialEq for ScannerOriented {
                fn eq(&self, other: &ScannerOriented) -> bool {
                    self.dirx == other.dirx
                        && self.diry == other.diry
                        && self.dirz == other.dirz
                        && self.up == other.up
                }
            }
            impl Default for ScannerOriented {
                fn default() -> Self {
                    Self {
                        offset: Pos::new(0, 0, 0),
                        dirx: false,
                        diry: false,
                        dirz: false,
                        up: Up::Z, // choice
                    }
                 }
            }
            impl fmt::Display for ScannerOriented {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "offset:{} dir:{}/{}/{} up:{:?}",
                           self.offset,
                           self.dirx, self.diry, self.dirz,
                           self.up)
                }
            }
            impl ScannerOriented {
                pub fn all() -> Vec::<ScannerOriented> {
                    let mut res = Vec::with_capacity(24);

                    for dirx in [false, true] {
                        for diry in [false, true] {
                            for dirz in [false, true] {
                                for up in [Up::X, Up::Y, Up::Z, Up::XR, Up::YR, Up::ZR] {
                                    res.push(ScannerOriented{
                                        offset: Pos::new(0, 0, 0),
                                        dirx,
                                        diry,
                                        dirz,
                                        up,
                                    });
                                }
                            }
                        }
                    }
                    res
                }
                pub fn compute_offset(self, pos_in_0: &Pos, pos: &Pos) -> Pos {
                    let (x, y, z);
                    match self.up {
                        Up::Z => {
                            // same alignment
                            x = pos_in_0.x + if self.dirx { 1 } else { -1 } * pos.x;
                            y = pos_in_0.y + if self.diry { 1 } else { -1 } * pos.y;
                            z = pos_in_0.z + if self.dirz { 1 } else { -1 } * pos.z;
                        },
                        Up::Y => {
                            x = pos_in_0.x + if self.diry { 1 } else { -1 } * pos.y;
                            y = pos_in_0.y + if self.dirz { 1 } else { -1 } * pos.z;
                            z = pos_in_0.z + if self.dirx { 1 } else { -1 } * pos.x;
                        },
                        Up::X => {
                            x = pos_in_0.x + if self.dirz { 1 } else { -1 } * pos.z;
                            y = pos_in_0.y + if self.dirx { 1 } else { -1 } * pos.x;
                            z = pos_in_0.z + if self.diry { 1 } else { -1 } * pos.y;
                        },
                        Up::ZR => {
                            x = pos_in_0.x + if self.dirx { 1 } else { -1 } * pos.x;
                            y = pos_in_0.y + if self.diry { 1 } else { -1 } * pos.z;
                            z = pos_in_0.z + if self.dirz { 1 } else { -1 } * pos.y;
                        },
                        Up::YR => {
                            x = pos_in_0.x + if self.diry { 1 } else { -1 } * pos.y;
                            y = pos_in_0.y + if self.dirz { 1 } else { -1 } * pos.x;
                            z = pos_in_0.z + if self.dirx { 1 } else { -1 } * pos.z;
                        },
                        Up::XR => {
                            x = pos_in_0.x + if self.dirz { 1 } else { -1 } * pos.z;
                            y = pos_in_0.y + if self.dirx { 1 } else { -1 } * pos.y;
                            z = pos_in_0.z + if self.diry { 1 } else { -1 } * pos.x;
                        },
                    }
                    Pos::new(x, y, z)
                }
                pub fn add_offset(self, pos: &Pos) -> Pos {
                    let (x, y, z);
                    match self.up {
                        Up::Z => {
                            // same alignment
                            x = self.offset.x + if self.dirx { -1 } else { 1 } * pos.x;
                            y = self.offset.y + if self.diry { -1 } else { 1 } * pos.y;
                            z = self.offset.z + if self.dirz { -1 } else { 1 } * pos.z;
                        },
                        Up::Y => {
                            x = self.offset.x + if self.diry { -1 } else { 1 } * pos.y;
                            y = self.offset.y + if self.dirz { -1 } else { 1 } * pos.z;
                            z = self.offset.z + if self.dirx { -1 } else { 1 } * pos.x;
                        },
                        Up::X => {
                            x = self.offset.x + if self.dirz { -1 } else { 1 } * pos.z;
                            y = self.offset.y + if self.dirx { -1 } else { 1 } * pos.x;
                            z = self.offset.z + if self.diry { -1 } else { 1 } * pos.y;
                        },
                        Up::ZR => {
                            x = self.offset.x + if self.dirx { -1 } else { 1 } * pos.x;
                            y = self.offset.y + if self.diry { -1 } else { 1 } * pos.z;
                            z = self.offset.z + if self.dirz { -1 } else { 1 } * pos.y;
                        },
                        Up::YR => {
                            x = self.offset.x + if self.diry { -1 } else { 1 } * pos.y;
                            y = self.offset.y + if self.dirz { -1 } else { 1 } * pos.x;
                            z = self.offset.z + if self.dirx { -1 } else { 1 } * pos.z;
                        },
                        Up::XR => {
                            x = self.offset.x + if self.dirz { -1 } else { 1 } * pos.z;
                            y = self.offset.y + if self.dirx { -1 } else { 1 } * pos.y;
                            z = self.offset.z + if self.diry { -1 } else { 1 } * pos.x;
                        },
                    }
                    Pos::new(x, y, z)
                }
            }
            // }}}
            // {{{ Scanner
            #[derive(Clone, Debug)]
            struct Scanner{
                a: Vec<Pos>,
                oriented: Option<ScannerOriented>,
                real: Vec<Option<Pos>>,
            }
            impl fmt::Display for Scanner{
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    for i in 0..self.a.len() {
                        write!(f, "{}:{}", i, self.a[i]).unwrap();
                        if let Some(real) = self.real[i] {
                            write!(f, "(r:{})", real).unwrap();
                        }
                        writeln!(f, "").unwrap();
                    }
                    if let Some(oriented) = self.oriented {
                        writeln!(f, "oriented:{}", oriented).unwrap();
                    }
                    Ok(())
                }
            }
            impl Scanner {
                pub fn new(a: Vec<Pos>) -> Self {
                    let mut real = Vec::with_capacity(a.len());
                    real.resize(a.len(), None);
                    Self {
                        a,
                        oriented: None,
                        real,
                    }
                }
                pub fn from_str(input : &'_ str) -> Vec<Self> {
                    let mut res = Vec::new();
                    let mut a = Vec::new();

                    for line in input.lines()
                                     .filter(|l| l.len() > 0)
                    {
                        if line.starts_with("--- scanner ") {
                            if a.len() > 0 {
                                res.push(Scanner::new(a.clone()));
                                a.clear();
                            }
                            let id = line.split_whitespace()
                                         .skip(2)
                                         .next()
                                         .unwrap()
                                         .parse::<usize>()
                                         .unwrap();
                            // sanity check
                            assert_eq!(id, res.len());
                        } else {
                            a.push(Pos::from_str(line));
                        }
                    }
                    res.push(Scanner::new(a));

                    res
                }
                pub fn beacon_distances(&self, from: usize) -> HashMap::<i32, usize> {
                    let mut res = HashMap::new();
                    for to in 0..self.a.len() {
                        if from == to { continue; }
                        let d = self.a[from].distance(&self.a[to]);
                        *res.entry(d).or_insert(0) += 1;
                    }
                    res
                }
                pub fn compute_compatible(scanners: &mut Vec<Self>,
                                          idref: usize, idcand: usize,
                                          uniques: &Vec::<HashSet::<((usize, usize), Pos)>>)
                {
                    assert!(scanners[idref].oriented.is_some());
                    let all = ScannerOriented::all();
                    let mut res = Vec::new();
                    for cand in &all {
                        // println!("cand {}", cand);
                        let mut offsets = HashSet::new();
                        for unique in uniques {
                            let refpos = unique.iter()
                                .filter(|(id, _)| id.0 == idref)
                                // .map(|(_, pos)| pos)
                                .map(|(id, _)| scanners[id.0].real[id.1].unwrap())
                                .next();
                            if refpos.is_none() { continue; }
                            let refpos = refpos.unwrap();

                            let candpos = unique.iter()
                                .filter(|(id, _)| id.0 == idcand)
                                .map(|(_, pos)| pos)
                                .next();
                            if candpos.is_none() { continue; }
                            let candpos = candpos.unwrap();

                            let offset = cand.compute_offset(&refpos, candpos);
                            // println!("{} -> {} so {}", refpos, candpos, offset);
                            offsets.insert(offset);
                        }
                        // for (idx, offset) in offsets.iter().enumerate() {
                        //     println!("{}; {}", idx, offset);
                        // }
                        if offsets.len() == 1 {
                            let mut cand = *cand;
                            let offset = *offsets.iter().next().unwrap();
                            cand.offset = offset;
                            res.push(cand);
                        }
                    }
                    if res.len() == 1 {
                        // println!("find {}/{} -> {}",
                        //          idref, idcand, res[0]);
                        for idx in 0..scanners[idcand].a.len() {
                            scanners[idcand].real[idx] =
                                Some(res[0].add_offset(
                                        &scanners[idcand].a[idx]));
                        }
                        scanners[idcand].oriented = Some(res[0]);
                    } else {
                        println!("no solution for {}/{}", idref, idcand);
                    }
                }
            }
            // }}}

            let mut scanners = Scanner::from_str(input);
            let n = scanners.len();

            let mut internal_distances = HashMap::new();
            for (id, scanner) in scanners.iter().enumerate() {
                for from in 0..scanner.a.len() {
                    internal_distances.insert((id, from),
                                              scanner.beacon_distances(from));
                }
            }
            let mut commons = HashMap::<(usize, usize), usize>::new();
            let mut uniques = Vec::<HashSet::<((usize, usize), Pos)>>::new();
            let mut marry = |from_id, from, from_pos, to_id, to, to_pos| {
                let from = (from_id, from);
                let to = (to_id, to);
                let id =
                    if let Some(id) = commons.get(&from) {
                        *id
                    } else
                    if let Some(id) = commons.get(&to) {
                        *id
                    } else {
                        let id = uniques.len();
                        uniques.push(HashSet::new());
                        id
                    };
                uniques[id].insert((to, to_pos));
                uniques[id].insert((from, from_pos));
                commons.insert(from, id);
                commons.insert(to, id);
            };
            for (id, scanner) in scanners.iter().enumerate() {
                for (from, from_pos) in scanner.a.iter().enumerate() {
                    let distfrom = internal_distances.get(&(id, from)).unwrap();
                    let distfromk = distfrom.keys().collect::<HashSet::<_>>(); // inefficient
                    for id2 in (id + 1)..n {
                        for (to, to_pos) in scanners[id2].a.iter().enumerate() {
                            let distto = internal_distances.get(&(id2, to)).unwrap();
                            let disttok = distto.keys().collect::<HashSet::<_>>(); // inefficient
                            let common = distfromk.intersection(&disttok)
                                .map(|k| distto[k] + distfrom[k])
                                .sum::<usize>();
                            if common >= 22 {
                                marry(id, from, *from_pos, id2, to, *to_pos);
                            }
                        }
                    }
                }
            }
            drop(marry);
            // for (id, _unique) in uniques.iter().enumerate() {
            //     println!("{}: {}", id, itertools::join(
            //             unique.iter().map(|(id, pos)| format!("{}/{} ({})", id.0, id.1, pos)),
            //             ", "));
            // }
            let _count = uniques.iter().map(|x| x.len()).sum::<usize>();
            let _source = scanners.iter().map(|x| x.a.len()).sum::<usize>();
            let _unique = uniques.len();
            // println!("mapped:{} (in {}) initial:{}", _count, _unique, _source);

            let mut scannerref = 0;
            scanners[0].oriented = Some(ScannerOriented::default());
            for idx in 0..scanners[0].a.len() {
                scanners[0].real[idx] = Some(scanners[0].a[idx]);
            }
            loop {
                if (0..n).filter(|x| scanners[*x].oriented == None).count() == 0 {
                    break;
                }
                let (best_id, best_common, _best_uniques) = (0..n)
                    .filter(|x| *x != scannerref)
                    .filter(|x| scanners[*x].oriented == None)
                    .map(|x| {
                        let c = uniques.iter().enumerate()
                            .filter_map(|(uid, poss)| {
                                let ref_count = poss.iter()
                                    .filter(|(id, _)| id.0 == scannerref)
                                    .count();
                                let cand_count = poss.iter()
                                    .filter(|(id, _)| id.0 == x)
                                    .count();
                                assert!(ref_count <= 1 && cand_count <= 1);
                                if ref_count == 1 && cand_count == 1
                                { Some(uid) } else { None }
                            })
                            .collect::<Vec<_>>();
                        (x, c.len(), c) })
                    .max_by(|(_, ca, _), (_, cb, _)| ca.cmp(&cb))
                    .unwrap();
                if best_common == 0 {
                    scannerref = (0..n)
                        .map(|x| (x + 1 + scannerref) % scanners.len())
                        .filter(|x| scanners[*x].oriented.is_some()).next().unwrap();
                    continue;
                }
                // println!("best with {} is {} with {} common: {}",
                //          scannerref,
                //          best_id, best_common,
                //          itertools::join(_best_uniques.iter()
                //                          .map(|uid| format!("{}", uid)),
                //                          ", "));
                Scanner::compute_compatible(&mut scanners, scannerref, best_id, &uniques);
                scannerref = best_id;
            }

            let mut beacon = HashSet::<Pos>::new();
            for scanner in scanners.iter() {
                for pos in scanner.real.iter() {
                    beacon.insert(pos.unwrap());
                }
            }
            let res = beacon.len();

            if part == 1 {
                format!("{}", res)
            } else {
                let pos = scanners.iter()
                    .map(|x| x.oriented.unwrap().offset)
                    .collect::<Vec<_>>();
                let far = (0..n)
                    .map(|x| (0..n)
                         .filter(|y| *y != x)
                         .map(|y| pos[x].manhattan(&pos[y]))
                         .max()
                         .unwrap())
                    .max()
                    .unwrap();
                format!("{}", far)
            }
        }
        20 => if part == 1 {
            pub type T = u16;
            let mut line = input.lines();
            let algo = line.next().unwrap()
                .chars()
                .map(|c| if c == '#' { 1 } else { 0 })
                .collect::<Vec<T>>();
            // dbg!(algo.len());

            let w = input.lines().skip(2).next().unwrap().chars().count();
            let h = input.lines().skip(2).count();
            // dbg!(w);
            // dbg!(h);
            assert_eq!(w, h);
            let n = w;
            let sz = n * n;

            let mut grid = Vec::new();
            grid.resize(sz, 0 as T);
            let idx_of = |x, y| x + n * y;

            let mut idx = 0;
            for line in input.lines().skip(2) {
                for c in line.chars() {
                    grid[idx] = if c == '#' { 1 as T } else { 0 };
                    idx += 1;
                }
            }
            assert_eq!(idx, sz);

            // extend
            let mut egrid = Vec::new();
            let extra = 10;
            let en = n + 2 * extra;
            let esz = en * en;
            egrid.resize(esz, 0 as T);
            let eidx_of = |x, y| x + en * y;
            for col in 0..n {
                for row in 0..n {
                    egrid[eidx_of(extra + col, extra + row)] = grid[idx_of(col, row)];
                }
            }
            drop(grid);
            let n = en;
            let sz = esz;
            let idx_of = |x, y| x + n * y;
            let grid = egrid;

            // algo
            assert_eq!(algo[0], 1); // 9*0 became 1
            assert_eq!(algo[511], 0); // 9*1 became 0, so infinite is 2 algo stable
            let mut agrid = Vec::new();
            agrid.resize(sz, algo[0] as T); // initial infinite 0
            for col in 1..(n - 1) {
                for row in 1..(n - 1) {
                    let mut sum = 0;
                    sum += grid[idx_of(col - 1, row - 1)] << 8;
                    sum += grid[idx_of(col - 0, row - 1)] << 7;
                    sum += grid[idx_of(col + 1, row - 1)] << 6;
                    sum += grid[idx_of(col - 1, row + 0)] << 5;
                    sum += grid[idx_of(col - 0, row + 0)] << 4;
                    sum += grid[idx_of(col + 1, row + 0)] << 3;
                    sum += grid[idx_of(col - 1, row + 1)] << 2;
                    sum += grid[idx_of(col - 0, row + 1)] << 1;
                    sum += grid[idx_of(col + 1, row + 1)] << 0;
                    assert!(sum < 512);
                    agrid[idx_of(col, row)] = algo[sum as usize];
                }
            }

            let mut res = 0;
            for col in 1..(n - 1) {
                for row in 1..(n - 1) {
                    let mut sum = 0;
                    sum += agrid[idx_of(col - 1, row - 1)] << 8;
                    sum += agrid[idx_of(col - 0, row - 1)] << 7;
                    sum += agrid[idx_of(col + 1, row - 1)] << 6;
                    sum += agrid[idx_of(col - 1, row + 0)] << 5;
                    sum += agrid[idx_of(col - 0, row + 0)] << 4;
                    sum += agrid[idx_of(col + 1, row + 0)] << 3;
                    sum += agrid[idx_of(col - 1, row + 1)] << 2;
                    sum += agrid[idx_of(col - 0, row + 1)] << 1;
                    sum += agrid[idx_of(col + 1, row + 1)] << 0;
                    assert!(sum < 512);
                    res += algo[sum as usize];
                }
            }

            // 5231
            format!("{}", res)
        } else {
            pub type T = u16;
            let mut line = input.lines();
            let algo = line.next().unwrap()
                .chars()
                .map(|c| if c == '#' { 1 } else { 0 })
                .collect::<Vec<T>>();
            // dbg!(algo.len());

            let w = input.lines().skip(2).next().unwrap().chars().count();
            let h = input.lines().skip(2).count();
            // dbg!(w);
            // dbg!(h);
            assert_eq!(w, h);
            let n = w;
            let sz = n * n;

            let mut grid = Vec::new();
            grid.resize(sz, 0 as T);
            let idx_of = |x, y| x + n * y;

            let mut idx = 0;
            for line in input.lines().skip(2) {
                for c in line.chars() {
                    grid[idx] = if c == '#' { 1 as T } else { 0 };
                    idx += 1;
                }
            }
            assert_eq!(idx, sz);

            // extend
            let mut egrid = Vec::new();
            let extra = 60;
            let en = n + 2 * extra;
            let esz = en * en;
            egrid.resize(esz, 0 as T);
            let eidx_of = |x, y| x + en * y;
            for col in 0..n {
                for row in 0..n {
                    egrid[eidx_of(extra + col, extra + row)] = grid[idx_of(col, row)];
                }
            }
            drop(grid);
            let n = en;
            let sz = esz;
            let idx_of = |x, y| x + n * y;
            let mut grid = egrid;
            let mut agrid = Vec::new();
            let mut res = 0;
            for step in 0..50 {
                agrid.resize(sz, algo[step % 2] as T); // initial infinite 0
                res = 0;
                for col in 1..(n - 1) {
                    for row in 1..(n - 1) {
                        let mut sum = 0;
                        sum += grid[idx_of(col - 1, row - 1)] << 8;
                        sum += grid[idx_of(col - 0, row - 1)] << 7;
                        sum += grid[idx_of(col + 1, row - 1)] << 6;
                        sum += grid[idx_of(col - 1, row + 0)] << 5;
                        sum += grid[idx_of(col - 0, row + 0)] << 4;
                        sum += grid[idx_of(col + 1, row + 0)] << 3;
                        sum += grid[idx_of(col - 1, row + 1)] << 2;
                        sum += grid[idx_of(col - 0, row + 1)] << 1;
                        sum += grid[idx_of(col + 1, row + 1)] << 0;
                        assert!(sum < 512);
                        agrid[idx_of(col, row)] = algo[sum as usize];
                        res += algo[sum as usize]; // valid only on pair step
                    }
                }
                // println!("step:{} res:{}", step, res);
                std::mem::swap(&mut agrid, &mut grid);
            }

            format!("{}", res)
        },
        21 => {
            assert_eq!(input.lines().count(), 2);
            assert!(input.lines().next().unwrap().starts_with("Player 1 starting position: "));
            assert!(input.lines().skip(1).next().unwrap().starts_with("Player 2 starting position: "));
            let mut pos1 = input.lines().next().unwrap()
                .replace("Player 1 starting position: ", "")
                .parse::<u64>()
                .unwrap();
            let mut pos2 = input.lines().skip(1).next().unwrap()
                .replace("Player 2 starting position: ", "")
                .parse::<u64>()
                .unwrap();

            if part == 1 {
                let mut dice = (1..).map(|x| x * 3).map(|x| x + (x - 1) + (x - 2));
                let mut n = 0;

                let (mut score1, mut score2) = (0u64, 0u64);
                loop {
                    pos1 += dice.next().unwrap();
                    n += 3;
                    pos1 = 1 + (pos1 - 1) % 10; // [1-10] loop
                    score1 += pos1;
                    if score1 >= 1000 { break; }

                    pos2 += dice.next().unwrap();
                    n += 3;
                    pos2 = 1 + (pos2 - 1) % 10; // [1-10] loop
                    score2 += pos2;
                    if score2 >= 1000 { break; }
                }
                // dbg!(score1, score2, n);

                format!("{}", cmp::min(score1, score2) * n)
            } else {
                fn round10(v: u64) -> u64 { 1 + (v - 1) % 10 }

                let mut score = HashMap::new();
                score.insert(((pos1, 0), (pos2, 0)), 1u128);
                let mut scorenext = HashMap::new();
                let (mut win1, mut win2) = (0u128, 0u128);
                loop {
                    for (((pos1, score1), (pos2, score2)), n) in score.iter() {
                        for a1 in [1, 2, 3] {
                            for b1 in [1, 2, 3] {
                                for c1 in [1, 2, 3] {
                                    let pos1 = round10(pos1 + a1 + b1 + c1);
                                    let score1 = *score1 + pos1;
                                    if score1 >= 21 {
                                        win1 += *n;
                                    } else {
                                        for a2 in [1, 2, 3] {
                                            for b2 in [1, 2, 3] {
                                                for c2 in [1, 2, 3] {
                                                    let pos2 = round10(pos2 + a2 + b2 + c2);
                                                    let score2 = *score2 + pos2;
                                                    if score2 >= 21 {
                                                        win2 += *n;
                                                    } else {
                                                        *scorenext.entry(((pos1, score1), (pos2, score2))).or_default() += *n;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if scorenext.len() == 0 {
                        break;
                    } else {
                        // dbg!(scorenext.len());
                        // dbg!(win1);
                        // dbg!(win2);
                        score.clear();
                        std::mem::swap(&mut score, &mut scorenext);
                    }
                }
                // dbg!(win1, win2);

                format!("{}", cmp::max(win1, win2))
            }
        }
        22 => {
            let mut xs = HashSet::new();
            let mut ys = HashSet::new();
            let mut zs = HashSet::new();
            let mut steps = Vec::new();
            for line in input.lines() {
                assert!(line.starts_with("on") || line.starts_with("of"));
                let on = if line.starts_with("on") { 1u8 } else { 0u8 };
                let line = line.replace("on ", "")
                               .replace("off ", "");
                let (mut x1, mut x2) = (0, 0);
                let (mut y1, mut y2) = (0, 0);
                let (mut z1, mut z2) = (0, 0);
                for tok in line.split(",") {
                    if tok.starts_with("x=") {
                        let (a, b) = tok.replace("x=", "")
                                        .split("..")
                                        .map(|x| x.parse::<i64>().unwrap())
                                        .collect_tuple()
                                        .unwrap();
                        x1 = a;
                        x2 = b;
                    }
                    if tok.starts_with("y=") {
                        let (a, b) = tok.replace("y=", "")
                                        .split("..")
                                        .map(|x| x.parse::<i64>().unwrap())
                                        .collect_tuple()
                                        .unwrap();
                        y1 = a;
                        y2 = b;
                    }
                    if tok.starts_with("z=") {
                        let (a, b) = tok.replace("z=", "")
                                        .split("..")
                                        .map(|x| x.parse::<i64>().unwrap())
                                        .collect_tuple()
                                        .unwrap();
                        z1 = a;
                        z2 = b;
                    }
                }
                if part == 2 || (x1.abs() <= 50 && x2.abs() <= 50
                                 && y1.abs() <= 50 && y2.abs() <= 50
                                 && z1.abs() <= 50 && z2.abs() <= 50)
                {
                    steps.push((on, x1, x2, y1, y2, z1, z2));
                    xs.insert(x1);
                    xs.insert(x2 + 1);
                    ys.insert(y1);
                    ys.insert(y2 + 1);
                    zs.insert(z1);
                    zs.insert(z2 + 1);
                }
            }
            let mut xs = xs.iter().cloned().sorted().collect::<Vec<_>>();
            let mut ys = ys.iter().cloned().sorted().collect::<Vec<_>>();
            let mut zs = zs.iter().cloned().sorted().collect::<Vec<_>>();
            xs.push(xs.last().unwrap() + 1);
            ys.push(ys.last().unwrap() + 1);
            zs.push(zs.last().unwrap() + 1);
            let xs = xs;
            let ys = ys;
            let zs = zs;

            let idx_x = |x| xs.iter().enumerate()
                    .find_map(|(i, v)| if *v == x { Some(i) } else { None })
                    .unwrap();
            let idx_y = |y| ys.iter().enumerate()
                    .find_map(|(i, v)| if *v == y { Some(i) } else { None })
                    .unwrap();
            let idx_z = |z| zs.iter().enumerate()
                    .find_map(|(i, v)| if *v == z { Some(i) } else { None })
                    .unwrap();

            let nx = xs.len();
            let ny = ys.len();
            let nz = zs.len();
            let sz = nx * ny * nz;
            let mut grid = Vec::new();
            grid.resize(sz, 0u8);
            let idx_of = |x, y, z| x + nx * y + (nx * ny) * z;

            for step in steps {
                let (on, x1, x2, y1, y2, z1, z2) = step;

                for x in idx_x(x1)..idx_x(x2 + 1) {
                    for y in idx_y(y1)..idx_y(y2 + 1) {
                        for z in idx_z(z1)..idx_z(z2 + 1) {
                            grid[idx_of(x, y, z)] = on;
                        }
                    }
                }
            }

            let mut sum = 0;
            for idx in 0..(xs.len() - 2) {
                for idy in 0..(ys.len() - 2) {
                    for idz in 0..(zs.len() - 2) {
                        if grid[idx_of(idx, idy, idz)] == 1 {
                            sum += (xs[idx + 1] - xs[idx])
                                * (ys[idy + 1] - ys[idy])
                                * (zs[idz + 1] - zs[idz]);
                        }
                    }
                }
            }
            format!("{}", sum)
        }
        23 => if part == 1 {
            assert_eq!(input.lines().count(), 5);
            assert_eq!(input.lines().nth(1).unwrap(), "#...........#");
            let row1 = input.lines().nth(2).unwrap()
                .to_string()
                .replace("#", "")
                .replace(" ", "");
            assert_eq!(row1.len(), 4);
            let row2 = input.lines().nth(3).unwrap()
                .to_string()
                .replace("#", "")
                .replace(" ", "");
            assert_eq!(row2.len(), 4);

            const N : usize = 7 + 4 * 2; // 7 free hallway points then rooms sequentially
            // #############
            // #01.2.3.4.56#
            // ###7#9# # ###
            //   #8# # # #
            //   #########
            type D = i8;
            let mut distances : [[D; N]; N] = [
              // 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14
                [0, 1, 3, 5, 7, 9,10, 3, 4, 5, 6, 7, 8, 9,10], // 0
                [0, 0, 2, 4, 6, 8, 9, 2, 3, 4, 5, 6, 7, 8, 9], // 1
                [0, 0, 0, 2, 4, 6, 7, 2, 3, 2, 3, 4, 5, 6, 7], // 2
                [0, 0, 0, 0, 2, 4, 5, 4, 5, 2, 3, 2, 3, 4, 5], // 3
                [0, 0, 0, 0, 0, 2, 3, 6, 7, 4, 5, 2, 3, 2, 3], // 4
                [0, 0, 0, 0, 0, 0, 1, 8, 9, 6, 7, 4, 5, 2, 3], // 5
                [0, 0, 0, 0, 0, 0, 0, 9,10, 7, 8, 5, 6, 3, 4], // 6
                [0, 0, 0, 0, 0, 0, 0, 0, 1, 4, 5, 6, 7, 8, 9], // 7
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 9,10], // 8
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 4, 5, 6, 7], // 9
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8], // 10
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 4, 5], // 11
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6], // 12
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], // 13
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 14
                ];
            for row in 1..N {
                for col in 0..row {
                    assert_eq!(distances[row][col], 0);
                    distances[row][col] = distances[col][row];
                }
            }
            for dst in (0..N).filter(|n| ![0, 6].contains(n)) {
                assert_eq!(distances[0][dst], 1 + distances[1][dst]);
                assert_eq!(distances[6][dst], 1 + distances[5][dst]);
            }
            for src in [7, 9, 11, 13] {
                assert_eq!(distances[src][src + 1], 1);
            }
            for src in 0..7 {
                for dst in [7, 9, 11, 13] {
                    assert_eq!(distances[src][dst + 1], distances[src][dst] + 1);
                    assert!(distances[src][dst] >= 2);
                }
            }
            let distances = distances;
            const NEIGHBOURS : [u32; N] = [
                1 << 1,                               // 0
                1 << 0  | 1 << 2  | 1 << 7,           // 1
                1 << 1  | 1 << 7  | 1 << 9  | 1 << 3, // 2
                1 << 2  | 1 << 9  | 1 << 11 | 1 << 4, // 3
                1 << 3  | 1 << 11 | 1 << 13 | 1 << 5, // 4
                1 << 4  | 1 << 13 | 1 << 6,           // 5
                1 << 5,                               // 6
                1 << 1  | 1 << 2  | 1 << 8,           // 7
                1 << 7,                               // 8
                1 << 2  | 1 << 3  | 1 << 10,          // 9
                1 << 9,                               // 10
                1 << 3  | 1 << 4  | 1 << 12,          // 11
                1 << 11,                              // 12
                1 << 4  | 1 << 5  | 1 << 14,          // 13
                1 << 13,                              // 14
            ];
            for (idx, v) in NEIGHBOURS.iter().enumerate() {
                assert_eq!((1 << idx) & *v, 0);
                for n in 0..N {
                    if idx != n && 1 << n & *v != 0 {
                        assert!(distances[idx][n] > 0);
                        assert!(NEIGHBOURS[n] & 1 << idx != 0);
                    } else {
                        assert!(NEIGHBOURS[n] & 1 << idx == 0);
                    }
                }
            }
            // dirty manual dijkstra to build path masks
            let mut masks = [[0u32; N]; N];
            for src in 0..N {
                let mut dists = [u32::MAX; N];
                let mut nodes = vec![src];
                let mut nnodes = Vec::new();
                dists[src] = 0;
                // extend
                loop {
                    nnodes.clear();
                    for node in &nodes {
                        for n in (0..N).filter(|x| 1 << *x & NEIGHBOURS[*node] != 0) {
                            if n != *node && dists[n] == u32::MAX { nnodes.push(n); }
                            dists[n] = std::cmp::min(dists[*node] + 1, dists[n]);
                        }
                    }
                    if nnodes.len() == 0 { break; }
                    std::mem::swap(&mut nodes, &mut nnodes);
                }
                let nmasks = &mut masks[src];
                // paths back
                loop {
                    if let Some(dst) = (0..N).filter(|x| *x != src)
                                             .filter(|x| nmasks[*x] == 0)
                                             .max_by(|a, b| dists[*a].cmp(&dists[*b])) {
                        let mut n = dst;
                        let mut dist = dists[dst]; // XXX rust bug if let Some((dst, mut dist)) = 
                        loop {
                            nmasks[dst] |= 1 << n;
                            dist -= 1;
                            n = (0..N).filter(|x| 1 << *x & NEIGHBOURS[n] != 0)
                                      .filter(|x| dists[*x] == dist)
                                      .next().unwrap();
                            if n == src { break; }
                        }

                    } else { break; }
                }
            }
            let masks = masks;

            #[derive(Copy, Clone, Eq, PartialEq, Debug)]
            enum NodeLife { Start, Hallway, Room, }
            #[derive(Copy, Clone, Eq, PartialEq, Debug)]
            struct Node {
                pub occ: char,
                pub life: NodeLife,
            }
            impl Node {
                pub fn new(occ: char) -> Self {
                    Self {
                        occ,
                        life: NodeLife::Start,
                    }
                }
            }
            impl Default for Node {
                fn default() -> Self {
                    Self::new('.')
                }
            }


            type C = u64;
            #[derive(Copy, Clone, Eq, PartialEq, Debug)]
            pub struct State {
                nodes: [Node; N],
                cost: C,
                distance: C, // optimistic measure by design
                // tmp: [(char, u8, u8, /*u16, u16*/); 20],
            }
            impl State {
                pub fn mask(&self) -> u32 {
                    let mut res = 0u32;
                    for idx in 0..N {
                        if self.nodes[idx].occ != '.' {
                            res |= 1 << idx;
                        }
                    }
                    res
                }
                pub fn distance(&self, distances: &'static [[D; N]; N]) -> C {
                    let mut res = 0;
                    for idx in 0..N {
                        if self.nodes[idx].occ != '.' {
                            let targets = match self.nodes[idx].occ {
                                'A' => &[ 7, 8],
                                'B' => &[ 9, 10],
                                'C' => &[11, 12],
                                'D' => &[13, 14],
                                _ => panic!(),
                            };
                            let cost = match self.nodes[idx].occ {
                                'A' => 1,
                                'B' => 10,
                                'C' => 100,
                                'D' => 1000,
                                _ => panic!(),
                            };
                            res += cost * cmp::min(distances[idx][targets[0]],
                                                   distances[idx][targets[1]]) as C;
                        }
                    }
                    res
                }
                pub fn aligned(&self) -> bool {
                    for idx in [7, 9, 11, 13] {
                        if self.nodes[idx].occ == '.'
                        || self.nodes[idx].occ != self.nodes[idx + 1].occ
                        || self.nodes[idx].occ != match idx {
                             7 => 'A',
                             9 => 'B',
                            11 => 'C',
                            13 => 'D',
                            _ => panic!(),
                        }
                        {
                            return false;
                        }
                    }
                    return true;
                }
            }
            impl Default for State {
                fn default() -> Self {
                    Self {
                        nodes: [Node::default(); N],
                        cost: 0,
                        distance: C::MAX,
                        // tmp: [(' ', 0, 0, /*0, 0*/); 20],
                    }
                }
            }
            impl fmt::Display for State {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "#############").unwrap();
                    if self.cost != C::MAX {
                        write!(f, " ({:9})", self.cost).unwrap();
                    }
                    // for (idx, (c, src, dst, /*dist, cost*/)) in self.tmp.iter().enumerate() {
                    //     if *c != ' ' {
                    //         // write!(f, " {}:{}{}→{}({}:{})", idx, *c, src, dst, dist, cost).unwrap();
                    //         write!(f, " {}:{}{}→{}", idx, *c, src, dst).unwrap();
                    //     }
                    // }

                    writeln!(f, "\n#{}{}.{}.{}.{}.{}{}#",
                        self.nodes[0].occ,
                        self.nodes[1].occ,
                        self.nodes[2].occ,
                        self.nodes[3].occ,
                        self.nodes[4].occ,
                        self.nodes[5].occ,
                        self.nodes[6].occ).unwrap();
                    writeln!(f, "###{}#{}#{}#{}###",
                        self.nodes[7].occ,
                        self.nodes[9].occ,
                        self.nodes[11].occ,
                        self.nodes[13].occ).unwrap();
                    writeln!(f, "  #{}#{}#{}#{}#",
                        self.nodes[8].occ,
                        self.nodes[10].occ,
                        self.nodes[12].occ,
                        self.nodes[14].occ)
                }
            }
            impl Ord for State {
                fn cmp(&self, other: &State) -> cmp::Ordering {
                    // finished first, then low depths (dijkstra like)
                    match (self.aligned(), other.aligned()) {
                        (true, true) => other.cost.cmp(&self.cost),
                        (true, false) => cmp::Ordering::Greater,
                        (false, true) => cmp::Ordering::Less,
                        (false, false) => other.distance.cmp(&self.distance)
                            .then_with(|| other.cost.cmp(&self.cost))
                    }
                }
            }
            impl PartialOrd for State {
                fn partial_cmp(&self, other: &State) -> Option<cmp::Ordering> {
                    Some(self.cmp(other))
                }
            }

            let mut start = State::default();
            let mut row1 = row1.chars();
            let mut row2 = row2.chars();
            for idx in 7..N {
                let occ = if idx % 2 == 1 { row1.next() } else { row2.next() };
                start.nodes[idx] = Node::new(occ.unwrap());
            }
            let start = start;

            pub struct IntoIteratorState {
                this: State,
                mask: u32,
                nodemoving: usize,
                nodetarget: usize,
                cost_clip: C,
                masks: &'static [[u32; N]; N], // need GAT
                distances: &'static [[D; N]; N],
            }
            impl State {
                fn into_iter(self,
                             cost_clip: C,
                             masks: &'static[[u32; N]; N],
                             distances: &'static [[D; N]; N]) -> IntoIteratorState {
                    IntoIteratorState {
                        this: self,
                        mask: self.mask(),
                        nodemoving: 0,
                        nodetarget: 0,
                        cost_clip,
                        masks,
                        distances,
                    }
                }
            }
            impl Iterator for IntoIteratorState {
                type Item = State;

                fn next(&mut self) -> Option<Self::Item> {
                    loop {
                        if self.nodemoving == N {
                            return None;
                        }
                        let src = &self.this.nodes[self.nodemoving];
                        if src.occ == '.'
                        || src.life == NodeLife::Room
                        || self.nodetarget == N
                        {
                            self.nodemoving += 1;
                            self.nodetarget = 0;
                            continue;
                        }
                        if self.nodetarget == self.nodemoving
                        || self.this.nodes[self.nodetarget].occ != '.'
                        || self.masks[self.nodemoving][self.nodetarget] & self.mask != 0
                        || (src.life == NodeLife::Start && self.nodetarget >= 7)
                        || (src.life == NodeLife::Hallway && self.nodetarget < 7)
                        || (src.life == NodeLife::Hallway
                           && src.occ != match self.nodetarget {
                                 7 |  8 => 'A',
                                 9 | 10 => 'B',
                                11 | 12 => 'C',
                                13 | 14 => 'D',
                                _ => panic!(), })
                        || (src.life == NodeLife::Hallway
                           && self.nodetarget % 2 == 1
                           && self.this.nodes[self.nodetarget + 1].occ != src.occ)
                        {
                            self.nodetarget += 1;
                            continue;
                        }

                        let dist = self.distances[self.nodemoving][self.nodetarget];
                        let cost = dist as C * match src.occ {
                            'A' => 1,
                            'B' => 10,
                            'C' => 100,
                            'D' => 1000,
                            _ => panic!(),
                        };

                        self.nodetarget += 1;
                        if self.this.cost + cost >= self.cost_clip {
                            continue;
                        }

                        let mut state = self.this.clone();
                        state.nodes[self.nodemoving] = Node::default();
                        state.nodes[self.nodetarget - 1].occ = src.occ;
                        state.nodes[self.nodetarget - 1].life = match src.life {
                            NodeLife::Start => NodeLife::Hallway,
                            NodeLife::Hallway => NodeLife::Room,
                            _ => panic!(),
                        };
                        state.cost += cost;
                        state.distance = state.distance(self.distances);
                        if state.cost + state.distance >= self.cost_clip {
                            continue;
                        }
                        return Some(state)
                    }
                }
            }
            let masks = unsafe {
                std::mem::transmute::<&[[u32; N]; N],
                                      &'static [[u32; N]; N]>(&masks) };
            let distances = unsafe {
                std::mem::transmute::<&[[D; N]; N],
                          &'static [[D; N]; N]>(&distances) };

            let mut heap = BinaryHeap::new();
            heap.push(start);
            let mut cost = C::MAX;
            while let Some(head) = heap.pop() {
                for state in head.into_iter(cost, masks, distances) {
                    if state.aligned() {
                        if state.cost < cost {
                            cost = state.cost;
                        }
                    } else {
                        heap.push(state);
                    }
                }
            }
            format!("{}", cost)
        } else {
            assert_eq!(input.lines().count(), 5);
            assert_eq!(input.lines().nth(1).unwrap(), "#...........#");
            let row1 = input.lines().nth(2).unwrap()
                .to_string()
                .replace("#", "")
                .replace(" ", "");
            assert_eq!(row1.len(), 4);
            let row2 = "DCBA";
            let row3 = "DBAC";
            let row4 = input.lines().nth(3).unwrap()
                .to_string()
                .replace("#", "")
                .replace(" ", "");
            assert_eq!(row4.len(), 4);

            const N : usize = 7 + 4 * 4; // 7 free hallway points then rooms sequentially
            // #############
            // #01.2.3.4.56#
            // ###7#1#5#9###
            //   #8#2#6#0#
            //   #9#3#7#1#
            //   #0#4#8#2#
            //   #########
            type D = i8;
            const NEIGHBOURS : [u32; N] = [
                1 << 1,                               // 0
                1 << 0  | 1 << 2  | 1 << 7,           // 1
                1 << 1  | 1 << 7  | 1 << 11 | 1 << 3, // 2
                1 << 2  | 1 << 11 | 1 << 15 | 1 << 4, // 3
                1 << 3  | 1 << 15 | 1 << 19 | 1 << 5, // 4
                1 << 4  | 1 << 19 | 1 << 6,           // 5
                1 << 5,                               // 6

                1 << 1  | 1 << 2  | 1 << 8,           // 7
                1 << 7  | 1 << 9,                     // 8
                1 << 8  | 1 << 10,                    // 9
                1 << 9,                               // 10

                1 << 2  | 1 << 3  | 1 << 12,          // 11
                1 << 11 | 1 << 13,                    // 12
                1 << 12 | 1 << 14,                    // 13
                1 << 13,                              // 14

                1 << 3  | 1 << 4  | 1 << 16,          // 15
                1 << 15 | 1 << 17,                    // 16
                1 << 16 | 1 << 18,                    // 17
                1 << 17,                              // 18

                1 << 4  | 1 << 5  | 1 << 20,          // 19
                1 << 19 | 1 << 21,                    // 20
                1 << 20 | 1 << 22,                    // 21
                1 << 21,                              // 21
            ];
            for (idx, v) in NEIGHBOURS.iter().enumerate() {
                assert_eq!((1 << idx) & *v, 0);
                for n in 0..N {
                    if idx != n && 1 << n & *v != 0 {
                        assert!(NEIGHBOURS[n] & 1 << idx != 0);
                    } else {
                        assert!(NEIGHBOURS[n] & 1 << idx == 0);
                    }
                }
            }

            let mut steps = HashSet::new();
            let mut add_double_steps = |a, b, c| {
                steps.insert((a, b));
                steps.insert((a, c));
                steps.insert((b, a));
                steps.insert((b, c));
                steps.insert((c, a));
                steps.insert((c, b));
            };
            add_double_steps(1, 2, 7);
            add_double_steps(2, 3, 11);
            add_double_steps(3, 4, 15);
            add_double_steps(4, 5, 19);
            let steps = steps;

            fn roombase(idx: usize) -> usize { 7 + ((idx - 7) / 4) * 4 }
            assert_eq!(roombase(7), 7);
            assert_eq!(roombase(10), 7);
            assert_eq!(roombase(11), 11);

            // dirty manual dijkstra to build path masks and distances
            let mut distances = [[0 as D; N]; N];
            let mut masks = [[0u32; N]; N];
            for src in 0..N {
                let ndists = &mut distances[src];
                let mut nodes = vec![src];
                let mut nnodes = Vec::new();
                for n in 0..N {
                    ndists[n] = D::MAX;
                }
                ndists[src] = 0;
                // extend
                loop {
                    nnodes.clear();
                    for n1 in &nodes {
                        for n2 in (0..N).filter(|x| 1 << *x & NEIGHBOURS[*n1] != 0) {
                            if ndists[n2] == D::MAX { nnodes.push(n2); }
                            let step = if steps.contains(&(*n1, n2)) { 2 } else { 1 };
                            ndists[n2] = std::cmp::min(ndists[*n1] + step, ndists[n2]);
                        }
                    }
                    if nnodes.len() == 0 { break; }
                    std::mem::swap(&mut nodes, &mut nnodes);
                }
                let nmasks = &mut masks[src];
                // paths back
                loop {
                    if let Some(dst) = (0..N).filter(|x| *x != src)
                                             .filter(|x| nmasks[*x] == 0)
                                             .max_by(|a, b| ndists[*a].cmp(&ndists[*b])) {
                        let mut n = dst;
                        loop {
                            nmasks[dst] |= 1 << n;
                            n = (0..N).filter(|x| 1 << *x & NEIGHBOURS[n] != 0)
                                      .min_by(|a, b| ndists[*a].cmp(&ndists[*b]))
                                      .unwrap();
                            if n == src { break; }
                        }

                    } else { break; }
                }
            }
            let distances = distances;
            let masks = masks;

            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            enum NodeLife { Start, Hallway, Room, }
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            struct Node {
                pub occ: char,
                pub life: NodeLife,
            }
            impl Node {
                pub fn new(occ: char) -> Self {
                    Self {
                        occ,
                        life: NodeLife::Start,
                    }
                }
            }
            impl Default for Node {
                fn default() -> Self {
                    Self::new('.')
                }
            }


            type C = u64;
            #[derive(Copy, Clone, Eq, PartialEq, Debug)]
            pub struct State {
                nodes: [Node; N],
                cost: C,
                depth: usize,
                distance: C, // optimistic measure by design
                #[cfg(feature = "path")]
                tmp: [(char, u8, u8, u16, u16); 100],
            }
            impl State {
                pub fn mask(&self) -> u32 {
                    let mut res = 0u32;
                    for idx in 0..N {
                        if self.nodes[idx].occ != '.' {
                            res |= 1 << idx;
                        }
                    }
                    res
                }
                pub fn distance(&self, distances: &'static [[D; N]; N]) -> C {
                    let mut res = 0;
                    for idx in 0..N {
                        if self.nodes[idx].occ != '.' {
                            let targets = match self.nodes[idx].occ {
                                'A' => &[ 7, 8, 9, 10],
                                'B' => &[11, 12, 13, 14],
                                'C' => &[15, 16, 17, 18],
                                'D' => &[19, 20, 21, 22],
                                _ => panic!(),
                            };
                            let cost = match self.nodes[idx].occ {
                                'A' => 1,
                                'B' => 10,
                                'C' => 100,
                                'D' => 1000,
                                _ => panic!(),
                            };
                            let mind = (0..N).filter(|x| targets.contains(x))
                                             .map(|x| distances[idx][x])
                                             .min()
                                             .unwrap();
                            res += cost * mind as C;
                        }
                    }
                    res
                }
                pub fn aligned(&self) -> bool {
                    for (idx, chr) in [(7, 'A'), (11, 'B'), (15, 'C'), (19, 'D')] {
                        if self.nodes[idx + 0].occ != chr
                        || self.nodes[idx + 1].occ != chr
                        || self.nodes[idx + 2].occ != chr
                        || self.nodes[idx + 3].occ != chr
                        {
                            return false;
                        }
                    }
                    return true;
                }
            }
            impl Default for State {
                fn default() -> Self {
                    Self {
                        nodes: [Node::default(); N],
                        cost: 0,
                        depth: 0,
                        distance: C::MAX,
                        #[cfg(feature = "path")]
                        tmp: [(' ', 0, 0, 0, 0); 100],
                    }
                }
            }
            impl Hash for State {
                fn hash<H: Hasher>(&self, state: &mut H) {
                    self.nodes.hash(state);
                }
            }

            impl fmt::Display for State {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "#############").unwrap();
                    if self.cost != C::MAX {
                        write!(f, " ({:9}) ({})", self.cost, self.depth).unwrap();
                    }
                    #[cfg(feature = "path")]
                    for (idx, (c, src, dst, dist, cost)) in self.tmp.iter().enumerate() {
                        if *c != ' ' {
                            write!(f, " {}:{}{}→{}({}:{})", idx, *c, src, dst, dist, cost).unwrap();
                        }
                    }

                    writeln!(f, "\n#{}{}.{}.{}.{}.{}{}#",
                        self.nodes[0].occ,
                        self.nodes[1].occ,
                        self.nodes[2].occ,
                        self.nodes[3].occ,
                        self.nodes[4].occ,
                        self.nodes[5].occ,
                        self.nodes[6].occ).unwrap();
                    writeln!(f, "###{}#{}#{}#{}###",
                        self.nodes[7 + 0 * 4].occ,
                        self.nodes[7 + 1 * 4].occ,
                        self.nodes[7 + 2 * 4].occ,
                        self.nodes[7 + 3 * 4].occ,
                        ).unwrap();
                    writeln!(f, "###{}#{}#{}#{}###",
                        self.nodes[7 + 0 * 4 + 1].occ,
                        self.nodes[7 + 1 * 4 + 1].occ,
                        self.nodes[7 + 2 * 4 + 1].occ,
                        self.nodes[7 + 3 * 4 + 1].occ,
                        ).unwrap();
                    writeln!(f, "###{}#{}#{}#{}###",
                        self.nodes[7 + 0 * 4 + 2].occ,
                        self.nodes[7 + 1 * 4 + 2].occ,
                        self.nodes[7 + 2 * 4 + 2].occ,
                        self.nodes[7 + 3 * 4 + 2].occ,
                        ).unwrap();
                    writeln!(f, "###{}#{}#{}#{}###",
                        self.nodes[7 + 0 * 4 + 3].occ,
                        self.nodes[7 + 1 * 4 + 3].occ,
                        self.nodes[7 + 2 * 4 + 3].occ,
                        self.nodes[7 + 3 * 4 + 3].occ,
                        )
                }
            }
            impl Ord for State {
                fn cmp(&self, other: &State) -> cmp::Ordering {
                    // finished first, then optimistic distance (A* like)
                    match (self.aligned(), other.aligned()) {
                        (true, true) => other.cost.cmp(&self.cost),
                        (true, false) => cmp::Ordering::Greater,
                        (false, true) => cmp::Ordering::Less,
                        (false, false) => other.distance.cmp(&self.distance)
                            // .then_with(|| other.cost.cmp(&self.cost))
                    }
                }
            }
            impl PartialOrd for State {
                fn partial_cmp(&self, other: &State) -> Option<cmp::Ordering> {
                    Some(self.cmp(other))
                }
            }

            let mut start = State::default();
            let mut row1 = row1.chars();
            let mut row2 = row2.chars();
            let mut row3 = row3.chars();
            let mut row4 = row4.chars();
            for idx in 7..N {
                let occ = match (idx - 7) % 4 {
                    0 => row1.next(),
                    1 => row2.next(),
                    2 => row3.next(),
                    3 => row4.next(),
                    _ => panic!(),
                };
                start.nodes[idx] = Node::new(occ.unwrap());
            }
            let start = start;

            pub struct IntoIteratorState {
                this: State,
                mask: u32,
                nodemoving: usize,
                nodetarget: usize,
                cost_clip: C,
                masks: &'static [[u32; N]; N], // need GAT
                distances: &'static [[D; N]; N],
            }
            impl State {
                fn into_iter(self,
                             cost_clip: C,
                             masks: &'static[[u32; N]; N],
                             distances: &'static [[D; N]; N]) -> IntoIteratorState {
                    IntoIteratorState {
                        this: self,
                        mask: self.mask(),
                        nodemoving: 0,
                        nodetarget: 0,
                        cost_clip,
                        masks,
                        distances,
                    }
                }
            }
            impl Iterator for IntoIteratorState {
                type Item = State;

                fn next(&mut self) -> Option<Self::Item> {
                    loop {
                        let (nodemoving, nodetarget) = (self.nodemoving, self.nodetarget);
                        if nodemoving == N {
                            return None;
                        }
                        let src = &self.this.nodes[nodemoving];
                        if src.occ == '.'
                        || src.life == NodeLife::Room
                        || nodetarget == N
                        {
                            self.nodemoving += 1;
                            self.nodetarget = 0;
                            continue;
                        }

                        if nodetarget == nodemoving
                        || self.masks[nodemoving][nodetarget] & self.mask != 0
                        || (src.life == NodeLife::Start && nodetarget >= 7)
                        || (src.life == NodeLife::Hallway && nodetarget < 7)
                        {
                            self.nodetarget += 1;
                            continue;
                        }
                        if src.life == NodeLife::Hallway {
                            let room_dst = match nodetarget {
                                 7 |  8 |  9 | 10 => 'A',
                                11 | 12 | 13 | 14 => 'B',
                                15 | 16 | 17 | 18 => 'C',
                                19 | 20 | 21 | 22 => 'D',
                                _ => panic!(), };
                            let base = roombase(nodetarget);
                            if src.occ != room_dst
                                // always end of the room
                            || (nodetarget == base + 2
                                && self.this.nodes[base + 3].occ != room_dst)
                            || (nodetarget == base + 1
                                && self.this.nodes[base + 2].occ != room_dst
                                && self.this.nodes[base + 3].occ != room_dst)
                            || (nodetarget == base + 0
                                && self.this.nodes[base + 1].occ != room_dst
                                && self.this.nodes[base + 2].occ != room_dst
                                && self.this.nodes[base + 3].occ != room_dst)
                            {
                                self.nodetarget += 1;
                                continue;
                            }
                        }

                        let dist = self.distances[nodemoving][nodetarget];
                        let cost = dist as C * match src.occ {
                            'A' => 1,
                            'B' => 10,
                            'C' => 100,
                            'D' => 1000,
                            _ => panic!(),
                        };

                        self.nodetarget += 1;
                        if self.this.cost + cost >= self.cost_clip {
                            continue;
                        }

                        let mut state = self.this.clone();
                        state.nodes[nodemoving] = Node::default();
                        state.nodes[nodetarget].occ = src.occ;
                        state.nodes[nodetarget].life = match src.life {
                            NodeLife::Start => NodeLife::Hallway,
                            NodeLife::Hallway => NodeLife::Room,
                            _ => panic!(),
                        };
                        state.cost += cost;
                        state.depth += 1;
                        state.distance = state.distance(self.distances);
                        if state.cost + state.distance >= self.cost_clip {
                            continue;
                        }
                        #[cfg(feature = "path")]
                        if let Some(idx) = state.tmp.iter().enumerate()
                            .find_map(|(idx, v)| if v.0 == ' ' { Some(idx) }  else { None })
                        {
                            state.tmp[idx] = (src.occ, nodemoving as u8, nodetarget as u8,
                                dist as u16, cost as u16);
                        }
                        return Some(state)
                    }
                }
            }
            let masks = unsafe {
                std::mem::transmute::<&[[u32; N]; N],
                                      &'static [[u32; N]; N]>(&masks) };
            let distances = unsafe {
                std::mem::transmute::<&[[D; N]; N],
                          &'static [[D; N]; N]>(&distances) };

            let mut heap = BinaryHeap::new();
            heap.push(start);
            const MAX_DEPTH : usize = 50;
            let mut seens = {
                let mut seens: [MaybeUninit<HashSet::<State>>; MAX_DEPTH] = unsafe {
                    MaybeUninit::uninit().assume_init()
                };
                for elem in &mut seens[..] {
                    elem.write(HashSet::new());
                }
                unsafe { std::mem::transmute::<_, [HashSet::<State>; MAX_DEPTH]>(seens) }
            };
            // seen.insert(&start);
            let mut cost = C::MAX;
            while let Some(head) = heap.pop() {
                for state in head.into_iter(cost, masks, distances) {
                    if state.aligned() {
                        cost = cmp::min(state.cost, cost);
                    } else
                    if !seens[state.depth - 1].contains(&state) {
                        heap.push(state);
                        seens[state.depth - 1].insert(state);
                    }
                }
            }
            format!("{}", cost)
        },
        24 => {
            pub type T = i64;
            let _apply = |data: T| -> bool {
                let data = data.to_string();
                if data.contains('0') {
                    return false;
                }
                let mut data = data.chars()
                                   .map(|d| d.to_digit(10).unwrap() as T);
                // let mut data = data.iter();
                let mut reg = [0 as T; 4]; // w x y z
                fn of(c: char) -> usize { match c {
                    'w' => 0,
                    'x' => 1,
                    'y' => 2,
                    'z' => 3,
                    _ => panic!(),
                }}
                fn val(s: &str, reg: &[T; 4]) -> T {
                    if ["w", "x", "y", "z"].contains(&s) {
                        reg[of(s.chars().next().unwrap())]
                    } else {
                        s.parse::<T>().unwrap()
                    }
                }
                for line in input.lines() {
                    let op1 = line.split_whitespace().nth(1).unwrap();
                    let op1 = op1.chars().next().unwrap();
                    if line.starts_with("inp") {
                        reg[of(op1)] = data.next().unwrap();
                        continue;
                    }
                    let v1 = reg[of(op1)];
                    let op2 = line.split_whitespace().nth(2).unwrap();
                    let v2 = val(op2, &reg);
                    if line.starts_with("add") {
                        reg[of(op1)] = v1 + v2;
                    } else
                    if line.starts_with("mul") {
                        reg[of(op1)] = v1 * v2;
                    } else
                    if line.starts_with("div") {
                        if v2 == 0 { return false; }
                        reg[of(op1)] = v1 / v2;
                    } else
                    if line.starts_with("mod") {
                        if v1 < 0 || v2 <= 0 { return false; }
                        reg[of(op1)] = v1 % v2;
                    } else
                    if line.starts_with("eql") {
                        reg[of(op1)] = if v1 == v2 { 1 } else { 0 };
                    }
                }
                reg[of('z')] == 0
            };
            // for i in {1..14} ; do sed -n "$((i * 18 - 17)),$((i * 18))p" data/24.input >> data/24.input.$i ; done
            // for i in {1..13} ; do echo $i ; diff -d -C 0 data/24.input.$i data/24.input.$((i + 1)) ; done

            let constants = input.lines()
                .collect::<Vec<_>>() // sad
                .chunks(18)
                .map(|lines| {
                    assert_eq!(lines[0], "inp w");
                    let c1 = lines[4][6..].parse::<T>().unwrap();
                    let c2 = lines[5][6..].parse::<T>().unwrap();
                    let c3 = lines[15][6..].parse::<T>().unwrap();
                    (c1, c2, c3) })
                .collect::<Vec<_>>();
            assert_eq!(constants.len(), 14);
            assert_eq!(constants[0].0, 1);
            assert_eq!(constants[0].1, 12);
            assert_eq!(constants[0].2, 9);
            assert_eq!(constants[3].0, 26);
            assert_eq!(constants[3].1, -9);
            assert_eq!(constants[3].2, 5);
            // dfs without double count (no recursive closure as usual...)
            // only z and w values controls a block resulting z, so trivial state
            fn scan(depth: usize,
                    z: T,
                    rev: bool,
                    constants: &Vec<(T, T, T)>,
                    seen: &mut HashSet<(usize, T)>)
                -> Option<T>
            {
                if depth == constants.len() {
                    if z == 0 { Some(0) } else { None }
                } else
                if seen.contains(&(depth, z)) {
                    None
                } else {
                    // let digits : &mut dyn Iterator<Item = T> = if rev { &mut((0..=9).rev()) } else { &mut(0..=9) };
                    let (start, incr, end) = if rev { (9, -1, 0) } else { (1, 1, 10) };
                    let mut w = start;
                    loop {
                        let (c1, c2, c3) = constants[depth];

                        // dump translation + checks
                        let x = 0;
                        let x = x + z;
                        if x < 0 || 26 == 0 { continue }
                        let x = x % 26;
                        if c1 == 0 { continue; }
                        let z = z / c1;
                        let x = x + c2;
                        let x = (x == w) as T;
                        let x = (x == 0) as T;
                        let y = 0;
                        let y = y + 25;
                        let y = y * x;
                        let y = y + 1;
                        let z = z * y;
                        let y = 0;
                        let y = y + w;
                        let y = y + c3;
                        let y = y * x;
                        let z = z + y;

                        if let Some(v) = scan(depth + 1, z, rev, constants, seen) {
                            let cur = constants.len() - depth - 1;
                            let cur = (10 as T).pow(cur as u32);
                            return Some(cur * w + v);
                        }

                        w += incr;
                        if w == end { break; }
                    }
                    seen.insert((depth, z));
                    None
                }
            }

            let mut seen = HashSet::new();
            if part == 1 {
                if let Some(res) = scan(0, 0, true, &constants, &mut seen) {
                    // println!("apply({})={}", res, _apply(res));
                    format!("{}", res)
                } else {
                    panic!("fail")
                }
            } else {
                if let Some(res) = scan(0, 0, false, &constants, &mut seen) {
                    // println!("apply({})={}", res, _apply(res));
                    format!("{}", res)
                } else {
                    panic!("fail")
                }
            }
        }
        25 => {
            let w = input.lines().next().unwrap().chars().count();
            let h = input.lines().count();

            #[repr(u8)]
            #[derive(PartialEq, Eq, Copy, Clone, Debug)]
            enum SeaCucumber {
                None = 0,
                East,
                South,
            }

            let mut grid = Vec::new();
            grid.resize(w * h, SeaCucumber::None);
            let idx_of = |x, y| (x % w) + w * (y % h);

            let mut idx = 0;
            for line in input.lines() {
                for c in line.chars() {
                    grid[idx] = match c {
                        '.' => SeaCucumber::None,
                        '>' => SeaCucumber::East,
                        'v' => SeaCucumber::South,
                        _ => panic!(""),
                    };
                    idx += 1;
                }
            }
            assert_eq!(idx, w * h);

            let mut ngrid = Vec::new();

            let mut step = 0;
            let [neast, nsouth] = [SeaCucumber::East, SeaCucumber::South]
                .map(|t| grid.iter().filter(|x| **x == t).count());

            loop {
                ngrid.clear();
                ngrid.resize(w * h, SeaCucumber::None);

                assert_eq!(neast, grid.iter().filter(|x| **x == SeaCucumber::East).count());
                assert_eq!(nsouth, grid.iter().filter(|x| **x == SeaCucumber::South).count());

                let mut moves = 0;
                for row in 0..h {
                    for col in 0..w {
                        if grid[idx_of(col, row)] == SeaCucumber::East {
                            if grid[idx_of(col + 1, row)] == SeaCucumber::None {
                                ngrid[idx_of(col + 1, row)] = SeaCucumber::East;
                                moves += 1;
                            } else {
                                ngrid[idx_of(col, row)] = SeaCucumber::East;
                            }
                        }
                    }
                }
                for row in 0..h {
                    for col in 0..w {
                        if grid[idx_of(col, row)] == SeaCucumber::South {
                            if grid[idx_of(col, row + 1)] != SeaCucumber::South
                                && ngrid[idx_of(col, row + 1)] == SeaCucumber::None {
                                ngrid[idx_of(col, row + 1)] = SeaCucumber::South;
                                moves += 1;
                            } else {
                                ngrid[idx_of(col, row)] = SeaCucumber::South;
                            }
                        }
                    }
                }
                step += 1;
                // println!("step:{} moves:{}", step, moves);

                if moves == 0 { break; }
                std::mem::swap(&mut ngrid, &mut grid);
            }

            format!("{}", step)
        }
        _ => String::from(""),
    }
}

fn main() {
    let options = Options::parse();

    if options.bench {
        for day in (1..=25).filter(|d| ![17, 22].contains(d)) {
            let data = get_data(day, &options.session);
            for part in [/*1,*/ 2] {
                let start = Instant::now();
                let _res = solve(day, part, &data);
                let elapsed = start.elapsed();
                println!("day {:2} part {} elapsed: {:10}µs", day, part, elapsed.as_micros());
            }
        }
    } else {
        let data = get_data(options.day, &options.session);
        let res = solve(options.day, options.part, &data);
        println!("day {} part {} solve: {}", options.day, options.part, res)
    }
}
