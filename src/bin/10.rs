use std::{
    collections::HashSet,
    ops::{Add, Div, Mul, Sub},
};

use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Tile {
    position: (usize, usize),
    pipe: Pipe,
}

impl Tile {
    fn new(position: (usize, usize), pipe: Pipe) -> Tile {
        Tile { position, pipe }
    }

    fn get_neighbours(&self, grid: &Vec<Vec<Tile>>) -> Vec<Tile> {
        let mut neighbours = vec![];
        let grid_height: usize = grid.len() - 1;
        let grid_width = grid[0].len() - 1;

        match grid[self.position.0][self.position.1].pipe {
            Pipe::NorthSouth => {
                // North
                if self.position.0 > 0 {
                    if matches!(
                        grid[self.position.0 - 1][self.position.1].pipe,
                        Pipe::NorthSouth | Pipe::SouthWest | Pipe::SouthEast | Pipe::Start
                    ) {
                        neighbours.push(grid[self.position.0 - 1][self.position.1])
                    }
                }
                // South
                if self.position.0 < grid_height {
                    if matches!(
                        grid[self.position.0 + 1][self.position.1].pipe,
                        Pipe::NorthSouth | Pipe::NorthWest | Pipe::NorthEast | Pipe::Start
                    ) {
                        neighbours.push(grid[self.position.0 + 1][self.position.1])
                    }
                }
            }
            Pipe::EastWest => {
                // East
                if self.position.1 < grid_width {
                    if matches!(
                        grid[self.position.0][self.position.1 + 1].pipe,
                        Pipe::EastWest | Pipe::NorthWest | Pipe::SouthWest | Pipe::Start
                    ) {
                        neighbours.push(grid[self.position.0][self.position.1 + 1])
                    }
                }
                // West
                if self.position.1 > 0 {
                    if matches!(
                        grid[self.position.0][self.position.1 - 1].pipe,
                        Pipe::EastWest | Pipe::NorthEast | Pipe::SouthEast | Pipe::Start
                    ) {
                        neighbours.push(grid[self.position.0][self.position.1 - 1])
                    }
                }
            }
            Pipe::NorthEast => {
                // North
                if self.position.0 > 0 {
                    if matches!(
                        grid[self.position.0 - 1][self.position.1].pipe,
                        Pipe::NorthSouth | Pipe::SouthWest | Pipe::SouthEast | Pipe::Start
                    ) {
                        neighbours.push(grid[self.position.0 - 1][self.position.1])
                    }
                }
                // East
                if self.position.1 < grid_width {
                    if matches!(
                        grid[self.position.0][self.position.1 + 1].pipe,
                        Pipe::EastWest | Pipe::NorthWest | Pipe::SouthWest | Pipe::Start
                    ) {
                        neighbours.push(grid[self.position.0][self.position.1 + 1])
                    }
                }
            }
            Pipe::NorthWest => {
                // North
                if self.position.0 > 0 {
                    if matches!(
                        grid[self.position.0 - 1][self.position.1].pipe,
                        Pipe::NorthSouth | Pipe::SouthWest | Pipe::SouthEast | Pipe::Start
                    ) {
                        neighbours.push(grid[self.position.0 - 1][self.position.1])
                    }
                }

                // West
                if self.position.0 > 0 {
                    if matches!(
                        grid[self.position.0][self.position.1 - 1].pipe,
                        Pipe::EastWest | Pipe::NorthEast | Pipe::SouthEast | Pipe::Start
                    ) {
                        neighbours.push(grid[self.position.0][self.position.1 - 1])
                    }
                }
            }
            Pipe::SouthWest => {
                // West
                if self.position.1 > 0 {
                    if matches!(
                        grid[self.position.0][self.position.1 - 1].pipe,
                        Pipe::EastWest | Pipe::NorthEast | Pipe::SouthEast | Pipe::Start
                    ) {
                        neighbours.push(grid[self.position.0][self.position.1 - 1])
                    }
                }
                // South
                if self.position.0 < grid_height {
                    if matches!(
                        grid[self.position.0 + 1][self.position.1].pipe,
                        Pipe::NorthSouth | Pipe::NorthWest | Pipe::NorthEast | Pipe::Start
                    ) {
                        neighbours.push(grid[self.position.0 + 1][self.position.1])
                    }
                }
            }
            Pipe::SouthEast => {
                // East
                if self.position.1 < grid_width {
                    if matches!(
                        grid[self.position.0][self.position.1 + 1].pipe,
                        Pipe::EastWest | Pipe::NorthWest | Pipe::SouthWest | Pipe::Start
                    ) {
                        neighbours.push(grid[self.position.0][self.position.1 + 1])
                    }
                }

                // South
                if self.position.0 < grid_height {
                    if matches!(
                        grid[self.position.0 + 1][self.position.1].pipe,
                        Pipe::NorthSouth | Pipe::NorthWest | Pipe::NorthEast | Pipe::Start
                    ) {
                        neighbours.push(grid[self.position.0 + 1][self.position.1])
                    }
                }
            }
            Pipe::Start => {
                // North
                if self.position.0 > 0 {
                    if matches!(
                        grid[self.position.0 - 1][self.position.1].pipe,
                        Pipe::NorthSouth | Pipe::SouthEast | Pipe::SouthWest
                    ) {
                        neighbours.push(grid[self.position.0 - 1][self.position.1])
                    }
                }

                // South
                if self.position.0 < grid_height {
                    if matches!(
                        grid[self.position.0 + 1][self.position.1].pipe,
                        Pipe::NorthSouth | Pipe::NorthEast | Pipe::NorthWest
                    ) {
                        neighbours.push(grid[self.position.0 + 1][self.position.1])
                    }
                }

                // West
                if self.position.1 > 0 {
                    if matches!(
                        grid[self.position.0][self.position.1 - 1].pipe,
                        Pipe::EastWest | Pipe::NorthEast | Pipe::SouthEast
                    ) {
                        neighbours.push(grid[self.position.0][self.position.1 - 1])
                    }
                }

                // East
                if self.position.1 < grid_width {
                    if matches!(
                        grid[self.position.0][self.position.1 + 1].pipe,
                        Pipe::EastWest | Pipe::NorthWest | Pipe::SouthWest
                    ) {
                        neighbours.push(grid[self.position.0][self.position.1 + 1])
                    }
                }
            }
            Pipe::Ground => (),
        }

        neighbours
    }
}

fn get_pipe(c: char) -> Pipe {
    match c {
        '|' => Pipe::NorthSouth,
        '-' => Pipe::EastWest,
        'L' => Pipe::NorthEast,
        'J' => Pipe::NorthWest,
        '7' => Pipe::SouthWest,
        'F' => Pipe::SouthEast,
        'S' => Pipe::Start,
        '.' => Pipe::Ground,
        _ => panic!("Unknown pipe: {}", c),
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<Tile>> = Vec::new();
    let mut start: Tile = Tile::new((0, 0), Pipe::Ground);

    input.lines().enumerate().for_each(|(r, line)| {
        let mut row: Vec<Tile> = Vec::new();
        line.chars().enumerate().for_each(|(c, ch)| {
            let pipe = get_pipe(ch);
            let tile = Tile::new((r, c), pipe);
            if tile.pipe == Pipe::Start {
                start = tile.clone();
            }
            row.push(tile);
        });
        grid.push(row);
    });

    let mut path: HashSet<Tile> = HashSet::new();
    path.insert(start);
    let mut to_explore: Vec<Tile> = start.get_neighbours(&grid);

    while to_explore.len() > 0 {
        let current = to_explore.pop().unwrap();
        for neighbour in current.get_neighbours(&grid) {
            if !path.contains(&neighbour) {
                to_explore.push(neighbour);
                path.insert(neighbour);
            }
        }
    }

    Some((path.len() / 2) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<Tile>> = Vec::new();
    let mut start: Tile = Tile::new((0, 0), Pipe::Ground);

    input.lines().enumerate().for_each(|(r, line)| {
        let mut row: Vec<Tile> = Vec::new();
        line.chars().enumerate().for_each(|(c, ch)| {
            let pipe = get_pipe(ch);
            let tile = Tile::new((r, c), pipe);
            if tile.pipe == Pipe::Start {
                start = tile.clone();
            }
            row.push(tile);
        });
        grid.push(row);
    });

    let mut path: Vec<Tile> = Vec::new();
    path.push(start);
    let mut to_explore: Vec<Tile> = start.get_neighbours(&grid);

    while to_explore.len() > 0 {
        let current = to_explore.pop().unwrap();
        for neighbour in current.get_neighbours(&grid) {
            if !path.contains(&neighbour) {
                to_explore.push(neighbour);
                path.push(neighbour);
            }
        }
    }

    println!("{:?}", path);

    //Some(shoelacea(&path).mul(2).sub(path.len() as i32).div(2).add(1) as u32)
    Some(0)
}

pub fn shoelacea(tiles: &Vec<Tile>) -> i32 {
    let len = tiles.len();

    let res = tiles.iter().enumerate().fold(0, |s: i32, (i, t)| {
        let l = (i + 1) % len;

        s + ((t.position.1 * tiles[l].position.0) as i32)
            - ((t.position.0 * tiles[l].position.1) as i32)
    });

    (res / 2) as i32
}

fn set_start_pipe(start: &Tile, grid: &Vec<Vec<Tile>>) -> Pipe {
    let neighbours: Vec<Tile> = start.get_neighbours(&grid);

    let north = neighbours
        .iter()
        .find(|n| n.position.0 < start.position.0)
        .is_some();

    let south = neighbours
        .iter()
        .find(|n| n.position.0 > start.position.0)
        .is_some();

    let west = neighbours
        .iter()
        .find(|n| n.position.1 < start.position.1)
        .is_some();

    let east = neighbours
        .iter()
        .find(|n| n.position.1 > start.position.1)
        .is_some();

    match (north, west, south, east) {
        (true, true, _, _) => Pipe::NorthWest,
        (true, _, true, _) => Pipe::NorthSouth,
        (true, _, _, true) => Pipe::NorthEast,
        (_, true, true, _) => Pipe::SouthWest,
        (_, _, true, true) => Pipe::SouthEast,
        (_, true, _, true) => Pipe::EastWest,
        _ => panic!("No valid tile to replace Start with was found"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
