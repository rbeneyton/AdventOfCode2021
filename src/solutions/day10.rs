pub fn solve(part: u8, input: &String) -> String {
    if part == 1 {
        let mut stack = Vec::new();
        let mut score = 0;
        'line: for line in input.lines() {
            stack.clear();
            'scan: for c in line.chars() {
                for (open, close, point) in [
                    ('(', ')', 3),
                    ('[', ']', 57),
                    ('{', '}', 1197),
                    ('<', '>', 25137),
                ] {
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
}
