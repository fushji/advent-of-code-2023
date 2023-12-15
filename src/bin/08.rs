use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let commands = input.lines().next().unwrap();

    let mut network: HashMap<&str, (&str, &str)> = HashMap::new();

    input.lines().skip(2).for_each(|line| {
        let parts: Vec<&str> = line.split(" = ").collect();
        let key = parts[0];
        let value = parts[1]
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split_once(", ")
            .unwrap();
        network.insert(key, value);
    });

    let mut step = network.get("AAA").unwrap();
    let mut counter = 0;

    for command in commands.chars().cycle() {
        counter += 1;
        if command == 'L' {
            if step.0 == "ZZZ" {
                return Some(counter);
            }
            let next = network.get(step.0).unwrap();

            step = next;
        } else if command == 'R' {
            if step.1 == "ZZZ" {
                return Some(counter);
            }
            let next = network.get(step.1).unwrap();
            step = next;
        }
    }

    Some(counter)
}

fn lcm(nums: &[usize]) -> usize {
    let mut result = 1;
    for &num in nums {
        result = num * result / gcd(num, result);
    }
    result
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

pub fn part_two(input: &str) -> Option<usize> {
    let commands = input.lines().next().unwrap();

    let mut network: HashMap<&str, (&str, &str)> = HashMap::new();

    input.lines().skip(2).for_each(|line| {
        let parts: Vec<&str> = line.split(" = ").collect();
        let key = parts[0];
        let value = parts[1]
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split_once(", ")
            .unwrap();
        network.insert(key, value);
    });

    let start_nodes: Vec<_> = network.iter().filter(|x| x.0.ends_with('A')).collect();

    let mut to_end_counts = HashMap::<&str, usize>::new();

    for start in start_nodes.iter() {
        let mut current_node_name = *start.0;
        let mut count = 0;
        let mut commands_list = commands.chars().cycle();

        while !current_node_name.ends_with('Z') {
            let current_instruction = commands_list.next().unwrap();
            let current_node = network.get(current_node_name).unwrap();

            let next_node_name = match current_instruction {
                'L' => current_node.0,
                'R' => current_node.1,
                _ => unreachable!(),
            };

            current_node_name = next_node_name;
            count += 1;
        }

        to_end_counts.insert(start.0, count);
    }

    let counts = to_end_counts.values().cloned().collect::<Vec<_>>();

    let lcm = lcm(&counts);

    Some(lcm)
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
