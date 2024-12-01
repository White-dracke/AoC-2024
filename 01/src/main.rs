use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "test_input.txt";
    let mut left_side = Vec::new();
    let mut right_side = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(ip) = line {
                let parts: Vec<&str> = ip.split_whitespace().collect();
                if parts.len() == 2 {
                    if let (Ok(left), Ok(right)) =
                        (parts[0].parse::<i32>(), parts[1].parse::<i32>())
                    {
                        left_side.push(left);
                        right_side.push(right);
                    }
                }
            }
        }
    }

    println!("Left side: {:?}", left_side);
    println!("Right side: {:?}", right_side);

    Ok(())
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
