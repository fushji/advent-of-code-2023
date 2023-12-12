use std::vec;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum_calibration = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let mut first = false;
        let mut calibration: Vec<String> = vec!["0".to_string(), "0".to_string()];
        for c in line.chars() {
            if c.is_numeric() {
                if !first {
                    first = true;
                    calibration[0] = c.to_string();
                }
                calibration[1] = c.to_string();
            }
        }
        let calibration_num: String = calibration.join("");

        let calib: i32 = calibration_num.parse().unwrap();
        sum_calibration += calib;
    }

    Some(sum_calibration as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    const NUM: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut sum_calibration = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let mut first = false;
        let mut calibration: Vec<String> = vec!["0".to_string(), "0".to_string()];
        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if !first {
                    first = true;
                    calibration[0] = c.to_string();
                }
                calibration[1] = c.to_string();
            }

            for (x, elem) in NUM.iter().enumerate() {
                if line[i..].starts_with(elem) {
                    if !first {
                        first = true;
                        calibration[0] = (x + 1).to_string();
                    }
                    calibration[1] = (x + 1).to_string();
                }
            }
        }
        let calibration_num: String = calibration.join("");

        let calib: i32 = calibration_num.parse().unwrap();
        sum_calibration += calib;
    }

    Some(sum_calibration as u32)
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
