advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let cards = input.split('\n').collect::<Vec<&str>>();
    let mut total = 0;
    for card in cards {
        let mut points = 0;
        let (_, cards_line) = card.split_once(": ").unwrap();
        let (winning_cards, my_cards) = cards_line.split_once(" | ").unwrap();

        let winning = winning_cards
            .split(' ')
            .filter(|&x| x.ne(""))
            .collect::<Vec<&str>>();

        let mine: Vec<&str> = my_cards
            .split(' ')
            .filter(|&x| x.ne(""))
            .collect::<Vec<&str>>();

        for card in mine {
            if winning.contains(&card) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }
        total += points;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = input.split('\n').collect::<Vec<&str>>();
    let mut n_winning = vec![0u32; cards.len()];
    let mut copies = vec![1u32; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let (_, cards_line) = card.split_once(": ").unwrap();
        let (winning_cards, my_cards) = cards_line.split_once(" | ").unwrap();

        let winning = winning_cards
            .split(' ')
            .filter(|&x| x.ne(""))
            .collect::<Vec<&str>>();

        let mine: Vec<&str> = my_cards
            .split(' ')
            .filter(|&x| x.ne(""))
            .collect::<Vec<&str>>();

        for card in mine {
            if winning.contains(&card) {
                n_winning[i] += 1;
            }
        }
    }

    for i in 0..n_winning.len() {
        for j in i + 1..=i + n_winning[i] as usize {
            copies[j] += copies[i] as u32;
        }
    }

    Some(copies.iter().sum())
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
