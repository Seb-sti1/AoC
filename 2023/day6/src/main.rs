use std::fs;
use std::str::FromStr;
use regex::{Regex};

struct Leaderboard {
    time: u64,
    dist: u64
}

fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");

    println!("Day 5");

    println!("Part 1: {}", part1(&process_input(&contents)));
    println!("Part 2: {}", part2(&process_input(&contents.replace(" ", ""))));
}

/// Process the input and convert it to a more intelligible structure
///
/// # Arguments
/// * `content` - the content of the input file
fn process_input(content: &String) -> Vec<Leaderboard> {
    let lines: Vec<&str> = content.split("\n").collect();

    let number_re: Regex = Regex::new(r"(\d+)").unwrap();

    let times: Vec<u64> = number_re.find_iter(lines[0]).map(|m| m.as_str().parse::<u64>().unwrap()).collect();
    let distances: Vec<u64> = number_re.find_iter(lines[1]).map(|m| m.as_str().parse::<u64>().unwrap()).collect();

    let mut leaderboards: Vec<Leaderboard> = vec![];

    for i in 0..times.len() {
        leaderboards.push(Leaderboard {
            time: times[i],
            dist: distances[i],
        })
    }

    leaderboards
}

fn part1(data: &Vec<Leaderboard>) -> i32 {
    let mut result= vec![];

    for leaderboard in data {
        let mut count = 0;
        for i in 0..leaderboard.time + 1 {
            let dist = i * (leaderboard.time - i);
            if dist > leaderboard.dist {
                count += 1;
            }
        }

        result.push(count);
    }

    result.into_iter().reduce(|acc, n| acc * n).unwrap()
}

fn part2(data: &Vec<Leaderboard>) -> i32 {
    part1(data)
}

#[cfg(test)]
mod tests {
    use crate::{part1, process_input};
    use crate::part2;

    #[test]
    fn part1_works() {
        let test_string = "Time:      7  15   30
Distance:  9  40  200".to_string();

        assert_eq!(part1(&process_input(&test_string)), 288);
    }

    #[test]
    fn part2_works() {
        let test_string = "Time:      71530
Distance:  940200".to_string();

        assert_eq!(part2(&process_input(&test_string)), 71503);
    }
}