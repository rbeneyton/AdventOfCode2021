use itertools::Itertools;
use std::iter::Peekable;

pub fn solve(part: u8, input: &String) -> String {
    if part == 1 {
        assert_eq!(input.lines().count(), 1);
        let input: Vec<i64> = input
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(|c| {
                let v = i64::from_str_radix(&c.to_string(), 16).unwrap();
                [v & (1 << 3), v & (1 << 2), v & (1 << 1), v & (1 << 0)]
            })
            .flatten()
            .map(|x| if x > 0 { 1 } else { 0 })
            .collect();
        let mut cursor = input.iter().peekable();

        fn process_packet<'buf, 'it, T>(cursor: &'it mut Peekable<T>) -> i64
        where
            T: Iterator<Item = &'buf i64>,
            T: Itertools,
        {
            let mut res = 0i64;
            // version: 3 bit
            let (v1, v2, v3) = cursor.next_tuple().unwrap();
            let version = (v1 << 2) + (v2 << 1) + v3;
            res += version;
            // type_id: 3 bit
            let (t1, t2, t3) = cursor.next_tuple().unwrap();
            match (t1, t2, t3) {
                // litteral
                (1, 0, 0) => {
                    let mut value = 0i64;
                    loop {
                        // group
                        let group_mark = cursor.next().unwrap();
                        for i in (0..4).rev() {
                            value += cursor.next().unwrap() << i;
                        }
                        if *group_mark == 0 {
                            break;
                        }
                        value <<= 4;
                    }
                    let _litteral = value;
                }
                // operator
                _ => {
                    let lenght_type_id = cursor.next().unwrap();
                    let lenght_type_id_width = if *lenght_type_id == 1 { 11 } else { 15 };
                    let mut value = 0i64;
                    for i in (0..lenght_type_id_width).rev() {
                        value += cursor.next().unwrap() << i;
                    }
                    if *lenght_type_id == 0 {
                        // value = total width of all subpacket
                        let buf_size = value as isize;
                        let start: *const i64 = *cursor.peek().unwrap();

                        loop {
                            res += process_packet(cursor);

                            let pos: *const i64 = *cursor.peek().unwrap();
                            let sz = unsafe { pos.offset_from(start) };
                            assert!(sz <= buf_size);
                            if sz == buf_size {
                                break;
                            }
                        }
                    } else if *lenght_type_id == 1 {
                        // value = number of subpacket
                        let nb_packet = value;
                        for _ in 0..nb_packet {
                            res += process_packet(cursor);
                        }
                    }
                }
            }

            res
        }

        let version_sum = process_packet(&mut cursor);
        format!("{}", version_sum)
    } else {
        assert_eq!(input.lines().count(), 1);
        let input: Vec<i64> = input
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(|c| {
                let v = i64::from_str_radix(&c.to_string(), 16).unwrap();
                [v & (1 << 3), v & (1 << 2), v & (1 << 1), v & (1 << 0)]
            })
            .flatten()
            .map(|x| if x > 0 { 1 } else { 0 })
            .collect();
        let mut cursor = input.iter().peekable();

        fn process_packet<'buf, 'it, T>(cursor: &'it mut Peekable<T>) -> i64
        where
            T: Iterator<Item = &'buf i64>,
            T: Itertools,
        {
            // version: 3 bit
            let (v1, v2, v3) = cursor.next_tuple().unwrap();
            let _version = (v1 << 2) + (v2 << 1) + v3;

            // type_id: 3 bit
            let (t1, t2, t3) = cursor.next_tuple().unwrap();
            let type_id = (t1 << 2) + (t2 << 1) + t3;
            if type_id == 4 {
                // litteral
                let mut litteral = 0i64;
                loop {
                    // group
                    let group_mark = cursor.next().unwrap();
                    for i in (0..4).rev() {
                        litteral += cursor.next().unwrap() << i;
                    }
                    if *group_mark == 0 {
                        break;
                    }
                    litteral <<= 4;
                }
                litteral
            } else {
                // operator
                let lenght_type_id = cursor.next().unwrap();
                let lenght_type_id_width = if *lenght_type_id == 1 { 11 } else { 15 };
                let mut value = 0i64;
                for i in (0..lenght_type_id_width).rev() {
                    value += cursor.next().unwrap() << i;
                }
                let mut values = Vec::new();
                if *lenght_type_id == 0 {
                    // value = total width of all subpacket
                    let buf_size = value as isize;
                    let start: *const i64 = *cursor.peek().unwrap();

                    loop {
                        values.push(process_packet(cursor));

                        let pos: *const i64 = *cursor.peek().unwrap();
                        let sz = unsafe { pos.offset_from(start) };
                        assert!(sz <= buf_size);
                        if sz == buf_size {
                            break;
                        }
                    }
                } else if *lenght_type_id == 1 {
                    // value = number of subpacket
                    let nb_packet = value;
                    for _ in 0..nb_packet {
                        values.push(process_packet(cursor));
                    }
                }
                match type_id {
                    0 => values.iter().sum(),
                    1 => values.iter().product(),
                    2 => *values.iter().min().unwrap(),
                    3 => *values.iter().max().unwrap(),
                    5 => {
                        assert_eq!(values.len(), 2);
                        if values[0] > values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        assert_eq!(values.len(), 2);
                        if values[0] < values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        assert_eq!(values.len(), 2);
                        if values[0] == values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!(""),
                }
            }
        }

        let version_sum = process_packet(&mut cursor);
        format!("{}", version_sum)
    }
}
