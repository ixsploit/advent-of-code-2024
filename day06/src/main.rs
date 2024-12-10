use std::fs;
use std::io;
use std::error::Error;

impl Direction {
    fn rotate_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn next_position(self, x: usize, y: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Up => x.checked_sub(1).map(|nx| (nx, y)),       
            Direction::Left => y.checked_sub(1).map(|ny| (x, ny)),     
            Direction::Down => Some((x + 1, y)),                       
            Direction::Right => Some((x, y + 1)),                      
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CellState {
    visited_count: usize,
    content: CellContent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellContent {
    Obstacle,
    Free,
    Guard(Direction),
}

impl CellState {
    fn new(content: CellContent) -> Self {
        CellState {
            visited_count: 0,
            content,
        }
    }

    fn visit(&mut self) {
        self.visited_count += 1;
    }

    fn is_obstacle(&self) -> bool {
        matches!(self.content, CellContent::Obstacle)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl CellState {
    fn from_char(c: char) -> Self {
        match c {
            '#' => CellState::new(CellContent::Obstacle),
            '.' => CellState::new(CellContent::Free),
            '^' => CellState::new(CellContent::Guard(Direction::Up)),
            '<' => CellState::new(CellContent::Guard(Direction::Left)),
            '>' => CellState::new(CellContent::Guard(Direction::Right)),
            'V' => CellState::new(CellContent::Guard(Direction::Down)),
            _ => CellState::new(CellContent::Free),
        }
    }
}

fn find_guard(map: &[Vec<CellState>]) -> Option<(usize, usize, Direction)> {
    for (x, row) in map.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if let CellContent::Guard(direction) = cell.content {
                return Some((x, y, direction));
            }
        }
    }
    None
}

fn move_guard(map: &mut Vec<Vec<CellState>>, x: usize, y: usize, direction: Direction) -> Option<(usize, usize, Direction)> {
    if let Some((nx, ny)) = direction.next_position(x, y) {
        if nx < map.len() && ny < map[0].len() && !map[nx][ny].is_obstacle() {
            // Mark the current cell as visited
            map[x][y].visit();

            // Move the guard to the new cell
            map[x][y].content = CellContent::Free; // Clear the old cell
            map[nx][ny].content = CellContent::Guard(direction);

            return Some((nx, ny, direction));
        } else {
            // Rotate the guard if it hits an obstacle
            map[x][y].content = CellContent::Guard(direction.rotate_right());
            return Some((x, y, direction.rotate_right()));
        }
    }
    None
}

fn create_map_from_file(input: &str) -> Vec<Vec<CellState>> {
    input
        .lines()
        .map(|line| line.chars().map(CellState::from_char).collect())
        .collect()
}

fn read_input(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

fn star_one() -> Result<usize, Box<dyn Error>> {
    let input = read_input("input/input.txt")?;
    let mut map = create_map_from_file(&input);

    let (mut x, mut y, mut direction) = find_guard(&map).unwrap();
    while let Some((new_x, new_y, new_direction)) = move_guard(&mut map, x, y, direction) {
        x = new_x;
        y = new_y;
        direction = new_direction;
    }

    // Count all visited cells
    let result = map.iter().flat_map(|row| row.iter()).filter(|cell| cell.visited_count > 0).count();
    Ok(result)
}

fn star_two() -> Result<usize, Box<dyn Error>> {
    let input = read_input("input/input_test.txt")?;
    let mut map = create_map_from_file(&input);

    let (mut x, mut y, mut direction) = find_guard(&map).unwrap();
    while let Some((new_x, new_y, new_direction)) = move_guard(&mut map, x, y, direction) {
        x = new_x;
        y = new_y;
        direction = new_direction;
    }

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

