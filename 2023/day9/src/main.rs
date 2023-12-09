use std::fs;
use regex::Regex;

struct List {
    raw_data: Vec<i32>,
}

impl List {
    fn data(&self) -> Vec<Vec<i32>> {
        let mut data: Vec<Vec<i32>> = vec![self.raw_data.clone()];

        let mut idx = 0;
        while (*data[idx].iter().max().unwrap() != 0 || *data[idx].iter().min().unwrap() != 0) && idx < 100_000 {

            let mut new_data: Vec<i32> = vec![];

            for i in 0..data[idx].len() - 1 {
                new_data.push(data[idx][i + 1] - data[idx][i]);
            }

            data.push(new_data);

            idx += 1;
        }

        data
    }

    fn find_next(&self) -> i32 {
        let mut data: Vec<Vec<i32>> = self.data();

        for i in (0..data.len() - 1).rev() {
            let next_value = data[i][data[i].len() - 1] + data[i + 1][data[i + 1].len() - 1];
            data[i].push(next_value);
        }

        data[0][data[0].len() - 1]
    }

    fn find_previous(&self) -> i32 {
        let mut data: Vec<Vec<i32>> = self.data();

        for i in (0..data.len() - 1).rev() {
            let next_value = data[i][0] - data[i + 1][0];
            data[i].insert(0, next_value);
        }

        data[0][0]
    }
}

fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");
    println!("Day 9");

    let data = process_input(&contents);

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn process_input(content: &String) -> Vec<List> {
    let lines = content.split("\n");
    let line_re = Regex::new(r"(-?\d+)").unwrap();

    let mut lists: Vec<List> = vec![];

    for line in lines {
        let raw_data: Vec<i32> = line_re.find_iter(line).map(|m| m.as_str().parse::<i32>().unwrap()).collect();

        lists.push(List {
            raw_data
        })
    }

    lists
}

fn part1(lists: &Vec<List>) -> i32 {
    lists.iter().map(|l| l.find_next()).sum()
}

fn part2(lists: &Vec<List>) -> i32 {
    lists.iter().map(|l| l.find_previous()).sum()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, process_input};

    #[test]
    fn part1_works() {
        let test_string = "0 3 6 9 12 15".to_string();
        assert_eq!(part1(&process_input(&test_string)), 18);
    }

    #[test]
    fn part1_works2() {
        let test_string = "1 3 6 10 15 21".to_string();
        assert_eq!(part1(&process_input(&test_string)), 28);
    }

    #[test]
    fn part1_works3() {
        let test_string = "10 13 16 21 30 45".to_string();
        assert_eq!(part1(&process_input(&test_string)), 68);
    }

    #[test]
    fn part2_works() {
        let test_string = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45".to_string();
        assert_eq!(part2(&process_input(&test_string)), 2);
    }
}