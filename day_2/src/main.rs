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

            let diffs_ok = report
                .windows(2)
                .all(|w| (1..=3).contains(&w[0].abs_diff(w[1])));
            let increasing = report.windows(2).all(|w| w[0] < w[1]);
            let decreasing = report.windows(2).all(|w| w[0] > w[1]);

            let is_safe = diffs_ok && (increasing || decreasing);
            is_safe as u32
        })
        .sum()
}

fn part_2(_input: &str) -> u32 {
    0
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
        assert_eq!(result, 0);
    }
}
