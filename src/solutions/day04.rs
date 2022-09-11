pub fn solve(part: u8, input: &String) -> String {
    if part == 1 {
        let mut line = input.lines();
        let numbers: Vec<_> = line
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<u8>().unwrap())
            .collect();

        const N: usize = 5;
        type Board = [[(u8, bool); N]; N];
        let mut boards = Vec::new();
        loop {
            match line.next() {
                None => break,
                Some(v) => {
                    if v.chars().count() > 0 {
                        panic!("invalid parse pattern");
                    }
                }
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
                            && board[4][scan].1)
                    {
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
        let numbers: Vec<_> = line
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<u8>().unwrap())
            .collect();

        const N: usize = 5;
        type Board = [[(u8, bool); N]; N];
        let mut boards = Vec::new();
        loop {
            match line.next() {
                None => break,
                Some(v) => {
                    if v.chars().count() > 0 {
                        panic!("invalid parse pattern");
                    }
                }
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
                if board.1 {
                    continue;
                } // win
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
                            && board.0[4][scan].1)
                    {
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
    }
}
