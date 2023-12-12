use std::cmp::{max, min};
use std::fs;
use regex::Regex;

struct Galaxy {
    i: i32,
    j: i32,
}

impl Galaxy {
    fn distance_to(&self, to: &Galaxy, line_to_expand: &Vec<i32>, column_to_expand: &Vec<i32>, expand: i32) -> u64 {
        let i_m: &i32 = min(&self.i, &to.i);
        let i_ma: &i32 = max(&self.i, &to.i);
        let j_m: &i32 = min(&self.j, &to.j);
        let j_ma: &i32 = max(&self.j, &to.j);

        let n = line_to_expand.iter().filter(|p: &&i32| *i_m < **p && **p < *i_ma).collect::<Vec<&i32>>().len() as u64
            + column_to_expand.iter().filter(|p: &&i32| *j_m < **p && **p < *j_ma).collect::<Vec<&i32>>().len() as u64;

        return (self.i - to.i).abs() as u64 + (self.j - to.j).abs() as u64 + n*(expand - 1) as u64;
    }
}


fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");

    let (line_to_expand, column_to_expand, galaxies) = process_input(&contents);

    println!("Part 1: {}", part1(&line_to_expand, &column_to_expand, &galaxies));
    println!("Part 2: {}", part2(&line_to_expand, &column_to_expand, &galaxies, 1_000_000));
}

fn expand_input(content: &String) -> (Vec<i32>, Vec<i32>) {
    let lines = content.split("\n").collect::<Vec<&str>>();

    let mut line_to_expand: Vec<i32> = vec![];
    let mut column_to_expand: Vec<i32> = vec![];

    for (i, line) in lines.iter().enumerate() {
        if line.replace(".", "").len() == 0 {
            line_to_expand.push(i as i32);
        }
    }

    for j in 0..lines[0].len() {
        let mut empty_column = true;
        for i in 0..lines.len() {
            if lines[i].as_bytes()[j] as char != '.' {
                empty_column = false;
                break;
            }
        }

        if empty_column {
            column_to_expand.push(j as i32);
        }
    }

    (line_to_expand, column_to_expand)
}

fn process_input(content: &String) -> (Vec<i32>, Vec<i32>, Vec<Galaxy>) {
    let (line_to_expand, column_to_expand) = expand_input(&content);

    let sharp_re = Regex::new("#").unwrap();
    let mut galaxy: Vec<Galaxy> = vec![];

    for (i, line) in content.split("\n").enumerate() {
        let mut m: Vec<Galaxy> = sharp_re.find_iter(line).map(|m| Galaxy { i: i as i32, j: m.start() as i32 }).collect();
        galaxy.append(&mut m);
    }

    (line_to_expand, column_to_expand, galaxy)
}

fn part1(line_to_expand: &Vec<i32>, column_to_expand: &Vec<i32>, galaxies: &Vec<Galaxy>) -> u64 {
    let mut sum = 0;

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            sum += galaxies[i].distance_to(&galaxies[j], line_to_expand, column_to_expand, 2);
        }
    }

    sum
}

fn part2(line_to_expand: &Vec<i32>, column_to_expand: &Vec<i32>, galaxies: &Vec<Galaxy>, expand: i32) -> u64 {
    let mut sum = 0;

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            sum += galaxies[i].distance_to(&galaxies[j], line_to_expand, column_to_expand, expand);
        }
    }

    sum
}


#[cfg(test)]
mod tests {
    use crate::{part1, part2, process_input};

    #[test]
    fn part1_works() {
        let test_string = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....".to_string();
        let (line_to_expand, column_to_expand, galaxies) = process_input(&test_string);
        assert_eq!(part1(&line_to_expand, &column_to_expand, &galaxies), 374);
    }

    #[test]
    fn part2_works() {
        let test_string = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....".to_string();
        let (line_to_expand, column_to_expand, galaxies) = process_input(&test_string);
        assert_eq!(part2(&line_to_expand, &column_to_expand, &galaxies, 10), 1030);
    }

    #[test]
    fn part2_works2() {
        let test_string = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....".to_string();
        let (line_to_expand, column_to_expand, galaxies) = process_input(&test_string);
        assert_eq!(part2(&line_to_expand, &column_to_expand, &galaxies, 100), 8410);
    }
}