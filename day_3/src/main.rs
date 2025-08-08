use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("day_3/data/input.txt").expect("Failed to read input file");
    let result_part_1 = part_1(&input);
    println!("Part 1: {result_part_1}");

    let result_part_2 = part_2(&input);
    println!("Part 2: {result_part_2}");
}

fn part_1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|c| {
            let x: u32 = c[1].parse().unwrap();
            let y: u32 = c[2].parse().unwrap();
            x * y
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    let re = Regex::new(r"don't\(\)|do\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut enabled = true;
    let mut sum = 0;

    for caps in re.captures_iter(input) {
        if let (Some(x), Some(y)) = (caps.get(1), caps.get(2)) {
            if enabled {
                let a: u32 = x.as_str().parse().unwrap();
                let b: u32 = y.as_str().parse().unwrap();
                sum += a * b;
            }
        } else {
            match caps.get(0).unwrap().as_str() {
                "don't()" => enabled = false,
                "do()" => enabled = true,
                _ => {}
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_day_3_part_1() {
        let input = fs::read_to_string("data/example.txt").unwrap();
        let result = part_1(&input);
        assert_eq!(result, 161);
    }

    #[test]
    fn test_day_3_part_2() {
        let input = fs::read_to_string("data/example.txt").unwrap();
        let result = part_2(&input);
        assert_eq!(result, 48);
    }
}
