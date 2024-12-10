use std::fs;
use std::io;
use std::error::Error;

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    //Subtract,
    Multiply,
    //Divide,
    Merge,
}

impl Operator {
    fn apply(&self, a: i64, b: i64) -> Option<i64> {
        match self {
            Operator::Add => Some(a + b),
            //Operator::Subtract => Some(a - b),
            Operator::Multiply => Some(a * b),
            Operator::Merge => {
                let concatenated = format!("{}{}", a, b);
                Some(concatenated.parse::<i64>().unwrap())
            }
            //Operator::Divide => {
            //    if b == 0 {
            //        None // Avoid division by zero
            //    } else {
            //        Some(a / b)
            //    }
            //}
        }
    }
}

fn read_input(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

fn evaluate_expression(numbers: &[i64], operators: &[Operator]) -> Option<i64> {
    let mut result = numbers[0];
    for (i, &op) in operators.iter().enumerate() {
        result = op.apply(result, numbers[i + 1])?;
    }
    Some(result)
}

fn operator_combinations(operators: &[Operator], length: usize) -> Vec<Vec<Operator>> {
    if length == 0 {
        return vec![vec![]];
    }
    let mut results = Vec::new();
    let smaller_combinations = operator_combinations(operators, length - 1);
    for &operator in operators {
        for mut combination in smaller_combinations.clone() {
            combination.push(operator);
            results.push(combination);
        }
    }
    results
}

fn get_result(parts: Vec<&str>, target: i64) -> Option<Vec<(Vec<i64>, Vec<Operator>)>> {
    let numbers: Vec<i64> = parts[1]
        .trim()
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect();
    
    let operators = vec![
        Operator::Add,
        //Operator::Subtract,
        Operator::Multiply,
        //Operator::Divide,
        Operator::Merge,
    ];
    
    let mut solutions = Vec::new();
    
    for operator_set in operator_combinations(&operators, numbers.len() - 1) {
        if let Some(result) = evaluate_expression(&numbers, &operator_set) {
            if result == target {
                solutions.push((numbers.clone(), operator_set));
                return Some(solutions); //we just need to find one solution
            }
        }
    }
    
    if solutions.is_empty() {
        None
    } else {
        Some(solutions)
    }
}

fn get_solutions(input: &str) -> i64 {
    let parts: Vec<&str> = input.split(':').collect();
    let target: i64 = parts[0].trim().parse().unwrap();
    println!("Target: {}", target);
    let solutions = get_result(parts, target);
    
    match solutions {
        Some(ref sols) => {
            for (numbers, operators) in sols {
                println!("Solution found: {:?} with operators {:?}", numbers, operators);
            }
            return target;
        },
        None => {return 0; }
    }
}

fn star_one() -> Result<i64, Box<dyn Error>> {
    let input = read_input("input/input.txt")?;
    let mut result = 0;
    
    for line in input.lines() {
        result += get_solutions(line);
        println!("Result: {}", result);
    }
    
    Ok(result)
}

fn star_two() -> Result<usize, Box<dyn Error>> {
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
