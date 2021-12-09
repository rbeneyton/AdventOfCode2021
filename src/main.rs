use curl::easy::Easy;
use clap::Parser;
use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

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

            println!("sz: {}", sz);

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

            println!("sz: {}", sz);

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
                for num in &wires_in_digit[&5] {
                    println!("num: {}", num);
                }
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
                            print!(" XXX");
                        } else {
                            print!(" {:3}", v);
                        }
                    }
                    println!("");
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
