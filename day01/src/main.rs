use std::fs;
use std::io::{self, Error, ErrorKind};


fn read_input() -> io::Result<String> {
    fs::read_to_string("input/input.txt")
}

fn get_lists(content: &str) -> Result<(Vec<i32>, Vec<i32>), Error> {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for line in content.lines() {
        let mut parts = line.split_whitespace().map(|s| s.parse::<i32>());
            if let (Some(Ok(num1)), Some(Ok(num2))) = (parts.next(), parts.next()) {
                left_list.push(num1);
                right_list.push(num2);
            } else {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid file format"));
            }
        }

    Ok((left_list, right_list))
}

fn calc_total_distance() -> Result<i32, Error> {
    let input = read_input()?;
    let (mut left_list, mut right_list) = get_lists(&input)?;
    left_list.sort_unstable();
    right_list.sort_unstable();
    Ok(left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l, r)| (r - l).abs()) 
        .sum())
}

fn main() {
    match calc_total_distance() {
        Ok(distance) => {
            println!("Distance: {}", distance);
        }
        Err(e) => {
            eprintln!("Failed to calculate distance: {}", e);
        }
    }
}
