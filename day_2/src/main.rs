use std::fs;

fn main() {
    let input = fs::read_to_string("day_2/data/input.txt").expect("Failed to read input file");
    let result_part_1 = part_1(&input);
    println!("Part 1: {result_part_1}");

    let result_part_2 = part_2(&input);
    println!("Part 2: {result_part_2}");
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let report: Vec<u32> = line
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            is_safe(&report) as u32
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let report: Vec<u32> = line
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            if is_safe(&report) {
                1
            } else {
                for i in 0..report.len() {
                    let mut temp = report.clone();
                    temp.remove(i);
                    if is_safe(&temp) {
                        return 1;
                    }
                }
                0
            }
        })
        .sum()
}

fn is_safe(report: &[u32]) -> bool {
    let diffs_ok = report
        .windows(2)
        .all(|w| (1..=3).contains(&w[0].abs_diff(w[1])));
    let increasing = report.windows(2).all(|w| w[0] < w[1]);
    let decreasing = report.windows(2).all(|w| w[0] > w[1]);

    diffs_ok && (increasing || decreasing)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("data/example.txt").unwrap();
        let result = part_1(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("data/example.txt").unwrap();
        let result = part_2(&input);
        assert_eq!(result, 4);
    }
}
