use std::cmp::{max, min, Ordering};
use std::fs;
use regex::{Regex};

const CARDS: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
const CARDS_part2: [char; 13] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

#[derive(Debug)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard
}
impl From<&HandType> for u8 {
    fn from(c: &HandType) -> Self {
        match c {
            HandType::FiveOfKind => 6,
            HandType::FourOfKind => 5,
            HandType::FullHouse => 4,
            HandType::ThreeOfKind => 3,
            HandType::TwoPair => 2,
            HandType::OnePair => 1,
            HandType::HighCard => 0
        }
    }
}

#[derive(Debug)]
struct Hand {
    value: String,
    letters: [i32; 5],
    hand_type: HandType,
    bid: u64
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if u8::from(&self.hand_type) < u8::from(&other.hand_type) {
            return Ordering::Less
        } else if u8::from(&self.hand_type) > u8::from(&other.hand_type) {
            return Ordering::Greater
        }

        for i in 0..5 {
            if self.letters[i] > other.letters[i] {
                return Ordering::Less
            } else if self.letters[i] < other.letters[i] {
                return Ordering::Greater
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.value.eq(&other.value);
    }
}

fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");
    println!("Day 7");

    let data = process_input(&contents, false);
    println!("Part 1: {}", part1(&data));

    let data = process_input(&contents, true);
    println!("Part 2: {}", part2(&data));
}

fn count_cards(value: &str, part2: bool) -> (HandType, [i32; 5]) {
    let mut count: [i32; 13] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut letters: [i32; 5] = [-1, -1, -1, -1, -1];


    for value_idx in 0..value.len() {
        let c = value.as_bytes()[value_idx] as char;
        for (i, card) in (if part2 { CARDS_part2 } else { CARDS }).iter().enumerate() {
            if c == *card {
                count[i] += 1;

                letters[value_idx] = i as i32;
            }
        }
    }

    let mut n_same_max = 0;
    let mut n_same_min = 100;

    for (idx, n) in count.iter().enumerate() {
        if *n > 0 && (!part2 || idx < 12){
            n_same_max = max(n_same_max, *n);
            n_same_min = min(n_same_min, *n);
        }
    }

    // add joker to the max (then translate to the card we have the most)
    if part2 {
        n_same_max += count[12];
    }

    let mut hand_type = HandType::HighCard;

    if n_same_max == 5 {
        hand_type = HandType::FiveOfKind;
    } else if n_same_max == 4 {
        hand_type = HandType::FourOfKind;
    } else if n_same_max == 3 && n_same_min == 2 {
        hand_type = HandType::FullHouse;
    } else if n_same_max == 3 {
        hand_type = HandType::ThreeOfKind;
    } else if n_same_max == 2 {
        let mut count_pairs = 0;

        for n in count {
            if n == 2 {
                count_pairs += 1;
            }
        }

        hand_type = if count_pairs == 2 {
            HandType::TwoPair
        } else {
            HandType::OnePair
        }
    }

    return (hand_type, letters);
}

/// Process the input and convert it to a more intelligible structure
///
/// # Arguments
/// * `content` - the content of the input file
fn process_input(content: &String, part2: bool) -> Vec<Hand> {
    let lines: Vec<&str> = content.split("\n").collect();

    let line_re: Regex = Regex::new(r"([AKQJT0-9]+) (\d+)").unwrap();

    let mut hands: Vec<Hand> = vec![];

    for i in 0..lines.len() {
        let result = line_re.captures(lines[i]).unwrap();

        let value = result.get(1).unwrap().as_str().to_string();
        let (hand_type, letters) = count_cards(&value, part2);

        hands.push(Hand {
            value,
            letters,
            hand_type,
            bid: result.get(2).unwrap().as_str().parse::<u64>().unwrap()
        });
    }

    hands.sort();

    hands
}

fn part1(data: &Vec<Hand>) -> i32 {
    let mut r = 0;

    for (i, d) in data.iter().enumerate() {
        println!("{}: {:?}", i + 1, d);
        r += (i + 1) as i32 * d.bid as i32;
    }

    r
}

fn part2(data: &Vec<Hand>) -> i32 {
    part1(data)
}

#[cfg(test)]
mod tests {
    use crate::{part1, process_input};
    use crate::part2;

    #[test]
    fn part1_works() {
        let test_string = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483".to_string();

        assert_eq!(part1(&process_input(&test_string, false)), 6440);
    }

    #[test]
    fn part1_works2() {
        let test_string = "AAAAA 2
22222 3
AAAAK 5
22223 7
AAAKK 11
22233 13
AAAKQ 17
22234 19
AAKKQ 23
22334 29
AAKQJ 31
22345 37
AKQJT 41
23456 43".to_string();

        assert_eq!(part1(&process_input(&test_string, false)), 1343);
    }

    #[test]
    fn part2_works() {
        let test_string = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483".to_string();

        assert_eq!(part2(&process_input(&test_string, true)), 5905);
    }
}