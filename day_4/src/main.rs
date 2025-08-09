use std::fs;

fn main() {
    let input = fs::read_to_string("day_4/data/input.txt").expect("Failed to read input file");
    let result_part_1 = part_1(&input);
    println!("Part 1: {result_part_1}");

    let result_part_2 = part_2(&input);
    println!("Part 2: {result_part_2}");
}

fn part_1(input: &str) -> u32 {
    const WORD: &[u8] = b"XMAS";
    const DIRECTIONS: &[(isize, isize)] = &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let grid: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut total = 0;

    for y in 0..rows {
        for x in 0..cols {
            for &(dy, dx) in DIRECTIONS {
                for (k, char) in WORD.iter().enumerate() {
                    let ny = y as isize + dy * k as isize;
                    let nx = x as isize + dx * k as isize;

                    if ny < 0 || nx < 0 {
                        break;
                    }
                    if ny >= rows as isize || nx >= cols as isize {
                        break;
                    }
                    let (ny, nx) = (ny as usize, nx as usize);

                    if grid[ny][nx] != *char {
                        break;
                    }

                    if k == WORD.len() - 1 {
                        total += 1;
                    }
                }
            }
        }
    }

    total
}

fn part_2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_day_4_part_1() {
        let input = fs::read_to_string("data/example.txt").unwrap();
        let result = part_1(&input);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_day_4_part_2() {
        let input = fs::read_to_string("data/example.txt").unwrap();
        let result = part_2(&input);
        assert_eq!(result, 0);
    }
}
