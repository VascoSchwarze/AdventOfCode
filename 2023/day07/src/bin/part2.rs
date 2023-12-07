use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

#[derive(Debug, Clone, Copy)]
struct Card {
    char: char,
}

impl Card {
    fn value(&self) -> Option<u32> {
        match self.char {
            'A' => Some(14),
            'K' => Some(13),
            'Q' => Some(12),
            'T' => Some(10),
            '9' => Some(9),
            '8' => Some(8),
            '7' => Some(7),
            '6' => Some(6),
            '5' => Some(5),
            '4' => Some(4),
            '3' => Some(3),
            '2' => Some(2),
            'J' => Some(1),
            _ => None,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_value = self.value();
        let other_value = other.value();

        self_value.partial_cmp(&other_value)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.char == other.char
    }
}

#[derive(PartialEq, Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_value = match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        };

        let other_value = match other {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        };

        self_value.partial_cmp(&other_value)
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u64,
}

impl Hand {
    fn get_type(cards: &[Card]) -> HandType {
        if cards.iter().any(|c| c.char == 'J') {
            let mut possible_types: Vec<HandType> = vec![];
            for c in ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2'] {
                let idx = cards.iter().position(|c| c.char == 'J').unwrap();
                let mut new_cards = cards.to_owned();
                new_cards[idx] = Card { char: c };
                possible_types.push(Hand::get_type(&new_cards));
            }
            possible_types.sort_by(|t1, t2| {
                if t1 > t2 {
                    return Ordering::Greater;
                }

                if t2 > t1 {
                    return Ordering::Less;
                }
                Ordering::Equal
            });
            return possible_types.last().unwrap().clone();
        }

        let card_map: HashMap<char, u32> =
            cards
                .iter()
                .map(|c| c.char)
                .fold(HashMap::new(), |mut map, char| {
                    map.entry(char)
                        .and_modify(|e| {
                            *e += 1;
                        })
                        .or_insert(1);
                    map
                });
        let card_count_set = card_map.values().copied().collect::<HashSet<_>>();

        match card_map.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if card_count_set.contains(&1) || card_count_set.contains(&4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if card_count_set.contains(&3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_type = Hand::get_type(&self.cards);
        let other_type = Hand::get_type(&other.cards);
        if self_type > other_type {
            return Some(Ordering::Greater);
        }

        if other_type > self_type {
            return Some(Ordering::Less);
        }

        self.cards.partial_cmp(&other.cards)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let cards = line.split(' ').next().unwrap();
            let bid = line
                .split(' ')
                .nth(1)
                .unwrap()
                .parse::<u64>()
                .expect("Should be a number");
            Hand {
                bid,
                cards: cards.chars().map(|c| Card { char: c }).collect(),
            }
        })
        .collect();

    hands.sort_by(|h1, h2| {
        if h1 > h2 {
            return Ordering::Greater;
        }
        if h2 > h1 {
            return Ordering::Less;
        }
        Ordering::Equal
    });

    let mut sum: u64 = 0;
    for (idx, hand) in hands.iter().enumerate() {
        sum += (u64::try_from(idx + 1).unwrap()) * hand.bid;
    }

    println!("{sum}");
}
