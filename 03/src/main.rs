use regex::Regex;
use shared_lib::read_lines;
use std::io;
use std::path::Path;

fn main() {
    let mut result = sum_array_results(get_array_from_memory("puzzle_input.txt").unwrap()).unwrap();
    println!("{}", result);
    result =
        sum_array_results(get_array_from_memory_conditional("puzzle_input.txt").unwrap()).unwrap();
    println!("{}", result);
}

fn sum_array_results(results: Vec<i32>) -> io::Result<i32> {
    Ok(results.iter().sum())
}

pub fn get_array_from_memory<P>(filename: P) -> io::Result<Vec<i32>>
where
    P: AsRef<Path>,
{
    let mut results = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        let lines: Vec<String> = lines.collect::<Result<_, _>>()?;
        let input = lines.join("");

        let re_mul = Regex::new(r"\bmul\((\d{1,3}),(\d{1,3})\)").unwrap();

        for captures in re_mul.captures_iter(&input) {
            let num1: i32 = captures[1].parse().unwrap();
            let num2: i32 = captures[2].parse().unwrap();
            results.push(num1 * num2);
        }
    }

    Ok(results)
}

pub fn get_array_from_memory_conditional<P>(filename: P) -> io::Result<Vec<i32>>
where
    P: AsRef<Path>,
{
    let mut results = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        let lines: Vec<String> = lines.collect::<Result<_, _>>()?;
        let input = lines.join("");

        let re_mul = Regex::new(r"\bmul\((\d{1,3}),(\d{1,3})\)").unwrap();
        let re_do = Regex::new(r"\bdo\(\)").unwrap();
        let re_dont = Regex::new(r"\bdon't\(\)").unwrap();

        let mut mul_enabled = true;

        let mut events = Vec::new();

        for m in re_do.find_iter(&input) {
            events.push((m.start(), "do"));
        }

        for m in re_dont.find_iter(&input) {
            events.push((m.start(), "don't"));
        }

        for m in re_mul.find_iter(&input) {
            events.push((m.start(), "mul"));
        }

        events.sort_by_key(|k| k.0);

        for event in events {
            match event.1 {
                "do" => {
                    mul_enabled = true;
                }
                "don't" => {
                    mul_enabled = false;
                }
                "mul" => {
                    if mul_enabled {
                        let m = re_mul.find_at(&input, event.0).unwrap();
                        let captures = re_mul.captures(&input[m.start()..m.end()]).unwrap();
                        let num1: i32 = captures[1].parse().unwrap();
                        let num2: i32 = captures[2].parse().unwrap();
                        results.push(num1 * num2);
                    }
                }
                _ => {}
            }
        }
    }

    Ok(results)
}
