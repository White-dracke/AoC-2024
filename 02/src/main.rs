use std::io;
use std::path::Path;

use shared_lib::read_lines;

fn is_safe(numbers: &[i32]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;
    let mut safe = true;

    for i in 1..numbers.len() {
        if numbers[i] <= numbers[i - 1] {
            increasing = false;
        }
        if numbers[i] >= numbers[i - 1] {
            decreasing = false;
        }
        if (numbers[i] - numbers[i - 1]).abs() < 1 || (numbers[i] - numbers[i - 1]).abs() > 3 {
            safe = false;
        }
    }

    (increasing || decreasing) && safe
}

fn count_safe_results<P>(filename: P) -> io::Result<usize>
where
    P: AsRef<Path>,
{
    let mut safe_count = 0;

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                let numbers: Vec<i32> = ip
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                if numbers.is_empty() {
                    continue;
                }

                if is_safe(&numbers) {
                    safe_count += 1;
                    continue;
                }
            }
        }
    }

    Ok(safe_count)
}

fn count_safe_results_with_tolerance<P>(filename: P) -> io::Result<usize>
where
    P: AsRef<Path>,
{
    let mut safe_count = 0;

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                let numbers: Vec<i32> = ip
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                if numbers.is_empty() {
                    continue;
                }

                if is_safe(&numbers) {
                    safe_count += 1;
                    continue;
                }

                let mut found_safe = false;
                for i in 0..numbers.len() {
                    let mut temp_numbers = numbers.clone();
                    temp_numbers.remove(i);
                    if is_safe(&temp_numbers) {
                        found_safe = true;
                        break;
                    }
                }

                if found_safe {
                    safe_count += 1;
                }
            }
        }
    }

    Ok(safe_count)
}

fn main() -> io::Result<()> {
    let safe_count = count_safe_results("puzzle_input.txt")?;
    println!("Number of safe results: {}", safe_count);
    let safe_count = count_safe_results_with_tolerance("puzzle_input.txt")?;
    println!("Number of safe results (safe module): {}", safe_count);
    Ok(())
}

#[cfg(test)]
mod test;
