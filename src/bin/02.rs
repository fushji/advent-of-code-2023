use std::collections::HashMap;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let bag = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let games = input.split('\n').collect::<Vec<&str>>();
    let mut results = vec![true; games.len()];

    for (idx, game) in games.iter().enumerate() {
        if game.is_empty() {
            continue;
        }
        let rounds = game
            .split(": ")
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .split("; ")
            .collect::<Vec<&str>>();

        for round in rounds {
            let cubes = round.split(", ").collect::<Vec<&str>>();
            cubes.iter().for_each(|cube| {
                let mut parts = cube.split(' ');
                let value = parts.next().unwrap().parse::<i32>().unwrap();
                let key = parts.next().unwrap();
                //  println!("{} {} ", key, value);
                if *bag.get(key).unwrap() < value {
                    results[idx] = false;
                }
            });
        }
    }

    let mut result = 0_u32;
    for (idx, r) in results.iter().enumerate() {
        if *r {
            result += (idx + 1) as u32;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = input.split('\n').collect::<Vec<&str>>();
    let mut result = 0;

    for game in games {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        if game.is_empty() {
            continue;
        }
        let rounds = game
            .split(": ")
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .split("; ")
            .collect::<Vec<&str>>();

        for round in rounds {
            let cubes = round.split(", ").collect::<Vec<&str>>();
            cubes.iter().for_each(|cube| {
                let mut parts = cube.split(' ');
                let value = parts.next().unwrap().parse::<i32>().unwrap();
                let key = parts.next().unwrap();
                //  println!("{} {} ", key, value);
                match key {
                    "red" => {
                        if red < value {
                            red = value
                        }
                    }
                    "green" => {
                        if green < value {
                            green = value
                        }
                    }
                    "blue" => {
                        if blue < value {
                            blue = value
                        }
                    }
                    _ => {}
                }
            });
        }

        result += red * green * blue;
    }

    Some(result as u32)
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
