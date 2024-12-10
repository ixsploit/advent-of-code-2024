use std::fs;
use std::io;
use std::error::Error;
use hashbrown::HashMap;
use itertools::Itertools;

fn read_input(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}
struct AntinodesMap {
    length: usize,                           
    height: usize,                           
    map: Vec<Vec<bool>>,                     
}
impl AntinodesMap {
    fn new(length: usize, height: usize) -> Self {
        Self {
            length,
            height,
            map: vec![vec![false; length]; height], 
        }
    }
    fn set_outer_star1(&mut self, pair: Vec<(usize, usize)>) {
        assert_eq!(pair.len(), 2, "Pair must contain exactly two points");

        let distancex = pair[0].0 as isize - pair[1].0 as isize;
        let distancey = pair[0].1 as isize - pair[1].1 as isize;

        let anti1 = (
            pair[0].0 as isize + distancex,
            pair[0].1 as isize + distancey,
        );
        let anti2 = (
            pair[1].0 as isize - distancex,
            pair[1].1 as isize - distancey,
        );

        let set_if_within_bounds = |x: isize, y: isize, map: &mut Vec<Vec<bool>>| {
            if x >= 0 && y >= 0 && (x as usize) < map[0].len() && (y as usize) < map.len() {
                map[y as usize][x as usize] = true;
            }
        };

        set_if_within_bounds(anti1.0, anti1.1, &mut self.map);
        set_if_within_bounds(anti2.0, anti2.1, &mut self.map);
    }

    fn set_outer_star2(&mut self, pair: Vec<(usize, usize)>) {
        assert_eq!(pair.len(), 2, "Pair must contain exactly two points");
        //antenna locations are also true
        self.map[pair[0].1][pair[0].0] = true;
        self.map[pair[1].1][pair[1].0] = true;
        let distancex = pair[0].0 as isize - pair[1].0 as isize;
        let distancey = pair[0].1 as isize - pair[1].1 as isize;

        let set_if_within_bounds = |x: isize, y: isize, map: &mut Vec<Vec<bool>>| -> bool {
            if x >= 0 && y >= 0 && (x as usize) < map[0].len() && (y as usize) < map.len() {
                map[y as usize][x as usize] = true;
                return true; 
            }
            false 
        };

        
        let mut current1 = (
            pair[0].0 as isize + distancex,
            pair[0].1 as isize + distancey,
        );
        let mut current2 = (
            pair[1].0 as isize - distancex,
            pair[1].1 as isize - distancey,
        );
        
        while set_if_within_bounds(current1.0, current1.1, &mut self.map) {
            current1.0 += distancex;
            current1.1 += distancey;
        }
        while set_if_within_bounds(current2.0, current2.1, &mut self.map) {
            current2.0 -= distancex;
            current2.1 -= distancey;
        }
    }

    fn fill_star1(&mut self, hmap: HashMap<char, Vec<(usize, usize)>>) {
        let mut cmap = hmap.clone();
        cmap.remove(&'.');
        cmap.retain(|_, v| v.len() > 1);
        for(_, locations) in &cmap {
            let pairs: Vec<_> = locations.into_iter().combinations(2).collect();
            for pair in pairs {
                let pair: Vec<(usize, usize)> = pair.iter().map(|&&p| p).collect();
                self.set_outer_star1(pair);
            }
        }
    }
    fn fill_star2(&mut self, hmap: HashMap<char, Vec<(usize, usize)>>) {
        let mut cmap = hmap.clone();
        cmap.remove(&'.');
        cmap.retain(|_, v| v.len() > 1);
        for(_, locations) in &cmap {
            let pairs: Vec<_> = locations.into_iter().combinations(2).collect();
            for pair in pairs {
                let pair: Vec<(usize, usize)> = pair.iter().map(|&&p| p).collect();
                self.set_outer_star2(pair);
            }
        }
    }
    fn print_map(&mut self) {
        for row in self.map.clone() {
            let line: String = row
                .iter()
                .map(|&cell| if cell { '#' } else { '.' }) 
                .collect(); 
            println!("{}", line); 
        }
    }
    
    fn count(&mut self) -> usize {
        self.map
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&value| value)
            .count()
    }
}
fn read_map(path: &str) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let input = read_input(path)?;
    let map = input.lines().map(
        |line| line.chars().collect())
        .collect();
    Ok(map)

}
fn frequency_map(map: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut hmap = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            hmap.entry(ch)
                .or_insert_with(Vec::new)
                .push((x, y));
        }
    }
    hmap
}
fn get_dimensions(map: &Vec<Vec<char>>) -> (usize, usize) {
    let height = map.len();
    let width = if height > 0 { map[0].len() } else { 0 };
    (height, width)
}


fn star_one() -> Result<usize, Box<dyn Error>> {
    let map = read_map("input/input.txt")?;
    let frequency_map = frequency_map(&map);
    let (length, height) = get_dimensions(&map);
    let mut antinodes = AntinodesMap::new(length, height);
    antinodes.fill_star1(frequency_map);
    Ok(antinodes.count())
}
fn star_two() -> Result<usize, Box<dyn Error>> {
    let map = read_map("input/input.txt")?;
    let frequency_map = frequency_map(&map);
    let (length, height) = get_dimensions(&map);
    let mut antinodes = AntinodesMap::new(length, height);
    antinodes.fill_star2(frequency_map);
    Ok(antinodes.count())
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


