use std::collections::HashMap;
use std::fs;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");
    println!("Day 8");

    let (directions, graph) = process_input(&contents);

    println!("Part 1: {}", part1(directions, &graph));
    println!("Part 2: {}", part2(directions, &graph));
}

fn process_input(content: &String) -> (&str, HashMap<&str, [&str; 2]>) {
    let lines: Vec<&str> = content.split("\n").collect();
    let mut graph: HashMap<&str, [&str; 2]> = HashMap::new();
    let directions= lines[0];

    let line_re = Regex::new(r"([A-Z0-9]+) = \(([A-Z0-9]+), ([A-Z0-9]+)\)").unwrap();

    for i in 2..lines.len() {
        let captures = line_re.captures(lines[i]).unwrap();

        graph.insert(captures.get(1).unwrap().as_str(),
                     [captures.get(2).unwrap().as_str(), captures.get(3).unwrap().as_str()]);
    }

    return (directions, graph);
}

fn count_until_end(directions: &str, graph: &HashMap<&str, [&str; 2]>, beginning: &str, is_end: fn (&str) -> bool) -> i32 {
    let mut node = beginning;
    let mut idx: i32 = 0;

    while !is_end(node) && idx < 100_000_000 {

        let direction = directions.as_bytes()[(idx % directions.len() as i32) as usize] as char;

        match direction {
            'R' => node = graph[node][1],
            'L' => node = graph[node][0],
            _ => {}
        }

        idx += 1;
    }

    idx
}

fn part1(directions: &str, graph: &HashMap<&str, [&str; 2]>) -> i32 {
    count_until_end(directions, graph, "AAA", |candidate: &str| candidate == "ZZZ")
}

fn part2(direction: &str, graph: &HashMap<&str, [&str; 2]>) -> u64 {
    let mut begins: Vec<&str> = vec![];

    for candidate in graph.keys() {
        if candidate.ends_with("A") {
            begins.push(*candidate);
        }
    }

    let counts: Vec<i32> = begins.iter().map(|b| count_until_end(direction, graph, b, |candidate: &str| candidate.ends_with("Z"))).collect();

    let mut lcm = num::integer::lcm(counts[0] as u64, counts[1] as u64);

    for i in 2..counts.len() {
        lcm = num::integer::lcm(counts[i] as u64, lcm);
    }

    lcm
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, process_input};

    #[test]
    fn part1_works() {
        let test_string = "0 3 6 9 12 15".to_string();
        let (directions, graph) = process_input(&test_string);
        assert_eq!(part1(directions, &graph), 18);
    }

    #[test]
    fn part1_works2() {
        let test_string = "1 3 6 10 15 21".to_string();
        let (directions, graph) = process_input(&test_string);
        assert_eq!(part1(directions, &graph), 28);
    }

    #[test]
    fn part1_works3() {
        let test_string = "10 13 16 21 30 45".to_string();
        let (directions, graph) = process_input(&test_string);
        assert_eq!(part1(directions, &graph), 68);
    }

    #[test]
    fn part2_works() {
        let test_string = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45".to_string();
        let (directions, graph) = process_input(&test_string);
        assert_eq!(part2(directions, &graph), 114);
    }
}