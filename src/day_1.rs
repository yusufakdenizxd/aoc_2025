use std::fs;

pub fn solve() {
    let file = read_file();
    let mut current = 50;
    let mut part1_password = 0;
    let mut part2_password = 0;
    for line in file.split("\n") {
        if line.trim().is_empty() {
            continue;
        }

        let (left_or_right, value) = parse_line(line);
        turn_lock(&mut current, left_or_right, value, &mut part2_password);

        if current == 0 {
            part1_password += 1;
        }
    }
    println!("Part1 Password is {}", part1_password);
    println!("Part2 Password is {}", part2_password);
}

fn read_file() -> String {
    return fs::read_to_string("./input/day1.txt").unwrap();
}

//Left is false right is true
fn parse_line(line: &str) -> (bool, i32) {
    let line = line.trim();
    let direction = &line[0..1];
    let number_str = &line[1..];

    let left_or_right = direction == "R";
    let value: i32 = number_str.parse().unwrap();

    (left_or_right, value)
}

fn turn_lock(current: &mut i32, left_or_right: bool, value: i32, part2password: &mut i32) {
    let s = *current;
    if left_or_right {
        let tmp = s - value;
        let a = tmp.div_euclid(100);
        let b = tmp.rem_euclid(100);
        *part2password += a.abs();
        if s == 0 && b > 0 {
            *part2password -= 1;
        } else if s > 0 && b == 0 {
            *part2password += 1;
        }
        *current = b;
    } else {
        let tmp = s + value;
        let (a, b) = (tmp / 100, tmp % 100);
        *part2password += a;
        *current = b;
    }
}
