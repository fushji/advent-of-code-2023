use std::cmp::Ordering;

use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

fn card_to_rank(card: char) -> usize {
    match card {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("bad"),
    }
}

fn card_to_rank_2(card: char) -> usize {
    match card {
        'J' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("bad"),
    }
}

fn get_hand_type(hand: Hand) -> HandType {
    let mut sorted_hand = hand.ranks;
    sorted_hand.sort_by(|a, b| b.cmp(a));
    match sorted_hand[0] {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => {
            if sorted_hand[1] == 2 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        }

        2 => {
            if sorted_hand[1] == 2 {
                HandType::TwoPairs
            } else {
                HandType::OnePair
            }
        }
        1 | 0 => HandType::HighCard,

        _ => panic!("Invalid Hand"),
    }
}

fn get_hand_type_2(hand: Hand2) -> HandType {
    let mut sorted_hand = hand.ranks;
    sorted_hand.sort_by(|a, b| b.cmp(a));
    match sorted_hand[0] {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => {
            if sorted_hand[1] == 2 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        }

        2 => {
            if sorted_hand[1] == 2 {
                HandType::TwoPairs
            } else {
                HandType::OnePair
            }
        }
        1 | 0 => HandType::HighCard,

        _ => panic!("Invalid Hand"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand {
    cards: [usize; 5],
    score: u32,
    ranks: [u8; 13],
}

impl Hand {
    fn new(hand: &str, score: u32) -> Self {
        let mut ranks = [0; 13];
        let mut cards = [0; 5];

        for (i, card) in hand.chars().enumerate() {
            ranks[card_to_rank(card)] += 1;
            cards[i] = card_to_rank(card);
        }

        Self {
            cards,
            score,
            ranks,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match get_hand_type(*self).cmp(&get_hand_type(*other)) {
            Ordering::Equal => Some(self.cards.cmp(&other.cards).reverse()),
            ord => Some(ord),
        }
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand2 {
    cards: [usize; 5],
    score: u32,
    ranks: [u8; 13],
}

impl Hand2 {
    fn new(hand: &str, score: u32) -> Self {
        let mut ranks = [0; 13];
        let mut cards = [0; 5];

        for (i, card) in hand.chars().enumerate() {
            ranks[card_to_rank_2(card)] += 1;
            cards[i] = card_to_rank_2(card);
        }

        Self {
            cards,
            score,
            ranks,
        }
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match get_hand_type_2(*self).cmp(&get_hand_type_2(*other)) {
            Ordering::Equal => Some(self.cards.cmp(&other.cards).reverse()),
            ord => Some(ord),
        }
    }
}
impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let (hand, score) = line.split(' ').collect_tuple().unwrap();
            Hand::new(hand, score.parse().unwrap())
        })
        .collect();

    hands.sort_by(|h1, h2| h2.cmp(h1));
    let score: u32 = hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx as u32 + 1) * hand.score)
        .sum();

    Some(score)
}

fn apply_j(hand: &mut Hand2) {
    let j = hand.ranks[0];

    match get_hand_type_2(*hand) {
        HandType::FiveOfAKind => {
            hand.ranks[0] = 0;
            hand.ranks[12] = 5;
        }
        HandType::FourOfAKind => {
            let mut max_rank_index = 1;
            for i in 1..13 {
                if hand.ranks[i] >= hand.ranks[max_rank_index] {
                    max_rank_index = i
                }
            }
            hand.ranks[0] = 0;
            hand.ranks[max_rank_index] = 5;
        }
        HandType::FullHouse => {
            let mut max_rank_index = 1;
            for i in 1..13 {
                if hand.ranks[i] >= hand.ranks[max_rank_index] {
                    max_rank_index = i
                }
            }
            hand.ranks[0] = 0;
            hand.ranks[max_rank_index] += j;
        }
        HandType::ThreeOfAKind => {
            let mut max_rank_index = 1;
            for i in 1..13 {
                if hand.ranks[i] >= hand.ranks[max_rank_index] {
                    max_rank_index = i
                }
            }
            hand.ranks[0] = 0;
            hand.ranks[max_rank_index] += j;
        }
        HandType::TwoPairs => {
            let mut max_rank_index = 1;
            for i in 1..13 {
                if hand.ranks[i] >= hand.ranks[max_rank_index] {
                    max_rank_index = i
                }
            }
            hand.ranks[0] = 0;
            hand.ranks[max_rank_index] += j;
        }
        HandType::OnePair => {
            let mut max_rank_index = 1;
            for i in 1..13 {
                if hand.ranks[i] >= hand.ranks[max_rank_index] {
                    max_rank_index = i
                }
            }

            hand.ranks[0] = 0;
            hand.ranks[max_rank_index] += j;
        }
        HandType::HighCard => {
            let mut max_rank_index = 1;
            for i in 1..13 {
                if hand.ranks[i] >= hand.ranks[max_rank_index] {
                    max_rank_index = i
                }
            }
            hand.ranks[0] = 0;
            hand.ranks[max_rank_index] += j;
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand2> = input
        .lines()
        .map(|line| {
            let (hand, score) = line.split(' ').collect_tuple().unwrap();
            Hand2::new(hand, score.parse().unwrap())
        })
        .collect();

    hands.iter_mut().for_each(|h| {
        if h.ranks[0] > 0 {
            apply_j(h)
        }
    });

    hands.sort_by(|h1, h2| h2.cmp(h1));
    let score: u32 = hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx as u32 + 1) * hand.score)
        .sum();

    Some(score)
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
