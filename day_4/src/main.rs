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

fn part_2(input: &str) -> u32 {
    let grid: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut total = 0;

    for y in 1..(rows - 1) {
        for x in 1..(cols - 1) {
            if grid[y][x] != b'A' {
                continue;
            }

            let top_left = grid[y - 1][x - 1];
            let top_right = grid[y - 1][x + 1];
            let bottom_left = grid[y + 1][x - 1];
            let bottom_right = grid[y + 1][x + 1];

            let diag1_ok = (top_left == b'M' && bottom_right == b'S')
                || (top_left == b'S' && bottom_right == b'M');
            let diag2_ok = (top_right == b'M' && bottom_left == b'S')
                || (top_right == b'S' && bottom_left == b'M');

            if diag1_ok && diag2_ok {
                total += 1;
            }
        }
    }

    total
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
        assert_eq!(result, 9);
    }
}
