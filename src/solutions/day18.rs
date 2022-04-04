use std::cmp;
use std::iter::Sum;
use std::ops::Add;

pub fn solve(part: u8, input: &String) -> String {
    #[derive(Clone, Copy, Eq)]
    struct Tok {
        d: u8,
        v: u32,
    }
    impl PartialEq for Tok {
        fn eq(&self, other: &Tok) -> bool {
            self.d == other.d && self.v == other.v
        }
    }

    #[derive(Eq, Clone)]
    struct Snail {
        a: Vec<Tok>,
    }
    impl Snail {
        pub fn new() -> Self {
            Self { a: Vec::new() }
        }
        pub fn from_str(input: &'_ str) -> Self {
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
                        res.a.push(Tok { d, v });
                    }
                }
            }
            res
        }
        pub fn explode(&mut self) -> bool {
            let n = self.a.len();
            let deep = self
                .a
                .iter()
                .enumerate()
                .find_map(|(idx, t)| if t.d == 5 { Some(idx) } else { None });
            if deep.is_none() {
                return false;
            }
            let idx = deep.unwrap();
            assert_eq!(self.a[idx + 1].d, 5);
            if idx != 0 {
                self.a[idx - 1].v += self.a[idx].v;
            }
            if idx + 1 != n - 1 {
                self.a[idx + 2].v += self.a[idx + 1].v;
            }
            self.a.remove(idx);
            self.a[idx] = Tok { d: 4, v: 0 };
            true
        }
        pub fn split(&mut self) -> bool {
            let big = self
                .a
                .iter()
                .enumerate()
                .find_map(|(idx, t)| if t.v >= 10 { Some(idx) } else { None });
            if big.is_none() {
                return false;
            }
            let idx = big.unwrap();
            let d = self.a[idx].d + 1;
            let l = self.a[idx].v / 2;
            let r = self.a[idx].v - l;
            self.a[idx] = Tok { d, v: l };
            self.a.insert(idx + 1, Tok { d, v: r });
            true
        }
        pub fn reduce(&mut self) {
            loop {
                if self.explode() {
                    continue;
                }
                if self.split() {
                    continue;
                }
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
                            a[idx] = Tok {
                                d: d - 1,
                                v: 3 * a[idx].v + 2 * a[idx + 1].v,
                            };
                            a.remove(idx + 1);
                            modified = true;
                            break;
                        }
                    }
                    if !modified {
                        break;
                    }
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
            if self.a.len() == 0 {
                return other;
            }

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
        where
            I: Iterator<Item = Self>,
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
    check_explode("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
    check_explode("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
    check_explode("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
    check_explode(
        "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
    );
    check_explode(
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
    );

    fn check_split(before: &str, after: &str) {
        let mut a = Snail::from_str(before);
        a.split();
        let b = Snail::from_str(after);
        assert!(a == b);
    }

    check_split(
        "[[[[0,7],4],[F,[0,D]]],[1,1]]",
        "[[[[0,7],4],[[7,8],[0,D]]],[1,1]]",
    );
    check_split(
        "[[[[0,7],4],[[7,8],[0,D]]],[1,1]]",
        "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
    );

    assert!(
        Snail::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]") + Snail::from_str("[1,1]")
            == Snail::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
    );

    fn check_sum(a: &[&str], sum: &str) {
        let a: Snail = a.iter().map(|x| Snail::from_str(x)).sum();
        let b = Snail::from_str(sum);
        assert!(a == b);
    }

    check_sum(
        &["[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]"],
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
    );
    check_sum(
        &["[1,1]", "[2,2]", "[3,3]", "[4,4]"],
        "[[[[1,1],[2,2]],[3,3]],[4,4]]",
    );
    check_sum(
        &["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"],
        "[[[[3,0],[5,3]],[4,4]],[5,5]]",
    );
    check_sum(
        &["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"],
        "[[[[5,0],[7,4]],[5,5]],[6,6]]",
    );
    check_sum(
        &[
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ],
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
    );

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
    check_magnitude(
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        3488,
    );

    if part == 1 {
        let magnitude = input
            .lines()
            .map(|l| Snail::from_str(l))
            .sum::<Snail>()
            .magnitude();

        format!("{}", magnitude)
    } else {
        let snails = input
            .lines()
            .map(|l| Snail::from_str(l))
            .collect::<Vec<Snail>>();

        let mut magnitude = 0;
        for a in 0..snails.len() {
            for b in (a + 1)..snails.len() {
                magnitude = cmp::max(snails[a].add_assign_magnitude(&snails[b]), magnitude);
                magnitude = cmp::max(snails[b].add_assign_magnitude(&snails[a]), magnitude);
            }
        }

        format!("{}", magnitude)
    }
}
