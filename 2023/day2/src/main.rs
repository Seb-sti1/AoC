use std::cmp::max;
use std::fs;
use regex::{Captures, Regex};
use std::str::FromStr;

enum Color {
    RED = 0,
    GREEN = 1,
    BLUE = 2,
}

impl FromStr for Color {
    type Err = ();
    fn from_str(input: &str) -> Result<Color, Self::Err> {
        match input {
            "red"  => Ok(Color::RED),
            "green"  => Ok(Color::GREEN),
            "blue"  => Ok(Color::BLUE),
            _      => Err(()),
        }
    }
}

struct Hand {
    colors: [i32; 3]
}

struct Game {
    index: i32,
    hands: Vec<Hand>
}

fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");

    println!("Day 1");

    let games: Vec<Game> = process_input(contents);

    println!("Part 1: {}", part1(&games));
    println!("Part 2: {}", part2(&games));
}

fn process_input(contents: String) -> Vec<Game> {
    let lines: Vec<&str> = contents.split("\n").collect();

    let game_re: Regex = Regex::new(r"Game \d*:(.*)").unwrap();
    let draw_re: Regex = Regex::new(r"([^;]*)").unwrap();
    let color_re: Regex = Regex::new(r" (\d*) ([a-z]*)").unwrap();

    let mut games: Vec<Game> = vec![];

    for i in 0..lines.len() {
        let mut game: Game = Game { index: -1, hands: vec![] };
        game.index = (i + 1) as i32;

        let game_str = game_re.find(lines[i]).unwrap().as_str();
        let hands_str: Vec<&str> = draw_re.find_iter(game_str).map(|m| m.as_str()).collect();

        for hand_str in hands_str {
            let mut hand: Hand = Hand { colors: [0, 0, 0]};
            let colors: Vec<Captures> = color_re.captures_iter(hand_str).collect();

            for color in colors {
                hand.colors[Color::from_str(color.get(2).unwrap().as_str()).unwrap() as usize] += color.get(1).unwrap().as_str().parse::<i32>().unwrap();
            }

            game.hands.push(hand);
        }

        games.push(game);
    }
    games
}

fn part1(games: &Vec<Game>) -> i32 {
    let max_cubes= [12, 13, 14];
    let mut result = 0;

    for game in games {
        let mut valid = true;

        'hand: for hand in &game.hands {
            if hand.colors[0] > max_cubes[0] {
                valid = false;
                break 'hand;
            }

            if hand.colors[1] > max_cubes[1] {
                valid = false;
                break 'hand;
            }

            if hand.colors[2] > max_cubes[2] {
                valid = false;
                break 'hand;
            }
        }

        if valid {
            result += game.index;
        }
    }

    result
}

fn part2(games: &Vec<Game>) -> i32 {
    let mut result = 0;

    for game in games {

        let mut min_hand: Hand = Hand {colors: [0, 0, 0]};

        for hand in &game.hands {
            min_hand.colors[0] = max(min_hand.colors[0], hand.colors[0]);
            min_hand.colors[1] = max(min_hand.colors[1], hand.colors[1]);
            min_hand.colors[2] = max(min_hand.colors[2], hand.colors[2]);
        }

        result += min_hand.colors[0] * min_hand.colors[1] * min_hand.colors[2];
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{part1, process_input};
    use crate::part2;

    #[test]
    fn part1_works() {
        let test_string = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string();
        let games = process_input(test_string);

        assert_eq!(part1(&games), 8);
    }

    #[test]
    fn part2_works() {
        let test_string = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string();
        let games = process_input(test_string);

        assert_eq!(part2(&games), 2286);
    }
}