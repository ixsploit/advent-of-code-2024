use std::fs;
use std::io;
use std::error::Error;
use std::collections::HashSet;

fn read_input(path: &str) -> io::Result<String> {
    fs::read_to_string(path).map(|s| s.trim().to_string())
} 

fn create_map(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as usize))
                .collect()
        })
        .collect()
}

fn traverse(map: &Vec<Vec<usize>>, row: usize, col: usize) -> HashSet<(usize, usize)> {
    let value = map[row][col];
    if value == 9 {
        let mut set: HashSet<(usize, usize)> = HashSet::new();
        set.insert((row,col));
        return set; 
    }
    let directions = [
        (row.checked_sub(1), Some(col)),    
        (Some(row + 1), Some(col)),         
        (Some(row), col.checked_sub(1)),    
        (Some(row), Some(col + 1)),         
    ];
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    for (checked_row, checked_col) in directions.iter() {
        if let (Some(r), Some(c)) = (checked_row, checked_col) {
            if *r < map.len() && *c < map[0].len() {
                if map[*r][*c] == value + 1 {
                    set.extend(traverse(map, *r, *c));
                }
            }
        }
    }
    set 
}

fn traverse2(map: &Vec<Vec<usize>>, row: usize, col: usize) -> usize {
    let value = map[row][col];
    if value == 9 {
        return 1; 
    }
    let directions = [
        (row.checked_sub(1), Some(col)),    
        (Some(row + 1), Some(col)),         
        (Some(row), col.checked_sub(1)),    
        (Some(row), Some(col + 1)),         
    ];
    let mut total = 0;
    for (checked_row, checked_col) in directions.iter() {
        if let (Some(r), Some(c)) = (checked_row, checked_col) {
            if *r < map.len() && *c < map[0].len() {
                if map[*r][*c] == value + 1 {
                    total += traverse2(map, *r, *c);
                }
            }
        }
    }
    total 
}
fn star_one() -> Result<usize, Box<dyn Error>> {
    let input = read_input("input/input.txt")?;
    let map = create_map(&input);
    let total_paths: usize = map.iter().enumerate().flat_map(|(row, line)| {
        let map_ref = &map;
        line.iter().enumerate().filter_map(move |(col, &val)| {
            if val == 0 {
                Some(traverse(map_ref, row, col).len())
            } else {
                None 
            }
        })
    }).sum();


    Ok(total_paths)
}

fn star_two() -> Result<usize, Box<dyn Error>> {
    let input = read_input("input/input.txt")?;
    let map = create_map(&input);
    let total_paths: usize = map.iter().enumerate().flat_map(|(row, line)| {
        let map_ref = &map;
        line.iter().enumerate().filter_map(move |(col, &val)| {
            if val == 0 {
                Some(traverse2(map_ref, row, col))
            } else {
                None 
            }
        })
    }).sum();
    Ok(total_paths)
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




