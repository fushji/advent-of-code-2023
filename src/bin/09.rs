advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let histories = input
        .lines()
        .map(|line| {
            let values = line
                .split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            values
        })
        .collect::<Vec<_>>();
    let mut sum = 0;

    for history in histories {
        let mut current = history.clone();
        let mut last: Vec<i32> = Vec::new();
        loop {
            if current.iter().sum::<i32>() == 0 {
                last.push(current.last().unwrap().clone());
                break;
            }

            last.push(current.last().unwrap().clone());

            let mut app: Vec<i32> = Vec::new();
            for i in 0..current.len() - 1 {
                app.push(current[i + 1] - current[i])
            }

            current = app;
        }
        last.reverse();
        for i in 1..last.len() {
            last[i] += last[i - 1];
        }
        sum += last.last().unwrap();
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let histories = input
        .lines()
        .map(|line| {
            let values = line
                .split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            values
        })
        .collect::<Vec<_>>();
    let mut sum = 0;

    for history in histories {
        let mut current = history.clone();
        let mut first: Vec<i32> = Vec::new();
        loop {
            if current.iter().sum::<i32>() == 0 {
                first.push(current.last().unwrap().clone());
                break;
            }

            first.push(current.first().unwrap().clone());

            let mut app: Vec<i32> = Vec::new();
            for i in 0..current.len() - 1 {
                app.push(current[i + 1] - current[i])
            }

            current = app;
        }
        first.reverse();

        for i in 1..first.len() {
            first[i] -= first[i - 1];
        }

        sum += first.last().unwrap();
    }

    Some(sum)
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
