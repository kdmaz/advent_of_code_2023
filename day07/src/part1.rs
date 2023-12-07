use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
};

pub fn part1(input: &str) -> i32 {
    let ranks = input
        .lines()
        .map(|line| {
            let mut line_parts = line.split(' ');
            let mut cards_it = line_parts
                .next()
                .expect("Expect cards to exist in line")
                .chars()
                .map(|c| CardType::try_from(c).expect("Card to be valid CardType"));
            let cards = [
                cards_it.next().expect("Card 1 to exist"),
                cards_it.next().expect("Card 2 to exist"),
                cards_it.next().expect("Card 3 to exist"),
                cards_it.next().expect("Card 4 to exist"),
                cards_it.next().expect("Card 5 to exist"),
            ];
            let bid = line_parts
                .next()
                .expect("Bid to exist in line")
                .parse::<i32>()
                .expect("Bid to be a valid integer");
            Hand::new(cards, bid)
        })
        .fold(BTreeSet::new(), |mut ranks, hand| {
            ranks.insert(hand);
            ranks
        });

    ranks
        .into_iter()
        .enumerate()
        .map(|(i, rank)| rank.bid * (i as i32 + 1))
        .sum()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
enum CardType {
    A = 13,
    K = 12,
    Q = 11,
    J = 10,
    T = 9,
    Nine = 8,
    Eight = 7,
    Seven = 6,
    Six = 5,
    Five = 4,
    Four = 3,
    Three = 2,
    Two = 1,
}

impl TryFrom<char> for CardType {
    type Error = ();

    fn try_from(card: char) -> Result<Self, Self::Error> {
        Ok(match card {
            'A' => CardType::A,
            'K' => CardType::K,
            'Q' => CardType::Q,
            'J' => CardType::J,
            'T' => CardType::T,
            '9' => CardType::Nine,
            '8' => CardType::Eight,
            '7' => CardType::Seven,
            '6' => CardType::Six,
            '5' => CardType::Five,
            '4' => CardType::Four,
            '3' => CardType::Three,
            '2' => CardType::Two,
            _ => return Err(()),
        })
    }
}

type Cards = [CardType; 5];

#[derive(Debug)]
struct Hand {
    bid: i32,
    hand_type: HandType,
    cards: Cards,
}

impl Hand {
    fn new(cards: Cards, bid: i32) -> Self {
        let hand_type = HandType::from(&cards);
        Self {
            bid,
            hand_type,
            cards,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}
impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                for (&c1, c2) in self.cards.iter().zip(other.cards) {
                    if c1 == c2 {
                        continue;
                    }

                    return c1.cmp(&c2);
                }

                Ordering::Equal
            }
            ord => ord,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl From<&Cards> for HandType {
    fn from(cards: &Cards) -> Self {
        let mut map = HashMap::new();

        for &card in cards {
            *map.entry(card).or_insert(0) += 1;
        }

        match map.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                let has_four = map.iter().any(|(_, &count)| count == 4);
                if has_four {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                let has_three = map.iter().any(|(_, &count)| count == 3);
                if has_three {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../example.txt");
        let output = part1(input);
        let expected = 6440;
        assert_eq!(output, expected);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("../input.txt");
        let output = part1(input);
        let expected = 247815719;
        assert_eq!(output, expected);
    }
}
