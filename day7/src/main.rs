use std::cmp::Ordering;

use either::Either;

christmas_tree::day!(7);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn value(char: char) -> i32 {
    match char {
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
        other => panic!("Invalid char: {}", other),
    }
}

fn values() -> impl Iterator<Item = char> {
    "23456789TJQKA".chars()
}

impl Rank {
    fn new(chars: [char; 5]) -> Self {
        let mut bundles = Vec::new();
        let mut seen = [false; 5];

        for (i, char) in chars.iter().enumerate() {
            if seen[i] {
                continue;
            }

            seen[i] = true;

            let mut count = 1;

            for (j, other) in chars.iter().enumerate().skip(i + 1) {
                if char == other {
                    seen[j] = true;
                    count += 1;
                }
            }

            bundles.push(count);
        }

        bundles.sort();

        match bundles.as_slice() {
            [1, 1, 1, 1, 1] => Rank::HighCard,
            [1, 1, 1, 2] => Rank::OnePair,
            [1, 2, 2] => Rank::TwoPair,
            [1, 1, 3] => Rank::ThreeOfAKind,
            [2, 3] => Rank::FullHouse,
            [1, 4] => Rank::FourOfAKind,
            [5] => Rank::FiveOfAKind,
            other => panic!("Invalid: {other:?}, input: {chars:?}"),
        }
    }

    fn part1(input: &str) -> Self {
        let mut chars_iter = input.chars();
        let chars = [
            chars_iter.next().unwrap(),
            chars_iter.next().unwrap(),
            chars_iter.next().unwrap(),
            chars_iter.next().unwrap(),
            chars_iter.next().unwrap(),
        ];

        Self::new(chars)
    }

    fn max_bundle_size(self) -> i32 {
        match self {
            Rank::HighCard => 1,
            Rank::OnePair => 2,
            Rank::TwoPair => 2,
            Rank::ThreeOfAKind => 3,
            Rank::FullHouse => 3,
            Rank::FourOfAKind => 4,
            Rank::FiveOfAKind => 5,
        }
    }

    fn from_single_bundle(size: i32) -> Self {
        match size {
            1 => Rank::HighCard,
            2 => Rank::OnePair,
            3 => Rank::FullHouse,
            4 => Rank::FourOfAKind,
            5 => Rank::FiveOfAKind,
            other => panic!("Invalid bundle size: {}", other),
        }
    }

    fn part2(input: &str) -> Self {
        let mut chars_iter = input.chars();
        let chars = [
            chars_iter.next().unwrap(),
            chars_iter.next().unwrap(),
            chars_iter.next().unwrap(),
            chars_iter.next().unwrap(),
            chars_iter.next().unwrap(),
        ];

        // let jokers = chars.iter().filter(|&&c| c == 'J').count();

        let mut best_rank = Rank::HighCard;
        for c0 in iter_chars(chars[0]) {
            for c1 in iter_chars(chars[1]) {
                for c2 in iter_chars(chars[2]) {
                    for c3 in iter_chars(chars[3]) {
                        for c4 in iter_chars(chars[4]) {
                            let rank = Self::new([c0, c1, c2, c3, c4]);
                            if rank > best_rank {
                                best_rank = rank;
                            }
                        }
                    }
                }
            }
        }

        best_rank
    }
}

fn iter_chars(c: char) -> impl Iterator<Item = char> {
    match c {
        'J' => Either::Left(values()),
        other => Either::Right(std::iter::once(other)),
    }
}

fn value_part2(char: char) -> i32 {
    match char {
        'J' => -1,
        other => value(other),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Bid<'a> {
    rank: Rank,
    hand: &'a str,
    amount: i64,
}

impl PartialOrd for Bid<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Bid<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank).then_with(|| {
            self.hand
                .chars()
                .zip(other.hand.chars())
                .map(|(a, b)| value(a).cmp(&value(b)))
                .find(|&ord| ord != Ordering::Equal)
                .unwrap_or(Ordering::Equal)
        })
    }
}

impl Bid<'_> {
    fn cmp_joker(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank).then_with(|| {
            self.hand
                .chars()
                .zip(other.hand.chars())
                .map(|(a, b)| value_part2(a).cmp(&value_part2(b)))
                .find(|&ord| ord != Ordering::Equal)
                .unwrap_or(Ordering::Equal)
        })
    }
}

fn part1(input: &str) -> i64 {
    let mut bids = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let hand = parts.next().unwrap();
            let rank = Rank::part1(hand);
            let amount = parts.next().unwrap().parse().unwrap();
            Bid { rank, amount, hand }
        })
        .collect::<Vec<_>>();

    bids.sort();
    bids.iter()
        .enumerate()
        .map(|(i, bid)| (i as i64 + 1) * bid.amount)
        .sum()
}

fn part2(input: &str) -> i64 {
    let mut bids = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let hand = parts.next().unwrap();
            let rank = Rank::part2(hand);
            let amount = parts.next().unwrap().parse().unwrap();
            Bid { rank, amount, hand }
        })
        .collect::<Vec<_>>();

    bids.sort_by(|a, b| a.cmp_joker(b));
    dbg!(&bids);
    bids.iter()
        .enumerate()
        .map(|(i, bid)| (i as i64 + 1) * bid.amount)
        .sum()
}

christmas_tree::examples! {
    r"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    " => 6440, 5905,
}
#[test]
fn caca() {
    let h1 = "QJJQ2";
    
    let r1 = Rank::part2(&h1);
}
