use std::fs;
use std::str::FromStr;
use std::time::Instant;
use regex::Regex;

#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum State {
    Unknown,
    Damaged,
    Operational,
}

impl FromStr for State {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "?" => Ok(State::Unknown),
            "." => Ok(State::Operational),
            "#" => Ok(State::Damaged),
            _ => Err(())
        };
    }
}

struct Line {
    states: Vec<State>,
    numbers: Vec<i32>,
}

impl Clone for Line {
    fn clone(&self) -> Self {
        Line {
            states: self.states.iter().map(|s: &State| s.to_owned()).collect(),
            numbers: self.numbers.iter().map(|n: &i32| n.to_owned()).collect(),
        }
    }
}


impl Line {
    fn change_obvious_unknown(&mut self) {
        let n_max = self.numbers.iter().max().unwrap();

        let mut consecutive = 0;

        let mut operational: Vec<i32> = vec![];

        for (i, c) in self.states.iter().enumerate() {
            match c {
                State::Unknown => {
                    if consecutive == *n_max {
                        if i as i32 - *n_max - 1 >= 0 {
                            operational.push(i as i32 - *n_max - 1);
                        }
                        operational.push(i as i32);
                    }
                    consecutive = 0;
                }
                State::Damaged => {
                    consecutive += 1;
                }
                State::Operational => {
                    consecutive = 0;
                }
            }
        }

        operational.iter().for_each(|i: &i32| self.states[*i as usize] = State::Operational);
    }

    fn is_coherent(&self) -> bool {
        if self.numbers.iter().sum::<i32>() > self.states.iter().filter(|s: &&State| **s != State::Operational).collect::<Vec<&State>>().len() as i32 {
            return false;
        }

        let mut consecutive = 0;
        let mut n_idx = 0;

        for s in &self.states {
            match s {
                State::Unknown => {
                    return (n_idx < self.numbers.len() && consecutive <= self.numbers[n_idx]) || consecutive == 0;
                }
                State::Damaged => {
                    consecutive += 1;
                }
                State::Operational => {
                    if consecutive > 0 {
                        if n_idx == self.numbers.len() || consecutive != self.numbers[n_idx] {
                            return false;
                        }

                        n_idx += 1;
                        consecutive = 0;
                    }
                }
            }
        }

        return (n_idx == self.numbers.len() && consecutive == 0) || (n_idx == self.numbers.len() - 1 && self.numbers[n_idx] == consecutive);
    }

    fn is_correct(&self) -> bool {
        let mut n_idx = 0;
        let mut next = 0;

        for s in &self.states {
            match *s {
                State::Damaged => {
                    next += 1;
                }
                State::Operational => {
                    if next != 0 {
                        if n_idx == self.numbers.len() || self.numbers[n_idx] != next {
                            return false;
                        }

                        next = 0;
                        n_idx += 1;
                    }
                }
                _ => {}
            }
        }

        return (n_idx == self.numbers.len() && next == 0) || (n_idx == self.numbers.len() - 1 && self.numbers[n_idx] == next);
    }

    fn next_states(&self) -> Vec<Line> {
        let mut first_unknown: i32 = -1;

        for (idx, s) in self.states.iter().enumerate() {
            if *s == State::Unknown {
                first_unknown = idx as i32;
                break;
            }
        }

        if first_unknown < 0 {
            return vec![];
        }

        let mut r: Vec<Line> = vec![self.clone(), self.clone()];

        r[0].states[first_unknown as usize] = State::Operational;
        r[1].states[first_unknown as usize] = State::Damaged;

        r.into_iter().filter(|l: &Line| l.is_coherent()).collect()
    }

    fn is_finished(&self) -> bool {
        for s in &self.states {
            if *s == State::Unknown {
                return false;
            }
        }
        true
    }
}

struct Input {
    lines: Vec<Line>,
}

impl Input {
    fn for_part2(&self) -> Input {
        let mut lines: Vec<Line> = vec![];

        for line in &self.lines {
            let mut l = Line {
                states: vec![],
                numbers: vec![],
            };

            l.states.append(&mut line.states.iter().map(|s: &State| s.to_owned()).collect());
            l.states.push(State::Unknown);
            l.states.append(&mut line.states.iter().map(|s: &State| s.to_owned()).collect());
            l.states.push(State::Unknown);
            l.states.append(&mut line.states.iter().map(|s: &State| s.to_owned()).collect());
            l.states.push(State::Unknown);
            l.states.append(&mut line.states.iter().map(|s: &State| s.to_owned()).collect());
            l.states.push(State::Unknown);
            l.states.append(&mut line.states.iter().map(|s: &State| s.to_owned()).collect());

            l.numbers.append(&mut line.numbers.iter().map(|n: &i32| n.to_owned()).collect());
            l.numbers.append(&mut line.numbers.iter().map(|n: &i32| n.to_owned()).collect());
            l.numbers.append(&mut line.numbers.iter().map(|n: &i32| n.to_owned()).collect());
            l.numbers.append(&mut line.numbers.iter().map(|n: &i32| n.to_owned()).collect());
            l.numbers.append(&mut line.numbers.iter().map(|n: &i32| n.to_owned()).collect());

            lines.push(l);
        }

        Input { lines }
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
    let mut lines: Vec<Line> = vec![];

    let number_re = Regex::new("([0-9]+)").unwrap();

    for line_str in content.split("\n") {
        let mut line = Line {
            states: vec![],
            numbers: vec![],
        };

        for c in line_str.as_bytes() {
            let c = *c as char;
            if c == ' ' {
                break;
            }

            line.states.push(State::from_str(c.to_string().as_str()).unwrap());
        }
        line.numbers = number_re.find_iter(line_str).map(|m| m.as_str().parse::<i32>().unwrap()).collect();

        line.change_obvious_unknown();
        lines.push(line);
    }

    Input { lines }
}

fn part1(input: &Input) -> u64 {
    let mut count = 0;
    let mut current_states: Vec<Line> = vec![];
    let size = input.lines.len();

    for (idx, line) in input.lines.iter().enumerate() {
        current_states.clear();
        current_states.push(line.clone());

        while current_states.iter().any(|s| !s.is_finished()) {
            let mut next_states: Vec<Line> = vec![];
            current_states.iter().for_each(|s| next_states.append(&mut s.next_states()));
            current_states = next_states;
        }

        count += current_states.iter().filter(|l| l.is_correct()).collect::<Vec<&Line>>().len() as u64;

        if idx % 20 == 0 {
            println!("{}%", idx * 100 / size);
        }
    }

    count
}

fn part2(input: &Input) -> u64 {
    let input = input.for_part2();

    part1(&input)
}


#[cfg(test)]
mod tests {
    use crate::{part1, part2, process_input};

    #[test]
    fn part1_works() {
        let test_string = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1".to_string();
        assert_eq!(part1(&process_input(&test_string)), 21);
    }

    #[test]
    fn part1_works2() {
        let test_string = "?###???????? 3,2,1".to_string();
        assert_eq!(part1(&process_input(&test_string)), 10);
    }

    #[test]
    fn part2_works() {
        let test_string = "???.### 1,1,3".to_string();
        assert_eq!(part2(&process_input(&test_string)), 1);
    }

    #[test]
    fn part2_works2() {
        let test_string = ".??..??...?##. 1,1,3".to_string();
        assert_eq!(part2(&process_input(&test_string)), 16384);
    }

    #[test]
    fn part2_works3() {
        let test_string = "?###???????? 3,2,1".to_string();
        assert_eq!(part2(&process_input(&test_string)), 506250);
    }

    #[test]
    fn part2_works4() {
        let test_string = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1".to_string();
        assert_eq!(part2(&process_input(&test_string)), 525152);
    }
}

