use std::cmp::{max, min};
use std::fs;
use regex::{ Match, Regex};

#[derive(Debug)]
struct Symbol {
    char: String,
    position: [usize; 2]
}

#[derive(Debug)]
struct Part {
    number: i32,
    symbol: Symbol
}


fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");

    println!("Day 3");

    let games = process_input(contents);

    println!("Part 1: {}", part1(&games));
    println!("Part 2: {}", part2(&games));
}

/// Return symbol (a character being neither a '.' or a number), if any, contained in the `line[start..end + 1]`
///
/// # Arguments
/// * `line` - The line from the input to search for a symbol
/// * `start` - The start of the substring
/// * `end` - The end (excluded) of the substring
fn find_symbol(line: String, start: usize, end: usize, line_idx: usize) -> Option<Symbol> {
    let re: Regex = Regex::new(r"[^\\.0-9]").unwrap();

    let substring = line[start..end].to_string();
    let symbol = re.find(&*substring);

    if symbol.is_some() {
        let symbol = symbol.unwrap();
        return Option::from(Symbol { char: symbol.as_str().to_string(), position: [line_idx, start + symbol.start()] })
    }

    None
}

/// Process the input and convert it to a more intelligible structure
///
/// # Arguments
/// * `content` - the content of the input file
fn process_input(content: String) -> Vec<Part> {
    let lines: Vec<&str> = content.split("\n").collect();

    let number_re: Regex = Regex::new(r"(\d+)").unwrap();
    let mut parts: Vec<Part> = vec![];

    for line_idx in 0..lines.len() {
        let line = lines[line_idx].to_string();
        let numbers: Vec<Match> = number_re.find_iter(&*line).collect();

        for number in numbers {
            let n = number.as_str().parse::<i32>().unwrap();
            let mut symbol: Option<Symbol> = None;

            for j in -1i32..2i32 {
                if 0 <= line_idx as i32 + j && line_idx as i32 + j < lines.len() as i32 {
                    symbol = find_symbol(lines[(line_idx as i32 + j) as usize].to_string(),
                                             max(number.start() as i32 - 1, 0) as usize,
                                             min(number.end() + 1, line.len() - 1),
                                          (line_idx as i32 + j) as usize);
                    if symbol.is_some() {
                        break;
                    }
                }
            }

            if symbol.is_some() {
               parts.push(Part {number: n, symbol: symbol.unwrap()});
            }
        }
    }
    parts
}

fn part1(parts: &Vec<Part>) -> i32 {
    let result = parts.iter().map(|p| p.number).sum();

    result
}

fn part2(parts: &Vec<Part>) -> i32 {
    let mut result = 0;
    let gears: Vec<&Part> = parts.iter().filter(|p| p.symbol.char == "*").collect();

    for i in 0..gears.len() {
        let gear= gears[i];

        let mut connected_gear: Option<&Part> = None;
        // try to find the connected gear
        for j in i + 1..gears.len() {
            let candidate = gears[j];
            // if the symbols have the save position
            if gear.symbol.position[0] == candidate.symbol.position[0]
                && gear.symbol.position[1] == candidate.symbol.position[1] {
                connected_gear = Option::from(candidate);
                break;
            }
        }

        if connected_gear.is_some() {
            let connected_gear = connected_gear.unwrap();
            result += connected_gear.number * gear.number;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{part1, process_input};
    use crate::part2;

    #[test]
    fn part1_works() {
        let test_string = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..".to_string();

        assert_eq!(part1(&process_input(test_string)), 4361);
    }

    #[test]
    fn part2_works() {
        let test_string = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..".to_string();

        assert_eq!(part2(&process_input(test_string)), 467835);
    }
}