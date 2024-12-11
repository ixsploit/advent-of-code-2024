use std::fs;
use std::io;
use std::error::Error;
use cached::proc_macro::cached;

fn read_input(path: &str) -> io::Result<String> {
    fs::read_to_string(path).map(|s| s.trim().to_string())
} 

fn apply_rules(number: usize) -> Vec<usize> {
    if number == 0 {
        return vec![1];
    }
    let len = number.to_string().len();
    if len % 2 == 0 {
        let left: usize = number.to_string()[..len/2].parse().unwrap();
        let right: usize = number.to_string()[len/2..].parse().unwrap();
        return vec![left, right];
    }
    vec![number*2024]

}

#[cached]
fn apply_rules_recursive(number: usize, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }
    if number == 0 {
        return apply_rules_recursive(1, depth-1);
    }
    let len = number.to_string().len();
    if len % 2 == 0 {
        let left: usize = number.to_string()[..len/2].parse().unwrap();
        let right: usize = number.to_string()[len/2..].parse().unwrap();
        return apply_rules_recursive(left, depth-1)+apply_rules_recursive(right, depth-1);
    }
    apply_rules_recursive(number*2024, depth-1)

}
fn star_one() -> Result<usize, Box<dyn Error>> {
    let input = read_input("input/input.txt")?;
    let mut stones: Vec<usize> = input.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
    for _ in 1..=25 {
        stones = stones
            .iter()
            .flat_map(|stone| apply_rules(*stone)) // Apply rules and flatten results
            .collect();
    }
    Ok(stones.len())
}
fn star_two() -> Result<usize, Box<dyn Error>> {
    let input = read_input("input/input.txt")?;
    let stones: Vec<usize> = input.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
    let result = stones
            .iter()
            .map(|stone| apply_rules_recursive(*stone, 75)) 
            .sum();
    Ok(result)

}

fn main() {
    match star_one() {
        Ok(i) => println!("Star one result: {}", i),
        Err(e) => eprintln!("Failed star one: {}", e),
    }
    match star_two() {
        Ok(i) => println!("Star two result: {}", i),
        Err(e) => eprintln!("Failed star two: {}", e),
    }
}





