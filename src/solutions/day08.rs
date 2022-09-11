use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(part: u8, input: &String) -> String {
    if part == 1 {
        let sum: usize = input
            .lines()
            .map(|line| {
                let (digits, displayed) = line.split('|').collect_tuple().unwrap();
                [1, 4, 7, 8]
                    .iter()
                    .map(|digit| {
                        let n_wires = match digit {
                            1 => 2,
                            4 => 4,
                            7 => 3,
                            8 => 7,
                            _ => panic!(),
                        };
                        let wires = digits
                            .split_whitespace()
                            .find(|x| x.len() == n_wires)
                            .unwrap()
                            .chars()
                            .sorted()
                            .collect::<String>();
                        let found: usize = displayed
                            .split_whitespace()
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
        let sum: usize = input
            .lines()
            .map(|line| {
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
                let a: char = num7.chars().find(|c| !num1.chars().contains(c)).unwrap();
                // rule 2: for 6 wires number (0, 6, 9), only 6 haven't c&f (the ones in 1)
                assert_eq!(
                    wires_in_digit[&6]
                        .iter()
                        .filter(|num| !num1.chars().all(|c| num.chars().contains(&c)))
                        .count(),
                    1
                );
                let num6 = &wires_in_digit[&6]
                    .iter()
                    .find(|num| !num1.chars().all(|c| num.chars().contains(&c)))
                    .unwrap();
                // rule 3: the missing wire in 6 is c
                assert_eq!(
                    num8.chars().filter(|c| !num6.chars().contains(c)).count(),
                    1
                );
                let c: char = num8.chars().find(|c| !num6.chars().contains(c)).unwrap();
                assert!(a != c);
                // rule 4: only c&f in 1
                assert!(num1.chars().contains(&c));
                let f: char = num1.chars().find(|ch| ch != &c).unwrap();
                assert!(f != a);
                assert!(f != c);
                // rule 5: b&d are present in 4, but only b is present in all 6 wires number 0 6 9
                let b: char = num4
                    .chars()
                    .filter(|c| !num1.chars().contains(c))
                    .find(|c| wires_in_digit[&6].iter().all(|num| num.chars().contains(c)))
                    .unwrap();
                assert!(b != a);
                assert!(b != c);
                assert!(b != f);
                // rule 6: d is present in 4
                let d: char = num4
                    .chars()
                    .find(|ch| (ch != &b) && (ch != &c) && (ch != &f))
                    .unwrap();
                assert!(d != a);
                assert!(d != c);
                assert!(d != f);
                assert!(d != b);
                // rule 7: the only 6 wire number ≠ 6 with d is 9
                let num9 = &wires_in_digit[&6]
                    .iter()
                    .filter(|num| num != num6)
                    .find(|num| num.chars().contains(&d))
                    .unwrap();
                // rule 8: the missing wire in 9 is e
                let e: char = num8.chars().find(|c| !num9.chars().contains(c)).unwrap();
                assert!(e != a);
                assert!(e != c);
                assert!(e != f);
                assert!(e != b);
                assert!(e != d);
                // rule 9: g is in 8
                let g: char = num8
                    .chars()
                    .find(|ch| {
                        (ch != &a)
                            && (ch != &b)
                            && (ch != &c)
                            && (ch != &d)
                            && (ch != &e)
                            && (ch != &f)
                    })
                    .unwrap();
                assert!(g != a);
                assert!(g != c);
                assert!(g != f);
                assert!(g != b);
                assert!(g != d);
                assert!(g != e);

                // find numbers
                let num3 = &wires_in_digit[&5]
                    .iter()
                    .find(|num| {
                        num.chars().contains(&a)
                            && num.chars().contains(&c)
                            && num.chars().contains(&d)
                            && num.chars().contains(&f)
                            && num.chars().contains(&g)
                    })
                    .unwrap();
                let num2 = &wires_in_digit[&5]
                    .iter()
                    .find(|num| {
                        num.chars().contains(&a)
                            && num.chars().contains(&c)
                            && num.chars().contains(&d)
                            && num.chars().contains(&e)
                            && num.chars().contains(&g)
                    })
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
                            if v == **num2 {
                                aim += 2;
                            } else if v == **num3 {
                                aim += 3;
                            } else {
                                aim += 5;
                            }
                        }
                        6 => {
                            if v == **num6 {
                                aim += 6;
                            } else if v == **num9 {
                                aim += 9;
                            }
                            // 0
                        }
                        7 => aim += 8,
                        _ => panic!(),
                    }
                }

                aim
            })
            .sum();

        format!("{}", sum)
    }
}
