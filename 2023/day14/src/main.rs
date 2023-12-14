use std::collections::HashMap;
use std::fmt::{Display};
use std::fs;
use std::io::Write;
use std::str::FromStr;
use std::time::Instant;

#[derive(PartialEq, Debug)]
#[derive(Copy, Clone)]
enum Type {
    Rounded,
    Cube,
    Empty,
}

impl FromStr for Type {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "O" => Ok(Type::Rounded),
            "#" => Ok(Type::Cube),
            "." => Ok(Type::Empty),
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
                    Type::Rounded => { r.push('O') }
                    Type::Cube => { r.push('#') }
                    Type::Empty => { r.push('.') }
                }
            }
            r.push('\n')
        }

        r
    }
}

impl Pattern {
    fn fall_north(&mut self) {
        let mut new_data: Vec<Vec<Type>> = vec![];

        for (i, line) in self.data.iter().enumerate() {
            new_data.push(vec![]);

            for (j, s) in line.iter().enumerate() {
                if *s == Type::Rounded {
                    let mut new_i = i;

                    while new_i > 0 && new_data[new_i - 1][j] == Type::Empty {
                        new_i = (new_i as i32 - 1) as usize;
                    }

                    if new_i == i {
                        new_data[i].push(Type::Rounded);
                    } else {
                        new_data[new_i][j] = Type::Rounded;
                        new_data[i].push(Type::Empty);
                    }
                } else {
                    new_data[i].push(s.to_owned());
                }
            }
        }

        self.data = new_data;
    }

    fn fall_east(&mut self) {
        let mut new_data: Vec<Vec<Type>> = self.data.to_owned();

        for j in (0..self.data[0].len()).rev() {
            for i in 0..self.data.len() {
                let s = &new_data[i][j];

                if *s == Type::Rounded {
                    let mut new_j = j;

                    while new_j + 1 < self.data[0].len() && new_data[i][new_j + 1] == Type::Empty {
                        new_j = (new_j as i32 + 1) as usize;
                    }

                    if new_j != j {
                        new_data[i][new_j] = Type::Rounded;
                        new_data[i][j] = Type::Empty;
                    }
                }
            }
        }

        self.data = new_data;
    }

    fn fall_south(&mut self) {
        let mut new_data: Vec<Vec<Type>> = self.data.to_owned();

        for (i, line) in self.data.iter().enumerate().rev() {
            for (j, s) in line.iter().enumerate() {
                if *s == Type::Rounded {
                    let mut new_i = i;

                    while new_i + 1 < new_data.len() && new_data[new_i + 1][j] == Type::Empty {
                        new_i = (new_i as i32 + 1) as usize;
                    }

                    if new_i != i {
                        new_data[new_i][j] = Type::Rounded;
                        new_data[i][j] = Type::Empty;
                    }
                }
            }
        }

        self.data = new_data;
    }

    fn fall_west(&mut self) {
        let mut new_data: Vec<Vec<Type>> = self.data.to_owned();

        for j in 0..self.data[0].len() {
            for i in 0..self.data.len() {
                let s = &new_data[i][j];

                if *s == Type::Rounded {
                    let mut new_j = j;

                    while new_j > 0 && new_data[i][new_j - 1] == Type::Empty {
                        new_j = (new_j as i32 - 1) as usize;
                    }

                    if new_j != j {
                        new_data[i][new_j] = Type::Rounded;
                        new_data[i][j] = Type::Empty;
                    }
                }
            }
        }

        self.data = new_data;
    }

    fn score(&self) -> u64 {
        let mut score = 0;

        for (i, line) in self.data.iter().enumerate() {
            score += (self.data.len() as u64 - i as u64) * line.iter().filter(|t: &&Type| **t == Type::Rounded).collect::<Vec<&Type>>().len() as u64;
        }

        score
    }

    fn do_cycle(&mut self) {
        self.fall_north();
        self.fall_west();
        self.fall_south();
        self.fall_east();
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
    let mut patterns = input.patterns.to_owned();

    for p in &mut patterns {
        p.fall_north();
    }

    patterns.iter().map(|p: &Pattern| p.score()).sum()
}

fn part2(input: &Input) -> u64 {
    let mut patterns = input.patterns.to_owned();

    let mut history = HashMap::new();

    for p in &mut patterns {
        let mut i: i32 = 0;

        while i < 1_000_000_000 {
            p.do_cycle();

            if history.contains_key(&p.to_string()) {
                let last = history.get(&p.to_string()).unwrap();
                let period: i32 = i - *last;

                i += ((1_000_000_000 - i) / period) * period;
            } else {
                history.insert(p.to_string(), i);
            }

            i += 1;
        }
    }

    patterns.iter().map(|p: &Pattern| p.score()).sum()
}


#[cfg(test)]
mod tests {
    use crate::{part1, part2, process_input};

    #[test]
    fn part1_works() {
        let test_string = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....".to_string();
        assert_eq!(part1(&process_input(&test_string)), 136);
    }


    #[test]
    fn part2_works() {
        let test_string = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....".to_string();
        assert_eq!(part2(&process_input(&test_string)), 64);
    }

    #[test]
    fn part2_works1() {
        let test_string = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....".to_string();
        let mut patterns = process_input(&test_string).patterns.pop().unwrap();

        patterns.fall_north();
        patterns.fall_west();
        patterns.fall_south();
        patterns.fall_east();

        assert_eq!(patterns.to_string(), ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
");
    }
}
