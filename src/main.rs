use advent_of_code_2024::days;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let (day, part) = parse_arguments();
    let input = load_input(&day);
    let result = solve(&day, &part, &input);
    println!("{} Part {}: {}", day, part, result);
}

fn parse_arguments() -> (String, String) {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print_usage();
    }

    let day = args[1].clone();
    validate_day(&day);

    let part = args[2].clone();
    validate_part(&part);

    (day, part)
}

fn print_usage() -> ! {
    eprintln!("Usage: cargo run -- dayXX part");
    eprintln!("Example: cargo run -- day01 1");
    std::process::exit(1);
}

fn validate_day(day: &str) {
    if !day.starts_with("day") || day.len() != 5 || !day[3..].chars().all(|c| c.is_ascii_digit()) {
        eprintln!("Error: Day must be in the format 'dayXX' where XX is the day number");
        std::process::exit(1);
    }
}

fn validate_part(part: &str) {
    if part != "part01" && part != "part02" {
        eprintln!("Error: Part must be either 'part01' or 'part02'");
        std::process::exit(1);
    }
}

fn load_input(day: &str) -> String {
    let filename = format!("{}.txt", day);
    let filepath = Path::new("data/inputs").join(filename);

    fs::read_to_string(&filepath).unwrap_or_else(|e| {
        eprintln!("Failed to read input file '{}': {}", filepath.display(), e);
        std::process::exit(1);
    })
}

fn solve(day: &str, part: &str, input: &str) -> u64 {
    match (day, part) {
        _ => {
            eprintln!("Solution for {} Part {} is not implemented.", day, part);
            std::process::exit(1);
        }
    }
}
