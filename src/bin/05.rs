advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let sections = input.split("\n\n").collect::<Vec<_>>();

    let seeds = sections[0].split(": ").collect::<Vec<_>>()[1]
        .split(' ')
        .collect::<Vec<_>>();

    let mut maps: Vec<Vec<Vec<u64>>> = Vec::new();

    for section in sections.iter().skip(1) {
        let mut map: Vec<Vec<u64>> = Vec::new();
        for line in section.lines().skip(1) {
            map.push(line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect());
        }
        maps.push(map);
    }

    let mut lowest_location: u64 = 0;
    let mut first = true;
    for seed in seeds {
        let mut src = seed.parse::<u64>().unwrap();
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

    Some(lowest_location as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sections = input.split("\n\n").collect::<Vec<_>>();

    let seeds_range = sections[0].split(": ").collect::<Vec<_>>()[1]
        .split(' ')
        .collect::<Vec<_>>();

    let mut maps: Vec<Vec<Vec<u64>>> = Vec::new();

    for section in sections.iter().skip(1) {
        let mut map: Vec<Vec<u64>> = Vec::new();
        for line in section.lines().skip(1) {
            map.push(line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect());
        }
        maps.push(map);
    }

    let mut lowest_location: u64 = 0;
    let mut first = true;
    let mut i = 0;
    let mut seeds: Vec<u64> = Vec::new();

    while i < seeds_range.len() {
        let start_seed = seeds_range[i].parse::<u64>().unwrap();
        let offset = seeds_range[i + 1].parse::<u64>().unwrap();

        for j in start_seed..start_seed + offset {
            seeds.push(j);
        }

        i += 2;
    }

    println!("{:?}", seeds.len());

    for seed in seeds {
        let mut src = seed;
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

    Some(lowest_location as u32)
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
