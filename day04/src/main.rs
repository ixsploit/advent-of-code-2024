use std::fs;
use std::io;
use std::error::Error;

fn read_input(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}
fn to_matrix(input: &str) -> Vec<Vec<char>> {
    input
        .lines() 
        .map(|line| line.chars().collect()) 
        .collect() 
}
fn read_diagonals(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = matrix.len();
    let cols = matrix.get(0).map_or(0, |row| row.len());
    let mut diagonals: Vec<Vec<char>> = vec![Vec::new(); rows + cols - 1]; 
    for i in 0..rows {
        for j in 0..cols {
            diagonals[i + j].push(matrix[i][j]); 
        }
    }
    diagonals
}
fn read_reverse_diagonals(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = matrix.len();
    let cols = matrix.get(0).map_or(0, |row| row.len());
    let mut reverse_diagonals: Vec<Vec<char>> = vec![Vec::new(); rows + cols - 1];
    for i in 0..rows {
        for j in 0..cols {
            let diagonal_index = cols - 1 + i - j;
            reverse_diagonals[diagonal_index].push(matrix[i][j]);
        }
    }
    reverse_diagonals
}

fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = matrix.len();
    let cols = matrix.get(0).map_or(0, |row| row.len());
    let mut transposed: Vec<Vec<char>> = vec![Vec::new(); cols]; 
    for i in 0..rows {
        for j in 0..cols {
            if transposed.len() <= j {
                transposed.push(Vec::new());
            }
            transposed[j].push(matrix[i][j]); 
        }
    }
    transposed
}
fn count_word(matrix: &Vec<Vec<char>>, word: &str) -> usize {
    let word_reversed: String = word.chars().rev().collect(); 
    let mut count = 0;
    for row in matrix {
        let row_str: String = row.iter().collect();
        count += row_str.matches(word).count();
        count += row_str.matches(&word_reversed).count();
    }
    count
}
fn star_one() ->Result<usize, Box<dyn Error>> {
    let input = read_input("input/input.txt")?;
    let word = "XMAS";
    let matrix = to_matrix(&input);
    let mut count = count_word(&matrix, word);
    let transpose = transpose(&matrix);
    count += count_word(&transpose, word);
    let diagonal = read_diagonals(&matrix);
    count += count_word(&diagonal, word);
    let reverse_diagonals = read_reverse_diagonals(&transpose);
    count += count_word(&reverse_diagonals, word);
    Ok(count)
}
fn count_x_mas(matrix: &Vec<Vec<char>>) -> usize {
    let rows = matrix.len();
    let cols = matrix.get(0).map_or(0, |row| row.len());
    let mut count = 0;
    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            if matrix[i][j] == 'A' {
                let mut left_diagonal = false;
                let mut right_diagonal = false;
                if (matrix[i - 1][j - 1] == 'M' && matrix[i + 1][j + 1] == 'S') || (matrix[i - 1][j - 1] == 'S' && matrix[i + 1][j + 1] == 'M') {
                    left_diagonal = true;
                }
                if (matrix[i - 1][j + 1] == 'M' && matrix[i + 1][j - 1] == 'S') || (matrix[i - 1][j + 1] == 'S' && matrix[i + 1][j - 1] == 'M') {
                    right_diagonal = true;
                }
                if left_diagonal && right_diagonal {
                    count += 1
                }
            }
        }
    }
    count
}
fn star_two() -> Result<usize, Box<dyn Error>> {
    let input = read_input("input/input.txt")?;
    let matrix = to_matrix(&input);
    let count = count_x_mas(&matrix);
    Ok(count)
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


