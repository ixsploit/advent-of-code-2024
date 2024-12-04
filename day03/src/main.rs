use std::fs;
use std::io;
use regex::Regex;
use std::error::Error;

fn read_input(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

fn get_uncorrupted_entries(input: &str) -> Result<String, Box<dyn Error>> {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let matches: Vec<&str> = re.find_iter(&input).map(|m| m.as_str()).collect();
    let result = matches.join(" ");
    Ok(result)
}

fn get_enabled_entries(input: &str) -> Result<i64, Box<dyn Error>> {
    let instructions = input; 
    let correct_instruction =
        Regex::new(r"(?<yes>do\(\))|(?<no>don't)|(?<mul>mul\((?<left>\d+),(?<right>\d+)\))")
            .unwrap();
    let mut activated = true;
    let mut result = 0i64;

    for cap in correct_instruction.captures_iter(&instructions) {
        if cap.name("yes").is_some() {
            activated = true;
        }
        if cap.name("no").is_some() {
            activated = false;
        }
        if let Some(mul) = cap.name("mul") {
            if activated {
                result += cap.name("left").unwrap().as_str().parse::<i64>()? 
                    * cap.name("right").unwrap().as_str().parse::<i64>()?;
            }
        }


    }
    Ok(result)

}
fn multiply_and_sum_entries(input: &str) -> Result<i64, Box<dyn Error>> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)")?;
    let mut sum = 0i64;
    for caps in re.captures_iter(&input) {
        let x1: i32 = caps[1].parse()?; 
        let x2: i32 = caps[2].parse()?; 
        let product = x1 as i64 * x2 as i64; 
        sum += product; 
    }
    Ok(sum)
}

fn corrupted_mul() -> Result<(), Box<dyn Error>> {
    let input = read_input("input/input.txt")?;
    let uncorrupted = get_uncorrupted_entries(&input)?;
    let sum = multiply_and_sum_entries(&uncorrupted)?;
    println!("Sum: {}", sum);
    let sum2 = get_enabled_entries(&input)?;
    println!("Sum: {}", sum2);
    Ok(())  
}
fn main() {
    match corrupted_mul() {
        Ok(_) => {
            println!("Safe reports calculated.");
        }
        Err(e) => {
            eprintln!("Failed to calculate corrupted multiplication: {}", e);
        }
    }
}
