use std::fs;
use std::io::{self, Error, ErrorKind};
use std::collections::HashMap;


fn read_input(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}


fn parse_and_sort_lists(content: &str) -> Result<(Vec<i32>, Vec<i32>), Error> {
    let (mut left_list, mut right_list) = get_lists(content)?;
    left_list.sort_unstable();
    right_list.sort_unstable();
    Ok((left_list, right_list))
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

fn calc_total_distance(left_list: &Vec<i32>, right_list: &Vec<i32>) -> Result<i32, Error> {
    Ok(left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l, r)| (r - l).abs()) 
        .sum())
}

//As the lists are sorted we can use the two pointer technique
//to calculate the occurences
fn count_occurrences_sorted(list1: &[i32], list2: &[i32]) -> HashMap<i32, usize> {
    let mut counts = HashMap::new();
    let mut i = 0; 
    let mut j = 0; 
    while i < list1.len() {
        let value = list1[i];
        let mut count = 0;
        while j < list2.len() && list2[j] < value {
            j += 1; 
        }
        while j < list2.len() && list2[j] == value {
            count += 1;
            j += 1; 
        }
        counts.insert(value, count);
        i += 1; 
    }
    counts
}

fn calc_similarity_score(left_list: &Vec<i32>, right_list: &Vec<i32>) -> Result<i32, Error> {
    let occurrences = count_occurrences_sorted(&left_list, &right_list);
    let mut similarity_score = 0;
    for (value, count) in &occurrences {
        similarity_score += value * (*count as i32);
    }
    Ok(similarity_score)
}

fn calc_scores() -> Result<(), Error> {
    let input = read_input("input/input.txt")?;
    let (left_list, right_list) = parse_and_sort_lists(&input)?;
    match calc_total_distance(&left_list, &right_list) {
        Ok(distance) => {
            println!("Distance: {}", distance);
        }
        Err(e) => {
            eprintln!("Failed to calculate distance: {}", e);
        }
    };
    match calc_similarity_score (&left_list, &right_list) {
        Ok(distance) => {
            println!("Similarity Score: {}", distance);
        }
        Err(e) => {
            eprintln!("Failed to calculate similarity score: {}", e);
        }
    };
    Ok(())
}

fn main() {
    match calc_scores() {
        Ok(_) => {
            println!("Scores calculated successfully.");
        }
        Err(e) => {
            eprintln!("Failed to calculate scores: {}", e)
        }
    }
}
