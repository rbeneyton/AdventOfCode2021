use itertools::Itertools;
use std::collections::HashSet;

pub fn solve(part: u8, input: &String) -> String {
    let mut xs = HashSet::new();
    let mut ys = HashSet::new();
    let mut zs = HashSet::new();
    let mut steps = Vec::new();
    for line in input.lines() {
        assert!(line.starts_with("on") || line.starts_with("of"));
        let on = if line.starts_with("on") { 1u8 } else { 0u8 };
        let line = line.replace("on ", "").replace("off ", "");
        let (mut x1, mut x2) = (0, 0);
        let (mut y1, mut y2) = (0, 0);
        let (mut z1, mut z2) = (0, 0);
        for tok in line.split(",") {
            if tok.starts_with("x=") {
                let (a, b) = tok
                    .replace("x=", "")
                    .split("..")
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect_tuple()
                    .unwrap();
                x1 = a;
                x2 = b;
            }
            if tok.starts_with("y=") {
                let (a, b) = tok
                    .replace("y=", "")
                    .split("..")
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect_tuple()
                    .unwrap();
                y1 = a;
                y2 = b;
            }
            if tok.starts_with("z=") {
                let (a, b) = tok
                    .replace("z=", "")
                    .split("..")
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect_tuple()
                    .unwrap();
                z1 = a;
                z2 = b;
            }
        }
        if part == 2
            || (x1.abs() <= 50
                && x2.abs() <= 50
                && y1.abs() <= 50
                && y2.abs() <= 50
                && z1.abs() <= 50
                && z2.abs() <= 50)
        {
            steps.push((on, x1, x2, y1, y2, z1, z2));
            xs.insert(x1);
            xs.insert(x2 + 1);
            ys.insert(y1);
            ys.insert(y2 + 1);
            zs.insert(z1);
            zs.insert(z2 + 1);
        }
    }
    let mut xs = xs.iter().cloned().sorted().collect::<Vec<_>>();
    let mut ys = ys.iter().cloned().sorted().collect::<Vec<_>>();
    let mut zs = zs.iter().cloned().sorted().collect::<Vec<_>>();
    xs.push(xs.last().unwrap() + 1);
    ys.push(ys.last().unwrap() + 1);
    zs.push(zs.last().unwrap() + 1);
    let xs = xs;
    let ys = ys;
    let zs = zs;

    let idx_x = |x| {
        xs.iter()
            .enumerate()
            .find_map(|(i, v)| if *v == x { Some(i) } else { None })
            .unwrap()
    };
    let idx_y = |y| {
        ys.iter()
            .enumerate()
            .find_map(|(i, v)| if *v == y { Some(i) } else { None })
            .unwrap()
    };
    let idx_z = |z| {
        zs.iter()
            .enumerate()
            .find_map(|(i, v)| if *v == z { Some(i) } else { None })
            .unwrap()
    };

    let nx = xs.len();
    let ny = ys.len();
    let nz = zs.len();
    let sz = nx * ny * nz;
    let mut grid = Vec::new();
    grid.resize(sz, 0u8);
    let idx_of = |x, y, z| x + nx * y + (nx * ny) * z;

    for step in steps {
        let (on, x1, x2, y1, y2, z1, z2) = step;

        for x in idx_x(x1)..idx_x(x2 + 1) {
            for y in idx_y(y1)..idx_y(y2 + 1) {
                for z in idx_z(z1)..idx_z(z2 + 1) {
                    grid[idx_of(x, y, z)] = on;
                }
            }
        }
    }

    let mut sum = 0;
    for idx in 0..(xs.len() - 2) {
        for idy in 0..(ys.len() - 2) {
            for idz in 0..(zs.len() - 2) {
                if grid[idx_of(idx, idy, idz)] == 1 {
                    sum +=
                        (xs[idx + 1] - xs[idx]) * (ys[idy + 1] - ys[idy]) * (zs[idz + 1] - zs[idz]);
                }
            }
        }
    }
    format!("{}", sum)
}
