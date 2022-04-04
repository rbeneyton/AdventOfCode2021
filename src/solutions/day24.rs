use std::collections::HashSet;

pub fn solve(part: u8, input: &String) -> String {
    pub type T = i64;
    let _apply = |data: T| -> bool {
        let data = data.to_string();
        if data.contains('0') {
            return false;
        }
        let mut data = data.chars().map(|d| d.to_digit(10).unwrap() as T);
        // let mut data = data.iter();
        let mut reg = [0 as T; 4]; // w x y z
        fn of(c: char) -> usize {
            match c {
                'w' => 0,
                'x' => 1,
                'y' => 2,
                'z' => 3,
                _ => panic!(),
            }
        }
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
            } else if line.starts_with("mul") {
                reg[of(op1)] = v1 * v2;
            } else if line.starts_with("div") {
                if v2 == 0 {
                    return false;
                }
                reg[of(op1)] = v1 / v2;
            } else if line.starts_with("mod") {
                if v1 < 0 || v2 <= 0 {
                    return false;
                }
                reg[of(op1)] = v1 % v2;
            } else if line.starts_with("eql") {
                reg[of(op1)] = if v1 == v2 { 1 } else { 0 };
            }
        }
        reg[of('z')] == 0
    };
    // for i in {1..14} ; do sed -n "$((i * 18 - 17)),$((i * 18))p" data/24.input >> data/24.input.$i ; done
    // for i in {1..13} ; do echo $i ; diff -d -C 0 data/24.input.$i data/24.input.$((i + 1)) ; done

    let constants = input
        .lines()
        .collect::<Vec<_>>() // sad
        .chunks(18)
        .map(|lines| {
            assert_eq!(lines[0], "inp w");
            let c1 = lines[4][6..].parse::<T>().unwrap();
            let c2 = lines[5][6..].parse::<T>().unwrap();
            let c3 = lines[15][6..].parse::<T>().unwrap();
            (c1, c2, c3)
        })
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
    fn scan(
        depth: usize,
        z: T,
        rev: bool,
        constants: &Vec<(T, T, T)>,
        seen: &mut HashSet<(usize, T)>,
    ) -> Option<T> {
        if depth == constants.len() {
            if z == 0 {
                Some(0)
            } else {
                None
            }
        } else if seen.contains(&(depth, z)) {
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
                if x < 0 || 26 == 0 {
                    continue;
                }
                let x = x % 26;
                if c1 == 0 {
                    continue;
                }
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
                if w == end {
                    break;
                }
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
