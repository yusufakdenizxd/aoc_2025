use std::fs;

pub fn solve() {
    let file = read_file();
    let mut total_joltage_part1 = 0;
    let mut total_joltage_part2 = 0;

    for line in file.lines() {
        if line.trim().is_empty() {
            continue;
        }
        total_joltage_part1 += find_largest_joltage(line, 2);
        total_joltage_part2 += find_largest_joltage(line, 12);
    }
    println!("Day 3 Part 1 is {}", total_joltage_part1);
    println!("Day 3 Part 2 is {}", total_joltage_part2);
}

fn read_file() -> String {
    return fs::read_to_string("./input/day3.txt").unwrap();
}

fn find_largest_joltage(line: &str, k: usize) -> usize {
    let nums: Vec<usize> = line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut stack: Vec<usize> = Vec::new();
    let n = nums.len();

    for (i, &d) in nums.iter().enumerate() {
        while let Some(&t) = stack.last() {
            let r = n - i;
            if (t < d) && stack.len() + r > k {
                stack.pop();
            } else {
                break;
            }
        }

        if stack.len() < k {
            stack.push(d);
        }
    }

    let mut num = 0;
    for d in stack {
        num = num * 10 + d;
    }

    num
}
