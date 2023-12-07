use std::cmp::Ordering::{Equal, Greater, Less};

const INPUT: &str = include_str!("day07.txt");

#[derive(Debug, Clone, Copy)]
enum Card {
    Two = 0,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn cmp1(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }

    fn cmp2(&self, other: &Self) -> std::cmp::Ordering {
        match (*self, *other) {
            (Card::J, Card::J) => Equal,
            (Card::J, _) => Less,
            (_, Card::J) => Greater,
            (a, b) => a.cmp1(&b),
        }
    }
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(Self::J),
            'Q' => Ok(Self::Q),
            'K' => Ok(Self::K),
            'A' => Ok(Self::A),
            _ => Err(()),
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        *self as u8 == *other as u8
    }
}

impl Eq for Card {}

#[derive(Debug, Clone, Copy)]
enum HandKind {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl PartialEq for HandKind {
    fn eq(&self, other: &Self) -> bool {
        (*self as u8) == (*other as u8)
    }
}

impl Eq for HandKind {}

impl PartialOrd for HandKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

#[derive(Debug, Clone, Copy)]
struct Hand([Card; 5]);

impl Hand {
    fn kind1(&self) -> HandKind {
        let mut counts = [0u8; 13];
        for &card in self.0.iter() {
            counts[card as usize] += 1;
        }

        match counts.iter().max().unwrap() {
            5 => HandKind::FiveOfAKind,
            4 => HandKind::FourOfAKind,
            3 => {
                if counts.iter().any(|&count| count == 2) {
                    HandKind::FullHouse
                } else {
                    HandKind::ThreeOfAKind
                }
            }
            2 => {
                if counts.iter().filter(|&&count| count == 2).count() == 2 {
                    HandKind::TwoPair
                } else {
                    HandKind::OnePair
                }
            }
            1 => HandKind::HighCard,
            _ => unreachable!(),
        }
    }

    fn kind2(&self) -> HandKind {
        let mut counts = [0u8; 13];
        for &card in self.0.iter() {
            counts[card as usize] += 1;
        }
        let joker_count = counts[Card::J as usize];
        counts[Card::J as usize] = 0;

        match counts.iter().max().unwrap() + joker_count {
            5 => HandKind::FiveOfAKind,
            4 => HandKind::FourOfAKind,
            3 => {
                // If there is a joker, there may be 2 pairs
                if counts.iter().filter(|&&count| count >= 2).count() == 2 {
                    HandKind::FullHouse
                } else {
                    HandKind::ThreeOfAKind
                }
            }
            2 => {
                // It is impossible to get two pair with a joker, so 2 pairs are needed
                if counts.iter().filter(|&&count| count == 2).count() == 2 {
                    HandKind::TwoPair
                } else {
                    HandKind::OnePair
                }
            }
            1 => HandKind::HighCard,
            _ => unreachable!(),
        }
    }
}

fn parse_input() -> Vec<(Hand, u16)> {
    INPUT
        .lines()
        .map(|line| {
            let (hand, bet) = line.split_once(' ').unwrap();
            let mut cards = hand.chars();

            let mut hand = Hand([Card::A; 5]);
            for card in hand.0.iter_mut() {
                *card = cards.next().unwrap().try_into().unwrap();
            }
            let bet = bet.parse().unwrap();
            (hand, bet)
        })
        .collect()
}

#[allow(dead_code)]
pub fn part1() -> u32 {
    let mut input = parse_input();
    input.sort_by(|(a, _), (b, _)| cmp_hands1(a, b));
    input
        .iter()
        .enumerate()
        .map(|(i, (_, bet))| (i + 1) as u32 * *bet as u32)
        .sum()
}

fn cmp_hands1(a: &Hand, b: &Hand) -> std::cmp::Ordering {
    match a.kind1().cmp(&b.kind1()) {
        Less => Less,
        Greater => Greater,
        Equal => {
            for (a, b) in a.0.iter().zip(b.0.iter()) {
                if a != b {
                    return a.cmp1(b);
                }
            }
            Equal
        }
    }
}

#[allow(dead_code)]
pub fn part2() -> u32 {
    let mut input = parse_input();
    input.sort_by(|(a, _), (b, _)| cmp_hands2(a, b));
    input
        .iter()
        .enumerate()
        .map(|(i, (_, bet))| (i + 1) as u32 * *bet as u32)
        .sum()
}

fn cmp_hands2(a: &Hand, b: &Hand) -> std::cmp::Ordering {
    match a.kind2().cmp(&b.kind2()) {
        Less => Less,
        Greater => Greater,
        Equal => {
            for (a, b) in a.0.iter().zip(b.0.iter()) {
                if a != b {
                    return a.cmp2(b);
                }
            }
            Equal
        }
    }
}
