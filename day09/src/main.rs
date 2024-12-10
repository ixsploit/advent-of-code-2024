use std::fs;
use std::io;
use std::error::Error;

fn read_input(path: &str) -> io::Result<String> {
    fs::read_to_string(path).map(|s| s.trim().to_string())
}

fn pushx(fs: &mut Vec<String>, c: String, times: usize) {
    for _ in 0..times {
        fs.push(c.clone()); // Clone the string to push multiple times
    }
}

fn read_into_fs(input: &str) -> Vec<String> {
    let input_vec: Vec<char> = input.chars().collect();
    let mut fs: Vec<String> = Vec::new(); // Now contains strings
    for (k, v) in input_vec.iter().enumerate() {
        let times = v.to_digit(10).unwrap() as usize;
        if k % 2 == 0 {
            let id = (k / 2).to_string();
            pushx(&mut fs, id, times);
        } else {
            pushx(&mut fs, '.'.to_string(), times);
        }
    }
    fs
}

fn format_fs(fs: &mut Vec<String>) {
    let mut left = 0 as usize;
    let mut right = fs.len()-1 as usize;
    while left < right{
        while fs[left] != "." && left < right {
            left += 1;
        }
        while fs[right] == "." && left < right{
            right -= 1;
        }
        fs[left] = fs[right].clone();
        fs[right] = '.'.to_string();
    }
}

fn checksum(fs: Vec<String>) -> usize {
    let mut res = 0 as usize;
    for (k, v) in fs.iter().enumerate() {
        if v == "." {
            return res
        } else {
            res += k * v.parse::<usize>().unwrap(); 
        }
    }
    res
}

fn star_one() -> Result<usize, Box<dyn Error>> {
    let input = read_input("input/input.txt")?;
    let mut fs = read_into_fs(&input);
    format_fs(&mut fs);
    Ok(checksum(fs))
}

fn star_two() -> Result<usize, Box<dyn Error>> {
    let input = read_input("input/input_test.txt")?;
    Ok(1)
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

