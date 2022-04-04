use std::cmp;
use std::collections::{BinaryHeap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;

pub fn solve(part: u8, input: &String) -> String {
    if part == 1 {
        assert_eq!(input.lines().count(), 5);
        assert_eq!(input.lines().nth(1).unwrap(), "#...........#");
        let row1 = input
            .lines()
            .nth(2)
            .unwrap()
            .to_string()
            .replace("#", "")
            .replace(" ", "");
        assert_eq!(row1.len(), 4);
        let row2 = input
            .lines()
            .nth(3)
            .unwrap()
            .to_string()
            .replace("#", "")
            .replace(" ", "");
        assert_eq!(row2.len(), 4);

        const N: usize = 7 + 4 * 2; // 7 free hallway points then rooms sequentially
                                    // #############
                                    // #01.2.3.4.56#
                                    // ###7#9# # ###
                                    //   #8# # # #
                                    //   #########
        type D = i8;
        let mut distances: [[D; N]; N] = [
            // 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14
            [0, 1, 3, 5, 7, 9, 10, 3, 4, 5, 6, 7, 8, 9, 10], // 0
            [0, 0, 2, 4, 6, 8, 9, 2, 3, 4, 5, 6, 7, 8, 9],   // 1
            [0, 0, 0, 2, 4, 6, 7, 2, 3, 2, 3, 4, 5, 6, 7],   // 2
            [0, 0, 0, 0, 2, 4, 5, 4, 5, 2, 3, 2, 3, 4, 5],   // 3
            [0, 0, 0, 0, 0, 2, 3, 6, 7, 4, 5, 2, 3, 2, 3],   // 4
            [0, 0, 0, 0, 0, 0, 1, 8, 9, 6, 7, 4, 5, 2, 3],   // 5
            [0, 0, 0, 0, 0, 0, 0, 9, 10, 7, 8, 5, 6, 3, 4],  // 6
            [0, 0, 0, 0, 0, 0, 0, 0, 1, 4, 5, 6, 7, 8, 9],   // 7
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 9, 10],  // 8
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 4, 5, 6, 7],   // 9
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8],   // 10
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 4, 5],   // 11
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6],   // 12
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],   // 13
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 14
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
        const NEIGHBOURS: [u32; N] = [
            1 << 1,                              // 0
            1 << 0 | 1 << 2 | 1 << 7,            // 1
            1 << 1 | 1 << 7 | 1 << 9 | 1 << 3,   // 2
            1 << 2 | 1 << 9 | 1 << 11 | 1 << 4,  // 3
            1 << 3 | 1 << 11 | 1 << 13 | 1 << 5, // 4
            1 << 4 | 1 << 13 | 1 << 6,           // 5
            1 << 5,                              // 6
            1 << 1 | 1 << 2 | 1 << 8,            // 7
            1 << 7,                              // 8
            1 << 2 | 1 << 3 | 1 << 10,           // 9
            1 << 9,                              // 10
            1 << 3 | 1 << 4 | 1 << 12,           // 11
            1 << 11,                             // 12
            1 << 4 | 1 << 5 | 1 << 14,           // 13
            1 << 13,                             // 14
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
                        if n != *node && dists[n] == u32::MAX {
                            nnodes.push(n);
                        }
                        dists[n] = std::cmp::min(dists[*node] + 1, dists[n]);
                    }
                }
                if nnodes.len() == 0 {
                    break;
                }
                std::mem::swap(&mut nodes, &mut nnodes);
            }
            let nmasks = &mut masks[src];
            // paths back
            loop {
                if let Some(dst) = (0..N)
                    .filter(|x| *x != src)
                    .filter(|x| nmasks[*x] == 0)
                    .max_by(|a, b| dists[*a].cmp(&dists[*b]))
                {
                    let mut n = dst;
                    let mut dist = dists[dst]; // XXX rust bug if let Some((dst, mut dist)) =
                    loop {
                        nmasks[dst] |= 1 << n;
                        dist -= 1;
                        n = (0..N)
                            .filter(|x| 1 << *x & NEIGHBOURS[n] != 0)
                            .filter(|x| dists[*x] == dist)
                            .next()
                            .unwrap();
                        if n == src {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        }
        let masks = masks;

        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        enum NodeLife {
            Start,
            Hallway,
            Room,
        }
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
                            'A' => &[7, 8],
                            'B' => &[9, 10],
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
                        res += cost
                            * cmp::min(distances[idx][targets[0]], distances[idx][targets[1]]) as C;
                    }
                }
                res
            }
            pub fn aligned(&self) -> bool {
                for idx in [7, 9, 11, 13] {
                    if self.nodes[idx].occ == '.'
                        || self.nodes[idx].occ != self.nodes[idx + 1].occ
                        || self.nodes[idx].occ
                            != match idx {
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

                writeln!(
                    f,
                    "\n#{}{}.{}.{}.{}.{}{}#",
                    self.nodes[0].occ,
                    self.nodes[1].occ,
                    self.nodes[2].occ,
                    self.nodes[3].occ,
                    self.nodes[4].occ,
                    self.nodes[5].occ,
                    self.nodes[6].occ
                )
                .unwrap();
                writeln!(
                    f,
                    "###{}#{}#{}#{}###",
                    self.nodes[7].occ, self.nodes[9].occ, self.nodes[11].occ, self.nodes[13].occ
                )
                .unwrap();
                writeln!(
                    f,
                    "  #{}#{}#{}#{}#",
                    self.nodes[8].occ, self.nodes[10].occ, self.nodes[12].occ, self.nodes[14].occ
                )
            }
        }
        impl Ord for State {
            fn cmp(&self, other: &State) -> cmp::Ordering {
                // finished first, then low depths (dijkstra like)
                match (self.aligned(), other.aligned()) {
                    (true, true) => other.cost.cmp(&self.cost),
                    (true, false) => cmp::Ordering::Greater,
                    (false, true) => cmp::Ordering::Less,
                    (false, false) => other
                        .distance
                        .cmp(&self.distance)
                        .then_with(|| other.cost.cmp(&self.cost)),
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
            let occ = if idx % 2 == 1 {
                row1.next()
            } else {
                row2.next()
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
            fn into_iter(
                self,
                cost_clip: C,
                masks: &'static [[u32; N]; N],
                distances: &'static [[D; N]; N],
            ) -> IntoIteratorState {
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
                    if src.occ == '.' || src.life == NodeLife::Room || self.nodetarget == N {
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
                            && src.occ
                                != match self.nodetarget {
                                    7 | 8 => 'A',
                                    9 | 10 => 'B',
                                    11 | 12 => 'C',
                                    13 | 14 => 'D',
                                    _ => panic!(),
                                })
                        || (src.life == NodeLife::Hallway
                            && self.nodetarget % 2 == 1
                            && self.this.nodes[self.nodetarget + 1].occ != src.occ)
                    {
                        self.nodetarget += 1;
                        continue;
                    }

                    let dist = self.distances[self.nodemoving][self.nodetarget];
                    let cost = dist as C
                        * match src.occ {
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
                    return Some(state);
                }
            }
        }
        let masks =
            unsafe { std::mem::transmute::<&[[u32; N]; N], &'static [[u32; N]; N]>(&masks) };
        let distances =
            unsafe { std::mem::transmute::<&[[D; N]; N], &'static [[D; N]; N]>(&distances) };

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
        let row1 = input
            .lines()
            .nth(2)
            .unwrap()
            .to_string()
            .replace("#", "")
            .replace(" ", "");
        assert_eq!(row1.len(), 4);
        let row2 = "DCBA";
        let row3 = "DBAC";
        let row4 = input
            .lines()
            .nth(3)
            .unwrap()
            .to_string()
            .replace("#", "")
            .replace(" ", "");
        assert_eq!(row4.len(), 4);

        const N: usize = 7 + 4 * 4; // 7 free hallway points then rooms sequentially
                                    // #############
                                    // #01.2.3.4.56#
                                    // ###7#1#5#9###
                                    //   #8#2#6#0#
                                    //   #9#3#7#1#
                                    //   #0#4#8#2#
                                    //   #########
        type D = i8;
        const NEIGHBOURS: [u32; N] = [
            1 << 1,                              // 0
            1 << 0 | 1 << 2 | 1 << 7,            // 1
            1 << 1 | 1 << 7 | 1 << 11 | 1 << 3,  // 2
            1 << 2 | 1 << 11 | 1 << 15 | 1 << 4, // 3
            1 << 3 | 1 << 15 | 1 << 19 | 1 << 5, // 4
            1 << 4 | 1 << 19 | 1 << 6,           // 5
            1 << 5,                              // 6
            1 << 1 | 1 << 2 | 1 << 8,            // 7
            1 << 7 | 1 << 9,                     // 8
            1 << 8 | 1 << 10,                    // 9
            1 << 9,                              // 10
            1 << 2 | 1 << 3 | 1 << 12,           // 11
            1 << 11 | 1 << 13,                   // 12
            1 << 12 | 1 << 14,                   // 13
            1 << 13,                             // 14
            1 << 3 | 1 << 4 | 1 << 16,           // 15
            1 << 15 | 1 << 17,                   // 16
            1 << 16 | 1 << 18,                   // 17
            1 << 17,                             // 18
            1 << 4 | 1 << 5 | 1 << 20,           // 19
            1 << 19 | 1 << 21,                   // 20
            1 << 20 | 1 << 22,                   // 21
            1 << 21,                             // 21
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

        fn roombase(idx: usize) -> usize {
            7 + ((idx - 7) / 4) * 4
        }
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
                        if ndists[n2] == D::MAX {
                            nnodes.push(n2);
                        }
                        let step = if steps.contains(&(*n1, n2)) { 2 } else { 1 };
                        ndists[n2] = std::cmp::min(ndists[*n1] + step, ndists[n2]);
                    }
                }
                if nnodes.len() == 0 {
                    break;
                }
                std::mem::swap(&mut nodes, &mut nnodes);
            }
            let nmasks = &mut masks[src];
            // paths back
            loop {
                if let Some(dst) = (0..N)
                    .filter(|x| *x != src)
                    .filter(|x| nmasks[*x] == 0)
                    .max_by(|a, b| ndists[*a].cmp(&ndists[*b]))
                {
                    let mut n = dst;
                    loop {
                        nmasks[dst] |= 1 << n;
                        n = (0..N)
                            .filter(|x| 1 << *x & NEIGHBOURS[n] != 0)
                            .min_by(|a, b| ndists[*a].cmp(&ndists[*b]))
                            .unwrap();
                        if n == src {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        }
        let distances = distances;
        let masks = masks;

        #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
        enum NodeLife {
            Start,
            Hallway,
            Room,
        }
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
                            'A' => &[7, 8, 9, 10],
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
                        let mind = (0..N)
                            .filter(|x| targets.contains(x))
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

                writeln!(
                    f,
                    "\n#{}{}.{}.{}.{}.{}{}#",
                    self.nodes[0].occ,
                    self.nodes[1].occ,
                    self.nodes[2].occ,
                    self.nodes[3].occ,
                    self.nodes[4].occ,
                    self.nodes[5].occ,
                    self.nodes[6].occ
                )
                .unwrap();
                writeln!(
                    f,
                    "###{}#{}#{}#{}###",
                    self.nodes[7 + 0 * 4].occ,
                    self.nodes[7 + 1 * 4].occ,
                    self.nodes[7 + 2 * 4].occ,
                    self.nodes[7 + 3 * 4].occ,
                )
                .unwrap();
                writeln!(
                    f,
                    "###{}#{}#{}#{}###",
                    self.nodes[7 + 0 * 4 + 1].occ,
                    self.nodes[7 + 1 * 4 + 1].occ,
                    self.nodes[7 + 2 * 4 + 1].occ,
                    self.nodes[7 + 3 * 4 + 1].occ,
                )
                .unwrap();
                writeln!(
                    f,
                    "###{}#{}#{}#{}###",
                    self.nodes[7 + 0 * 4 + 2].occ,
                    self.nodes[7 + 1 * 4 + 2].occ,
                    self.nodes[7 + 2 * 4 + 2].occ,
                    self.nodes[7 + 3 * 4 + 2].occ,
                )
                .unwrap();
                writeln!(
                    f,
                    "###{}#{}#{}#{}###",
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
                    (false, false) => other.distance.cmp(&self.distance), // .then_with(|| other.cost.cmp(&self.cost))
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
            fn into_iter(
                self,
                cost_clip: C,
                masks: &'static [[u32; N]; N],
                distances: &'static [[D; N]; N],
            ) -> IntoIteratorState {
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
                    if src.occ == '.' || src.life == NodeLife::Room || nodetarget == N {
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
                            7 | 8 | 9 | 10 => 'A',
                            11 | 12 | 13 | 14 => 'B',
                            15 | 16 | 17 | 18 => 'C',
                            19 | 20 | 21 | 22 => 'D',
                            _ => panic!(),
                        };
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
                    let cost = dist as C
                        * match src.occ {
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
                    if let Some(idx) = state.tmp.iter().enumerate().find_map(|(idx, v)| {
                        if v.0 == ' ' {
                            Some(idx)
                        } else {
                            None
                        }
                    }) {
                        state.tmp[idx] = (
                            src.occ,
                            nodemoving as u8,
                            nodetarget as u8,
                            dist as u16,
                            cost as u16,
                        );
                    }
                    return Some(state);
                }
            }
        }
        let masks =
            unsafe { std::mem::transmute::<&[[u32; N]; N], &'static [[u32; N]; N]>(&masks) };
        let distances =
            unsafe { std::mem::transmute::<&[[D; N]; N], &'static [[D; N]; N]>(&distances) };

        let mut heap = BinaryHeap::new();
        heap.push(start);
        const MAX_DEPTH: usize = 50;
        let mut seens = {
            let mut seens: [MaybeUninit<HashSet<State>>; MAX_DEPTH] =
                unsafe { MaybeUninit::uninit().assume_init() };
            for elem in &mut seens[..] {
                elem.write(HashSet::new());
            }
            unsafe { std::mem::transmute::<_, [HashSet<State>; MAX_DEPTH]>(seens) }
        };
        // seen.insert(&start);
        let mut cost = C::MAX;
        while let Some(head) = heap.pop() {
            for state in head.into_iter(cost, masks, distances) {
                if state.aligned() {
                    cost = cmp::min(state.cost, cost);
                } else if !seens[state.depth - 1].contains(&state) {
                    heap.push(state);
                    seens[state.depth - 1].insert(state);
                }
            }
        }
        format!("{}", cost)
    }
}
