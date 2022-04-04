use std::cmp;
use std::collections::HashMap;

pub fn solve(part: u8, input: &String) -> String {
    assert_eq!(input.lines().count(), 2);
    assert!(input
        .lines()
        .next()
        .unwrap()
        .starts_with("Player 1 starting position: "));
    assert!(input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .starts_with("Player 2 starting position: "));
    let mut pos1 = input
        .lines()
        .next()
        .unwrap()
        .replace("Player 1 starting position: ", "")
        .parse::<u64>()
        .unwrap();
    let mut pos2 = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
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
            if score1 >= 1000 {
                break;
            }

            pos2 += dice.next().unwrap();
            n += 3;
            pos2 = 1 + (pos2 - 1) % 10; // [1-10] loop
            score2 += pos2;
            if score2 >= 1000 {
                break;
            }
        }
        // dbg!(score1, score2, n);

        format!("{}", cmp::min(score1, score2) * n)
    } else {
        fn round10(v: u64) -> u64 {
            1 + (v - 1) % 10
        }

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
                                                *scorenext
                                                    .entry(((pos1, score1), (pos2, score2)))
                                                    .or_default() += *n;
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
