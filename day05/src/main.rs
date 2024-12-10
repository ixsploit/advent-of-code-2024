use std::fs;
use std::io;
use std::error::Error;
use hashbrown::{HashMap, HashSet};


struct Sections {
    rules: Vec<(usize, usize)>,
    updates: Vec<Vec<usize>>,
}
fn read_input(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}
fn parse_input(input: &str) -> Sections {
    let mut sections = input.split("\n\n");
    
    let rules = sections
        .next()
        .unwrap_or("")
        .lines()
        .filter_map(|line| {
            let mut parts = line.split('|');
            let a = parts.next()?.parse::<usize>().ok()?;
            let b = parts.next()?.parse::<usize>().ok()?;
            Some((a, b))
        })
        .collect::<Vec<(usize, usize)>>();

    let updates = sections
        .next()
        .unwrap_or("")
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|num| num.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    Sections{rules, updates}
}
fn check_rules(update: &Vec<usize>, rules: &Vec<(usize, usize)>) -> usize {
    let valid = rules.iter().all(|line| {
        let left_position = update.iter().position(|&x| x == line.0);
        let right_position = update.iter().position(|&y| y == line.1);
        match (left_position, right_position) {
            (Some(left_idx), Some(right_idx)) => left_idx < right_idx, 
            _ => true, 
        }
    });

    if valid {
        update[update.len() / 2] 
    } else {
        0 
    }
}


fn star_one() -> Result<usize, Box<dyn Error>> {
    let input = read_input("input/input.txt")?;
    let sections = parse_input(&input);
    let result: usize = sections.updates.into_iter().map(|update| {
        check_rules(&update, &sections.rules)
    }).sum();
    Ok(result)
}

fn star_two() -> Result<usize, Box<dyn Error>> {
    let input = read_input("input/input.txt")?;
    let sections = parse_input(&input);
    let mut orderings = HashMap::<usize, HashSet<usize>>::new();
    for rule in &sections.rules {
        orderings.entry(rule.1).or_default().insert(rule.0);
    }
    let rules = &sections.rules;
    let wrong_order: Vec<_> = sections
        .updates
        .into_iter()
        .filter(|update| check_rules(update, rules) == 0)
        .collect();
    let result = wrong_order.into_iter().map(|mut update| {
        update.sort_by(|a, b| orderings[b].contains(a).cmp(&true));
        update[update.len() / 2]
    }).sum();
    Ok(result)
}

fn main() {
    match star_one() {
        Ok(i) => {
            println!("Star one result: {}", i);
        }
        Err(e) => {
            eprintln!("Failed star one: {}", e);
        }
    }
    match star_two() {
        Ok(i) => {
            println!("Star two result: {}", i);
        }
        Err(e) => {
            eprintln!("Failed star two: {}", e);
        }
    }
}


