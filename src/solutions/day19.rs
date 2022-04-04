use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;

pub fn solve(part: u8, input: &String) -> String {
    // {{{ Pos
    #[derive(Clone, Copy, Eq, Hash, Debug)]
    struct Pos {
        x: i32,
        y: i32,
        z: i32,
    }
    impl PartialEq for Pos {
        fn eq(&self, other: &Pos) -> bool {
            self.x == other.x && self.y == other.y && self.z == other.z
        }
    }
    impl Pos {
        pub fn new(x: i32, y: i32, z: i32) -> Self {
            Self { x, y, z }
        }
        pub fn from_str(input: &'_ str) -> Self {
            let (x, y, z) = input
                .split(',')
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
            (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
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
    enum Up {
        X,
        Y,
        Z,
        XR,
        YR,
        ZR,
    }
    #[derive(Clone, Copy, Debug, Eq)]
    struct ScannerOriented {
        offset: Pos, // in '0' referential
        dirx: bool,  // means reverted if true
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
            write!(
                f,
                "offset:{} dir:{}/{}/{} up:{:?}",
                self.offset, self.dirx, self.diry, self.dirz, self.up
            )
        }
    }
    impl ScannerOriented {
        pub fn all() -> Vec<ScannerOriented> {
            let mut res = Vec::with_capacity(24);

            for dirx in [false, true] {
                for diry in [false, true] {
                    for dirz in [false, true] {
                        for up in [Up::X, Up::Y, Up::Z, Up::XR, Up::YR, Up::ZR] {
                            res.push(ScannerOriented {
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
                }
                Up::Y => {
                    x = pos_in_0.x + if self.diry { 1 } else { -1 } * pos.y;
                    y = pos_in_0.y + if self.dirz { 1 } else { -1 } * pos.z;
                    z = pos_in_0.z + if self.dirx { 1 } else { -1 } * pos.x;
                }
                Up::X => {
                    x = pos_in_0.x + if self.dirz { 1 } else { -1 } * pos.z;
                    y = pos_in_0.y + if self.dirx { 1 } else { -1 } * pos.x;
                    z = pos_in_0.z + if self.diry { 1 } else { -1 } * pos.y;
                }
                Up::ZR => {
                    x = pos_in_0.x + if self.dirx { 1 } else { -1 } * pos.x;
                    y = pos_in_0.y + if self.diry { 1 } else { -1 } * pos.z;
                    z = pos_in_0.z + if self.dirz { 1 } else { -1 } * pos.y;
                }
                Up::YR => {
                    x = pos_in_0.x + if self.diry { 1 } else { -1 } * pos.y;
                    y = pos_in_0.y + if self.dirz { 1 } else { -1 } * pos.x;
                    z = pos_in_0.z + if self.dirx { 1 } else { -1 } * pos.z;
                }
                Up::XR => {
                    x = pos_in_0.x + if self.dirz { 1 } else { -1 } * pos.z;
                    y = pos_in_0.y + if self.dirx { 1 } else { -1 } * pos.y;
                    z = pos_in_0.z + if self.diry { 1 } else { -1 } * pos.x;
                }
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
                }
                Up::Y => {
                    x = self.offset.x + if self.diry { -1 } else { 1 } * pos.y;
                    y = self.offset.y + if self.dirz { -1 } else { 1 } * pos.z;
                    z = self.offset.z + if self.dirx { -1 } else { 1 } * pos.x;
                }
                Up::X => {
                    x = self.offset.x + if self.dirz { -1 } else { 1 } * pos.z;
                    y = self.offset.y + if self.dirx { -1 } else { 1 } * pos.x;
                    z = self.offset.z + if self.diry { -1 } else { 1 } * pos.y;
                }
                Up::ZR => {
                    x = self.offset.x + if self.dirx { -1 } else { 1 } * pos.x;
                    y = self.offset.y + if self.diry { -1 } else { 1 } * pos.z;
                    z = self.offset.z + if self.dirz { -1 } else { 1 } * pos.y;
                }
                Up::YR => {
                    x = self.offset.x + if self.diry { -1 } else { 1 } * pos.y;
                    y = self.offset.y + if self.dirz { -1 } else { 1 } * pos.x;
                    z = self.offset.z + if self.dirx { -1 } else { 1 } * pos.z;
                }
                Up::XR => {
                    x = self.offset.x + if self.dirz { -1 } else { 1 } * pos.z;
                    y = self.offset.y + if self.dirx { -1 } else { 1 } * pos.y;
                    z = self.offset.z + if self.diry { -1 } else { 1 } * pos.x;
                }
            }
            Pos::new(x, y, z)
        }
    }
    // }}}
    // {{{ Scanner
    #[derive(Clone, Debug)]
    struct Scanner {
        a: Vec<Pos>,
        oriented: Option<ScannerOriented>,
        real: Vec<Option<Pos>>,
    }
    impl fmt::Display for Scanner {
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
        pub fn from_str(input: &'_ str) -> Vec<Self> {
            let mut res = Vec::new();
            let mut a = Vec::new();

            for line in input.lines().filter(|l| l.len() > 0) {
                if line.starts_with("--- scanner ") {
                    if a.len() > 0 {
                        res.push(Scanner::new(a.clone()));
                        a.clear();
                    }
                    let id = line
                        .split_whitespace()
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
        pub fn beacon_distances(&self, from: usize) -> HashMap<i32, usize> {
            let mut res = HashMap::new();
            for to in 0..self.a.len() {
                if from == to {
                    continue;
                }
                let d = self.a[from].distance(&self.a[to]);
                *res.entry(d).or_insert(0) += 1;
            }
            res
        }
        pub fn compute_compatible(
            scanners: &mut Vec<Self>,
            idref: usize,
            idcand: usize,
            uniques: &Vec<HashSet<((usize, usize), Pos)>>,
        ) {
            assert!(scanners[idref].oriented.is_some());
            let all = ScannerOriented::all();
            let mut res = Vec::new();
            for cand in &all {
                // println!("cand {}", cand);
                let mut offsets = HashSet::new();
                for unique in uniques {
                    let refpos = unique
                        .iter()
                        .filter(|(id, _)| id.0 == idref)
                        // .map(|(_, pos)| pos)
                        .map(|(id, _)| scanners[id.0].real[id.1].unwrap())
                        .next();
                    if refpos.is_none() {
                        continue;
                    }
                    let refpos = refpos.unwrap();

                    let candpos = unique
                        .iter()
                        .filter(|(id, _)| id.0 == idcand)
                        .map(|(_, pos)| pos)
                        .next();
                    if candpos.is_none() {
                        continue;
                    }
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
                    scanners[idcand].real[idx] = Some(res[0].add_offset(&scanners[idcand].a[idx]));
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
            internal_distances.insert((id, from), scanner.beacon_distances(from));
        }
    }
    let mut commons = HashMap::<(usize, usize), usize>::new();
    let mut uniques = Vec::<HashSet<((usize, usize), Pos)>>::new();
    let mut marry = |from_id, from, from_pos, to_id, to, to_pos| {
        let from = (from_id, from);
        let to = (to_id, to);
        let id = if let Some(id) = commons.get(&from) {
            *id
        } else if let Some(id) = commons.get(&to) {
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
            let distfromk = distfrom.keys().collect::<HashSet<_>>(); // inefficient
            for id2 in (id + 1)..n {
                for (to, to_pos) in scanners[id2].a.iter().enumerate() {
                    let distto = internal_distances.get(&(id2, to)).unwrap();
                    let disttok = distto.keys().collect::<HashSet<_>>(); // inefficient
                    let common = distfromk
                        .intersection(&disttok)
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
                let c = uniques
                    .iter()
                    .enumerate()
                    .filter_map(|(uid, poss)| {
                        let ref_count = poss.iter().filter(|(id, _)| id.0 == scannerref).count();
                        let cand_count = poss.iter().filter(|(id, _)| id.0 == x).count();
                        assert!(ref_count <= 1 && cand_count <= 1);
                        if ref_count == 1 && cand_count == 1 {
                            Some(uid)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                (x, c.len(), c)
            })
            .max_by(|(_, ca, _), (_, cb, _)| ca.cmp(&cb))
            .unwrap();
        if best_common == 0 {
            scannerref = (0..n)
                .map(|x| (x + 1 + scannerref) % scanners.len())
                .filter(|x| scanners[*x].oriented.is_some())
                .next()
                .unwrap();
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
        let pos = scanners
            .iter()
            .map(|x| x.oriented.unwrap().offset)
            .collect::<Vec<_>>();
        let far = (0..n)
            .map(|x| {
                (0..n)
                    .filter(|y| *y != x)
                    .map(|y| pos[x].manhattan(&pos[y]))
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap();
        format!("{}", far)
    }
}
