use std::fs;

pub fn solve() {
    let file = read_file();
    let mut sum1 = 0;
    let mut sum2 = 0;
    for range in file.split(",") {
        let (start, end) = parse_range(range.trim());
        sum1 += get_invalid_sum_in_range_part1(start, end);
        sum2 += get_invalid_sum_in_range_part2(start, end);
    }
    println!("Day 2 Part 1 is {}", sum1);
    println!("Day 2 Part 2 is {}", sum2);
}

fn get_invalid_sum_in_range_part1(start: usize, end: usize) -> usize {
    let mut invalids = 0;
    'outer: for i in start..(end + 1) {
        let s = i.to_string();
        let len = s.len();
        if s.len() % 2 == 1 {
            continue;
        }

        for (a, b) in s.bytes().take(len / 2).zip(s.bytes().skip(len / 2)) {
            if a != b {
                continue 'outer;
            }
        }
        invalids += i;
    }
    invalids
}

fn get_invalid_sum_in_range_part2(start: usize, end: usize) -> usize {
    let mut invalids = 0;
    'outer: for i in start..(end + 1) {
        let s = i.to_string();
        let len = s.len();
        for b_len in 1..=len / 2 {
            //Only evenly idivide
            if len % b_len != 0 {
                continue;
            }

            let b = &s[..b_len];
            if b.repeat(len / b_len) == s {
                invalids += i;
                continue 'outer;
            }
        }
    }
    invalids
}

fn read_file() -> String {
    return fs::read_to_string("./input/day2.txt").unwrap();
}

fn parse_range(id: &str) -> (usize, usize) {
    let numbers: Vec<_> = id.split("-").collect();
    (
        numbers[0].parse::<usize>().unwrap(),
        numbers[1].parse::<usize>().unwrap(),
    )
}
