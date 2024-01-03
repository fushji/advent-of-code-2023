advent_of_code::solution!(11);

pub fn build_galaxies(input: &str) -> Vec<(usize, usize)> {
    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    for (i, row) in input.lines().enumerate() {
        for (j, col) in row.chars().enumerate() {
            if col.eq(&'#') {
                galaxies.push((i, j))
            }
        }
    }
    galaxies
}

fn extract_empty_rows(mut galaxies: Vec<(usize, usize)>) -> Vec<usize> {
    let mut empty_rows: Vec<usize> = Vec::new();

    galaxies.sort_by(|a, b| a.0.cmp(&b.0));

    for i in 0..galaxies.len() - 1 {
        if galaxies[i + 1].0 - galaxies[i].0 > 1 {
            for j in galaxies[i].0 + 1..galaxies[i + 1].0 {
                empty_rows.push(j);
            }
        }
    }

    empty_rows
}

fn extract_empty_cols(mut galaxies: Vec<(usize, usize)>) -> Vec<usize> {
    let mut empty_cols: Vec<usize> = Vec::new();

    galaxies.sort_by(|a, b| a.1.cmp(&b.1));

    for i in 0..galaxies.len() - 1 {
        if galaxies[i + 1].1 - galaxies[i].1 > 1 {
            for j in galaxies[i].1 + 1..galaxies[i + 1].1 {
                empty_cols.push(j);
            }
        }
    }

    empty_cols
}

fn expand_galaxies(
    galaxies: &mut [(usize, usize)],
    empty_rows: &[usize],
    empty_cols: &[usize],
    rate: i32,
) {
    for galaxy in galaxies {
        let num_step_x = empty_rows.iter().filter(|r| **r < galaxy.0).count() as i32;
        let num_step_y = empty_cols.iter().filter(|c| **c < galaxy.1).count() as i32;

        galaxy.0 += (num_step_x * (rate - 1)) as usize;
        galaxy.1 += (num_step_y * (rate - 1)) as usize;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut galaxies: Vec<(usize, usize)> = build_galaxies(input);
    let empty_rows: Vec<usize> = extract_empty_rows(galaxies.clone());
    let empty_cols: Vec<usize> = extract_empty_cols(galaxies.clone());

    expand_galaxies(&mut galaxies, &empty_rows, &empty_cols, 2);

    let mut distances_sum: u32 = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let distance = (galaxies[i].0 as i32 - galaxies[j].0 as i32).abs()
                + (galaxies[i].1 as i32 - galaxies[j].1 as i32).abs();

            distances_sum += distance as u32;
        }
    }

    Some(distances_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut galaxies: Vec<(usize, usize)> = build_galaxies(input);
    let empty_rows: Vec<usize> = extract_empty_rows(galaxies.clone());
    let empty_cols: Vec<usize> = extract_empty_cols(galaxies.clone());

    expand_galaxies(&mut galaxies, &empty_rows, &empty_cols, 1_000_000);

    let mut distances_sum: u64 = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let distance = (galaxies[i].0 as i32 - galaxies[j].0 as i32).abs()
                + (galaxies[i].1 as i32 - galaxies[j].1 as i32).abs();

            distances_sum += distance as u64;
        }
    }

    Some(distances_sum)
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
