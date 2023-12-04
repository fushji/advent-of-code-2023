use std::collections::HashSet;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input.split('\n').collect::<Vec<&str>>();

    let mut coordinates: HashSet<[usize; 2]> = HashSet::new();

    for (r, line) in grid.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch.is_numeric() || ch == '.' {
                continue;
            }

            for i in r - 1..r + 2 {
                for j in c - 1..c + 2 {
                    if i >= grid.len()
                        || j >= grid[i].len()
                        || !(grid[i].chars().nth(j).unwrap().is_numeric())
                    {
                        continue;
                    }
                    let mut k = j;
                    while k > 0 && grid[i].chars().nth(k - 1).unwrap().is_numeric() {
                        k -= 1;
                    }

                    coordinates.insert([i, k]);
                }
            }
        }
    }

    let mut result = 0;

    for c in coordinates.iter() {
        let mut digits: Vec<String> = vec![];

        let mut col = c[1];
        while col < grid[c[0]].len() && grid[c[0]].chars().nth(col).unwrap().is_numeric() {
            digits.push(grid[c[0]].chars().nth(col).unwrap().to_string());
            col += 1
        }

        result += digits.join("").parse::<u32>().unwrap();
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input.split('\n').collect::<Vec<&str>>();
    let mut result = 0;

    for (r, line) in grid.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch != '*' {
                continue;
            }

            let mut coordinates: HashSet<[usize; 2]> = HashSet::new();

            for i in r - 1..r + 2 {
                for j in c - 1..c + 2 {
                    if i >= grid.len()
                        || j >= grid[i].len()
                        || !(grid[i].chars().nth(j).unwrap().is_numeric())
                    {
                        continue;
                    }
                    let mut k = j;
                    while k > 0 && grid[i].chars().nth(k - 1).unwrap().is_numeric() {
                        k -= 1;
                    }

                    coordinates.insert([i, k]);
                }
            }

            if coordinates.len() != 2 {
                continue;
            }

            let mut product = 1;

            for c in coordinates.iter() {
                let mut digits: Vec<String> = vec![];

                let mut col = c[1];
                while col < grid[c[0]].len() && grid[c[0]].chars().nth(col).unwrap().is_numeric() {
                    digits.push(grid[c[0]].chars().nth(col).unwrap().to_string());
                    col += 1
                }

                product *= digits.join("").parse::<u32>().unwrap();
            }

            result += product;
        }
    }

    Some(result)
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
