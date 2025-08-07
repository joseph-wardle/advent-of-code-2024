use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("day_1/data/input.txt").expect("Failed to read input file");
    let result_part_1 = part_1(&input);
    println!("Part 1: {result_part_1}");

    let result_part_2 = part_2(&input);
    println!("Part 2: {result_part_2}");
}

fn part_1(input: &str) -> u32 {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let mut it = line.split_whitespace();
            let a: u32 = it.next().unwrap().parse().unwrap();
            let b: u32 = it.next().unwrap().parse().unwrap();
            (a, b)
        })
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    left.iter().zip(right).map(|(a, b)| a.abs_diff(b)).sum()
}

fn part_2(input: &str) -> u32 {
    let (left, right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let mut it = line.split_whitespace();
            let a: u32 = it.next().unwrap().parse().unwrap();
            let b: u32 = it.next().unwrap().parse().unwrap();
            (a, b)
        })
        .unzip();

    let mut right_map = HashMap::new();
    for num in right {
        *right_map.entry(num).or_insert(0) += 1;
    }

    left.iter()
        .map(|&num| {
            if let Some(count) = right_map.get(&num) {
                num * count
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_day_1_part_1() {
        let input = fs::read_to_string("data/example.txt").unwrap();
        let result = part_1(&input);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_day_1_part_2() {
        let input = fs::read_to_string("data/example.txt").unwrap();
        let result = part_2(&input);
        assert_eq!(result, 31);
    }
}
