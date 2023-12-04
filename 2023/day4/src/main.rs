use std::fs;
use regex::{Captures, Match, Regex};

#[derive(Debug)]
struct Card {
    index: i32,
    winning: Vec<i32>,
    yours: Vec<i32>
}

fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");

    println!("Day 4");

    let cards = process_input(contents);

    println!("Part 1: {}", part1(&cards));
    println!("Part 2: {}", part2(&cards));
}

/// Process the input and convert it to a more intelligible structure
///
/// # Arguments
/// * `content` - the content of the input file
fn process_input(content: String) -> Vec<Card> {
    let lines: Vec<&str> = content.split("\n").collect();

    let line_re: Regex = Regex::new(r"Card *(\d+): *([^|]*) *\| *([^|]*)").unwrap();
    let number_re: Regex = Regex::new(r"(\d+)").unwrap();
    let mut cards: Vec<Card> = vec![];

    for line_idx in 0..lines.len() {
        let line = lines[line_idx].to_string();
        let mut card = Card {
            index: (line_idx + 1) as i32,
            winning: vec![],
            yours: vec![],
        };

        let numbers: Captures = line_re.captures(&*line).unwrap();
        let winnings: Vec<Match> = number_re.find_iter(numbers.get(2).unwrap().as_str()).collect();
        let yours: Vec<Match> = number_re.find_iter(numbers.get(3).unwrap().as_str()).collect();

        for m in winnings {
            card.winning.push(m.as_str().parse::<i32>().unwrap())
        }

        for m in yours {
            card.yours.push(m.as_str().parse::<i32>().unwrap())
        }

        cards.push(card);
    }
    cards
}

fn part1(cards: &Vec<Card>) -> i32 {
    let mut score: i32 = 0;

    for card in cards {
        let mut valid_numbers = 0;

        for n in &card.yours {
            if card.winning.contains(&n) {
                valid_numbers += 1;
            }
        }

        if valid_numbers > 0 {
            score += 2_i32.pow(valid_numbers - 1);
        }
    }

    score
}

fn part2(cards: &Vec<Card>) -> i32 {
    let mut cards_count: Vec<i32> = vec![];
    cards.iter().for_each(|_| cards_count.push(1));

    for card in cards {
        let mut i: usize = 1;

        for n in &card.yours {
            if card.winning.contains(&n) {
                cards_count[(card.index - 1) as usize + i] += 1 * cards_count[(card.index - 1) as usize];
                i += 1;
            }
        }
    }

    cards_count.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::{part1, process_input};
    use crate::part2;

    #[test]
    fn part1_works() {
        let test_string = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string();

        assert_eq!(part1(&process_input(test_string)), 13);
    }

    #[test]
    fn part2_works() {
        let test_string = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string();

        assert_eq!(part2(&process_input(test_string)), 30);
    }
}