use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(part: u8, input: &String) -> String {
    if part == 1 {
        let polymer = input.lines().next().unwrap();
        let templates = input
            .lines()
            .filter(|x| x.contains(" -> "))
            .map(|x| {
                x.split(" -> ")
                    .collect_tuple::<(_, _)>()
                    .map(|(pair, insert)| {
                        (
                            (
                                pair.chars().next().unwrap(),
                                pair.chars().skip(1).next().unwrap(),
                            ),
                            insert.chars().next().unwrap(),
                        )
                    })
                    .unwrap()
            })
            .collect::<HashMap<_, _>>();

        let mut polymer = String::from(polymer);
        for _step in 0..10 {
            let last = polymer.chars().last();
            polymer = polymer
                .chars()
                .tuple_windows()
                .map(|(a, b)| [a, *templates.get(&(a, b)).unwrap()])
                .flatten()
                .chain(last)
                .collect();
        }

        let keys_counts = polymer
            .chars()
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
        let templates = input
            .lines()
            .filter(|x| x.contains(" -> "))
            .map(|x| {
                x.split(" -> ")
                    .collect_tuple::<(_, _)>()
                    .map(|(pair, insert)| {
                        (
                            (
                                pair.chars().next().unwrap(),
                                pair.chars().skip(1).next().unwrap(),
                            ),
                            insert.chars().next().unwrap(),
                        )
                    })
                    .unwrap()
            })
            .collect::<HashMap<_, _>>();

        let mut polymer =
            polymer
                .chars()
                .tuple_windows()
                .fold(HashMap::<_, usize>::new(), |mut h, (a, b)| {
                    *h.entry((a, b)).or_default() += 1; // XXX double count!
                    h
                });
        for _step in 0..40 {
            polymer = polymer
                .iter()
                .map(|((a, b), n)| {
                    let spawn = *templates.get(&(*a, *b)).unwrap();
                    [((*a, spawn), *n), ((spawn, *b), *n)]
                })
                .flatten()
                .fold(HashMap::<_, usize>::new(), |mut h, ((a, b), n)| {
                    *h.entry((a, b)).or_default() += n;
                    h
                });
        }

        let keys_counts = polymer
            .iter()
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
    }
}
