use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "puzzle_input.txt";
    let total_distance = calculate_total_distance(path)?;
    println!("Total distance: {}", total_distance);
    let score = calculate_total_score(path)?;
    print!("Total score {}: ", score);
    Ok(())
}

fn calculate_similarity_list(left_side: Vec<i32>, right_side: Vec<i32>) -> Vec<i32> {
    let result = left_side
        .iter()
        .map(|left| left * right_side.iter().filter(|&&right| right == *left).count() as i32)
        .collect();
    return result;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_sides<P>(filename: P) -> io::Result<(Vec<i32>, Vec<i32>)>
where
    P: AsRef<Path>,
{
    let mut left_side = Vec::new();
    let mut right_side = Vec::new();

    if let Ok(lines) = read_lines(filename) {
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

    Ok((left_side, right_side))
}

pub fn calculate_total_score<P>(filename: P) -> io::Result<i32>
where
    P: AsRef<Path>,
{
    let (left_side, right_side) = read_sides(filename)?;
    let scores = calculate_similarity_list(left_side, right_side);
    let total_score: i32 = scores.iter().sum();
    Ok(total_score)
}

pub fn calculate_total_distance<P>(filename: P) -> io::Result<i32>
where
    P: AsRef<Path>,
{
    let (mut left_side, mut right_side) = read_sides(filename)?;
    let distances = calculate_distances(&mut left_side, &mut right_side);
    let total_distance: i32 = distances.iter().sum();
    Ok(total_distance)
}

fn calculate_distances(left_side: &mut Vec<i32>, right_side: &mut Vec<i32>) -> Vec<i32> {
    left_side.sort();
    right_side.sort();

    left_side
        .iter()
        .zip(right_side.iter())
        .map(|(left, right)| (left - right).abs())
        .collect()
}

#[cfg(test)]
mod test;
