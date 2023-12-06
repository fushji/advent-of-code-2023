advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split('\n').collect::<Vec<_>>();
    let times = lines[0].split(':').collect::<Vec<_>>()[1]
        .split(' ')
        .filter(|c| !c.is_empty())
        .collect::<Vec<_>>();
    let distances = lines[1].split(':').collect::<Vec<_>>()[1]
        .split(' ')
        .filter(|c| !c.is_empty())
        .collect::<Vec<_>>();

    let mut total = 1;

    for i in 0..times.len() {
        let mut winning = 0;

        let time = times[i].parse::<u32>().unwrap();

        for j in 0..=time {
            let distance = j * (time - j);
            if distance > distances[i].parse::<u32>().unwrap() {
                winning += 1;
            }
        }

        total *= winning;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split('\n').collect::<Vec<_>>();
    let time = lines[0].split(':').collect::<Vec<_>>()[1]
        .split(' ')
        .filter(|c| !c.is_empty())
        .collect::<Vec<_>>()
        .join("")
        .parse::<f64>()
        .unwrap();
    let distance = lines[1].split(':').collect::<Vec<_>>()[1]
        .split(' ')
        .filter(|c| !c.is_empty())
        .collect::<Vec<_>>()
        .join("")
        .parse::<f64>()
        .unwrap();

    let min = (time - (time.powf(2.0) - 4.0 * distance).sqrt()) / 2.0;
    let max = (time + (time.powf(2.0) - 4.0 * distance).sqrt()) / 2.0;

    Some((max - min + 1.0) as u32)
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
