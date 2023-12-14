use std::cmp::min;
use std::fmt::{Display, Formatter};
use std::fs;
use std::str::FromStr;
use std::time::Instant;
use lazy_static::lazy_static;

#[derive(PartialEq, Debug)]
#[derive(Copy, Clone)]
enum Type {
    Rock,
    Ash,
}

impl FromStr for Type {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "." => Ok(Type::Ash),
            "#" => Ok(Type::Rock),
            _ => Err(())
        };
    }
}

#[derive(Debug)]
struct Line {
    states: Vec<Type>,
}

impl Clone for Line {
    fn clone(&self) -> Self {
        Line {
            states: self.states.iter().map(|s: &Type| s.to_owned()).collect(),
        }
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut l = Line {
            states: vec![]
        };

        for c in s.as_bytes() {
            let c = *c as char;
            l.states.push(Type::from_str(c.to_string().as_str()).unwrap());
        }

        Ok(l)
    }
}


struct Input {
    patterns: Vec<Pattern>,
}

struct Pattern {
    data: Vec<Vec<Type>>,
}

impl Clone for Pattern {
    fn clone(&self) -> Self {
        Pattern {
            data: self.data.to_owned(),
        }
    }
}

impl ToString for Pattern {
    fn to_string(&self) -> String {
        let mut r = String::from("");

        for line in &self.data {
            for t in line {
                match t {
                    Type::Rock => { r.push('#') }
                    Type::Ash => { r.push('.') }
                }
            }
            r.push('\n')
        }

        r
    }
}

impl Pattern {
    fn identical_line(&self, a: usize, b: usize) -> bool {
        let l_a = &self.data[a];
        let l_b = &self.data[b];

        for i in 0..l_a.len() {
            if l_a[i] != l_b[i] {
                return false;
            }
        }

        true
    }

    fn identical_column(&self, a: usize, b: usize) -> bool {
        for i in 0..self.data.len() {
            if self.data[i][a] != self.data[i][b] {
                return false;
            }
        }

        true
    }

    fn count_diff_line(&self, a: usize, b: usize) -> i32 {
        let l_a = &self.data[a];
        let l_b = &self.data[b];

        let mut count = 0;
        for i in 0..l_a.len() {
            if l_a[i] != l_b[i] {
                count += 1;
            }
        }
        count
    }

    fn count_diff_column(&self, a: usize, b: usize) -> i32 {
        let mut count = 0;
        for i in 0..self.data.len() {
            if self.data[i][a] != self.data[i][b] {
                count += 1;
            }
        }

        count
    }

    fn is_mirror_line(&self, pivot: usize) -> bool {
        let n = min(pivot + 1, self.data.len() - pivot - 1);

        for i in 0..n {
            if !self.identical_line(pivot - i, pivot + i + 1) {
                return false;
            }
        }

        true
    }

    fn is_mirror_column(&self, pivot: usize) -> bool {
        let n = min(pivot + 1, self.data[0].len() - pivot - 1);
        for i in 0..n {
            if !self.identical_column(pivot - i, pivot + i + 1) {
                return false;
            }
        }

        true
    }

    fn is_near_mirror_line(&self, pivot: usize) -> (bool, (i32, i32)) {
        let n = min(pivot + 1, self.data.len() - pivot - 1);

        let mut count = 0;
        let mut line_to_change: (i32, i32) = (0, 0);

        for i in 0..n {
            let c = self.count_diff_line(pivot - i, pivot + i + 1);
            if c == 1 {
                line_to_change = (pivot as i32, i as i32);
            }
            count += c;
        }

        (count == 1, line_to_change)
    }

    fn is_near_mirror_column(&self, pivot: usize) -> (bool, (i32, i32)) {
        let n = min(pivot + 1, self.data[0].len() - pivot - 1);

        let mut count = 0;
        let mut column_to_change: (i32, i32) = (0, 0);

        for i in 0..n {
            let c = self.count_diff_column(pivot - i, pivot + i + 1);
            if c == 1 {
                column_to_change = (pivot as i32, i as i32);
            }
            count += c;
        }

        (count == 1, column_to_change)
    }

    fn fix_line(&mut self, pivot: i32, idx: i32) {
        let a = (pivot - idx) as usize;
        let b = (pivot + idx + 1) as usize;

        for i in 0..self.data[0].len() {
            if self.data[a][i] != self.data[b][i] {
                self.data[b][i] = self.data[a][i].clone();
                return;
            }
        }
    }

    fn fix_column(&mut self, pivot: i32, idx: i32) {
        let a = (pivot - idx) as usize;
        let b = (pivot + idx + 1) as usize;

        for i in 0..self.data.len() {
            if self.data[i][a] != self.data[i][b] {
                self.data[i][b] = self.data[i][a].clone();
                return;
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");

    let data = process_input(&contents);

    let t1 = Instant::now();
    let p1 = part1(&data);
    let t1 = t1.elapsed();
    println!("Part 1 (in {}s): {} ", t1.as_millis() as f32 / 1000f32, p1);

    let t2 = Instant::now();
    let p2 = part2(&data);
    let t2 = t2.elapsed();
    println!("Part 2 (in {}s): {} ", t2.as_millis() as f32 / 1000f32, p2);
}

fn process_input(content: &String) -> Input {
    let mut patterns: Vec<Pattern> = vec![];
    let mut pattern = Pattern {
        data: vec![],
    };

    for line_str in content.split("\n") {
        if line_str == "" {
            patterns.push(pattern);

            pattern = Pattern {
                data: vec![],
            };
        } else {
            pattern.data.push(Line::from_str(line_str).unwrap().states);
        }
    }

    if pattern.data.len() > 0 {
        patterns.push(pattern);

        pattern = Pattern {
            data: vec![],
        };
    }

    Input { patterns }
}

fn part1(input: &Input) -> u64 {
    let mut count: u64 = 0;

    for p in &input.patterns {
        for i in 0..(p.data.len() as i32 - 1) as usize {
            if p.is_mirror_line(i) {
                count += 100 * (i as u64 + 1);
            }
        }

        for i in 0..(p.data[0].len() as i32 - 1) as usize {
            if p.is_mirror_column(i) {
                count += i as u64 + 1;
            }
        }
    }

    count
}

fn part2(input: &Input) -> u64 {
    let mut new_input = Input {
        patterns: vec![],
    };

    'pattern_for: for mut p in &input.patterns {
        let mut p = p.to_owned();

        for i in 0..(p.data.len() as i32 - 1) as usize {
            let (r, (pivot, index)) = p.is_near_mirror_line(i);
            if r {
                p.fix_line(pivot, index);

                new_input.patterns.push(p);
                continue 'pattern_for;
            }
        }

        for i in 0..(p.data[0].len() as i32 - 1) as usize {
            let (r, (pivot, index)) = p.is_near_mirror_column(i);
            if r {
                p.fix_column(pivot, index);

                new_input.patterns.push(p);
                continue 'pattern_for;
            }
        }

        panic!("That's why")
    }

    let mut count = 0;

    for (idx, p) in input.patterns.iter().enumerate() {
        let new_p = &new_input.patterns[idx];

        for i in 0..(p.data.len() as i32 - 1) as usize {
            if new_p.is_mirror_line(i) && !p.is_mirror_line(i) {
                count += 100 * (i as u64 + 1);
            }
        }

        for i in 0..(p.data[0].len() as i32 - 1) as usize {
            if new_p.is_mirror_column(i) && !p.is_mirror_column(i) {
                count += i as u64 + 1;
            }
        }
    }

    count
}


#[cfg(test)]
mod tests {
    use crate::{part1, part2, process_input};

    #[test]
    fn part1_works() {
        let test_string = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#".to_string();
        assert_eq!(part1(&process_input(&test_string)), 405);
    }

    #[test]
    fn part1_works1() {
        let test_string = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#".to_string();
        assert_eq!(part1(&process_input(&test_string)), 400);
    }

    #[test]
    fn part1_works2() {
        let test_string = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.".to_string();
        assert_eq!(part1(&process_input(&test_string)), 5);
    }

    #[test]
    fn part2_works() {
        let test_string = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#".to_string();
        assert_eq!(part2(&process_input(&test_string)), 400);
    }

    #[test]
    fn part2_works2() {
        let test_string = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.".to_string();
        assert_eq!(part2(&process_input(&test_string)), 300);
    }
}