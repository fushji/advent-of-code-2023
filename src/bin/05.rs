use std::cmp;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<i64> {
    let sections = input.split("\n\n").collect::<Vec<_>>();

    let seeds = sections[0].split(": ").collect::<Vec<_>>()[1]
        .split(' ')
        .collect::<Vec<_>>();

    let mut maps: Vec<Vec<Vec<i64>>> = Vec::new();

    for section in sections.iter().skip(1) {
        let mut map: Vec<Vec<i64>> = Vec::new();
        for line in section.lines().skip(1) {
            map.push(line.split(' ').map(|x| x.parse::<i64>().unwrap()).collect());
        }
        maps.push(map);
    }

    let mut lowest_location: i64 = 0;
    let mut first = true;
    for seed in seeds {
        let mut src = seed.parse::<i64>().unwrap();
        for map in maps.iter() {
            for row in map {
                if src >= row[1] && src <= row[1] + row[2] {
                    src = row[0] + (src - row[1]);
                    break;
                }
            }
        }

        if first {
            lowest_location = src;
            first = false;
        }

        if src < lowest_location {
            lowest_location = src;
        }
    }

    Some(lowest_location as i64)
}

pub fn part_two(input: &str) -> Option<i64> {
    let sections = input.split("\n\n").collect::<Vec<_>>();

    let mut seed_ranges = sections[0].split(": ").collect::<Vec<_>>()[1]
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1]))
        .collect::<Vec<_>>();

    let mut maps: Vec<Vec<Vec<i64>>> = Vec::new();

    for section in sections.iter().skip(1) {
        let mut map: Vec<Vec<i64>> = Vec::new();
        for line in section.lines().skip(1) {
            map.push(line.split(' ').map(|x| x.parse::<i64>().unwrap()).collect());
        }

        maps.push(map);
    }

    for map in maps {
        let mut ranges: Vec<(i64, i64)> = Vec::new();

        while !seed_ranges.is_empty() {
            let mut overlap = false;
            let seed = seed_ranges.pop().unwrap();
            for row in map.iter() {
                let overlap_start = cmp::max(seed.0, row[1]);
                let overlap_end = cmp::min(seed.1, row[1] + row[2]);
                if overlap_start < overlap_end {
                    ranges.push((
                        overlap_start - row[1] + row[0],
                        overlap_end - row[1] + row[0],
                    ));
                    if overlap_start > seed.0 {
                        seed_ranges.push((seed.0, overlap_start));
                    }
                    if seed.1 > overlap_end {
                        seed_ranges.push((overlap_end, seed.1));
                    }
                    overlap = true;
                    break;
                }
            }
            if !overlap {
                ranges.push(seed);
            }
        }
        seed_ranges = ranges;
    }

    let lowest_location: i64 = seed_ranges.iter().map(|x: &(i64, i64)| x.0).min().unwrap();
    Some(lowest_location)
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
