use std::fs;
use std::io::{self, Error};


fn read_input(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

fn check_safety(report: &[i32]) -> Result<bool, Error> {
    let mut increasing = true;
    let mut decreasing = true;
    let mut distance = true;
    if report.len() <= 1 {
        return Ok(true);
    }

    for window in report.windows(2) {
        if let [a, b] = window {
            if a > b {
                decreasing = false 
            } else if b > a {
                increasing = false 
            }
            if (a-b).abs() == 0 {
                distance = false
            } else if (a-b).abs() > 3 {
                distance = false
            }
        }
    }
    Ok((increasing || decreasing) && distance)
}

fn check_safety_dampener(report: Vec<i32>) -> Result<bool, Error> {
    if check_safety(&report)? {
        return Ok(true)
    }
    for i in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(i);
        if check_safety(&modified_report)? {
            return Ok(true); 
        }
    }
    return Ok(false)

}

fn calc_safe_reports() -> Result<(), Error> {
    let input = read_input("input/input.txt")?;
    let mut results = Vec::new();

    for line in input.lines() {
        let parts: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        match check_safety_dampener(parts) {
            Ok(is_safe) => results.push(is_safe),
            Err(e) => return Err(e), 
        }

    }
    let count = results.iter().filter(|&&s| s).count();
    println!("Safe reports: {}", count);
    Ok(())



}

fn main() {
    match calc_safe_reports() {
        Ok(_) => {
            println!("Safe reports calculated.");
        }
        Err(e) => {
            eprintln!("Failed to calculate safe reports: {}", e);
        }
    }

    

    println!("Hello, world!");
}
