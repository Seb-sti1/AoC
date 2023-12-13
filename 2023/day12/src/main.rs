use std::fs;
use std::str::FromStr;
use std::time::Instant;
use regex::Regex;

#[derive(PartialEq, Debug)]
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


#[derive(Debug)]
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

    fn find_next_number_to_add(&self) -> (i32, i32, i32) {
        let mut consecutive = 0;
        let mut n_idx = 0;
        let mut pos_of_last_idx = 0;

        for (idx, s) in self.states.iter().enumerate() {
            match s {
                State::Unknown => {
                    return (consecutive, n_idx, pos_of_last_idx);
                }
                State::Damaged => {
                    consecutive += 1;
                }
                State::Operational => {
                    if consecutive > 0 {
                        if n_idx == self.numbers.len() as i32 || consecutive != self.numbers[n_idx as usize] {
                            return (-1, self.numbers.len() as i32, pos_of_last_idx);
                        }

                        n_idx += 1;
                        pos_of_last_idx = idx as i32;
                        consecutive = 0;
                    }
                }
            }
        }

        return (consecutive, n_idx, pos_of_last_idx);
    }

    fn truncate(&mut self) {
        let (_, n_idx, pos_of_last_idx) = self.find_next_number_to_add();

        if n_idx == 0 {
            return;
        }

        self.states = self.states[pos_of_last_idx as usize + 1..].to_vec();
        self.numbers = self.numbers[n_idx as usize..].to_vec();

        // remove starting dots
        let mut starting_working = 0;
        for (i, c) in self.states.iter().enumerate() {
            if *c != State::Operational {
                starting_working = i;
                break;
            }
        }
        self.states = self.states[starting_working..].to_vec();
    }

    fn is_coherent(&self) -> bool {
        if self.is_correct() {
            return true;
        }

        // if there are too little places to place every damaged springs
        if self.numbers.iter().sum::<i32>() > self.states.iter().filter(|s: &&State| **s != State::Operational).collect::<Vec<&State>>().len() as i32 {
            return false;
        }

        // if there are too many places to place every damaged springs
        if self.numbers.iter().sum::<i32>() < self.states.iter().filter(|s: &&State| **s == State::Damaged).collect::<Vec<&State>>().len() as i32 {
            return false;
        }

        let (consecutive, n_idx, _) = self.find_next_number_to_add();
        let r = (n_idx == self.numbers.len() as i32 && consecutive == 0) || (n_idx < self.numbers.len() as i32 && consecutive <= self.numbers[n_idx as usize]);
        return r;
    }

    fn is_correct(&self) -> bool {
        return self.numbers.len() == 0 && self.states.len() == 0;
    }

    fn next_states(&self) -> Vec<Line> {
        let next_n = self.numbers[0] as usize;

        let mut first_damaged = (self.states.len() as i32 - self.numbers[0] + 1) as usize;
        for i in 0..first_damaged {
            if self.states[i] == State::Damaged {
                first_damaged = i + 1;
                break;
            }
        }


        let mut possibilities: Vec<usize> = vec![];
        for i in 0..first_damaged {
            let mut possible = true;
            for j in i..(i + next_n) {
                if self.states[j] == State::Operational {
                    possible = false;
                    break;
                }
            }

            if possible
                && (i as i32 == 0 || self.states[(i as i32 - 1) as usize] != State::Damaged)
                && (i + next_n == self.states.len() || self.states[i + next_n] != State::Damaged) {
                possibilities.push(i);
            }
        }

        let mut next_states: Vec<Line> = vec![];
        for p in possibilities {
            let mut next_state = self.clone();
            next_state.numbers = self.numbers[1..].to_vec();

            if p + next_n < next_state.states.len() {
                if next_state.states[p + next_n] == State::Damaged {
                    panic!("There is a big mistake...");
                }

                next_state.states = next_state.states[p + next_n + 1..].to_vec();
            } else {
                next_state.states = vec![];
            }

            let nb_damaged_states = next_state.states.iter().filter(|s: &&State| **s == State::Damaged).collect::<Vec<&State>>().len() as i32;
            let max_nb_damaged = next_state.numbers.iter().sum();

            if nb_damaged_states <= max_nb_damaged {
                // if all the numbers were added
                if nb_damaged_states == max_nb_damaged {
                    next_state.states = next_state.states.into_iter().map(|s: State| if s == State::Unknown { State::Operational } else { s }).collect();
                }

                if next_state.is_coherent() {
                    next_state.truncate();
                    // TODO: add only in not in next_states
                    next_states.push(next_state);
                }
            }
        }

        next_states
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
        line.truncate();
        lines.push(line);
    }

    Input { lines }
}

fn part1(input: &Input) -> u64 {
    let mut count = 0;
    let mut current_states: Vec<Line> = vec![];

    for (idx, line) in input.lines.iter().enumerate() {
        current_states.clear();
        current_states.push(line.clone());

        let mut inter_count = 0;

        while current_states.len() > 0 {
            let mut next_states: Vec<Line> = vec![];
            current_states.iter().for_each(|s| next_states.append(&mut s.next_states()));

            current_states.clear();
            for next_state in next_states {
                if !next_state.is_finished() {
                    current_states.push(next_state);
                } else {
                    inter_count += 1;
                }
            }
        }

        println!("{}: {} ", idx, inter_count);
        count += inter_count;
    }

    count
}

fn part2(input: &Input) -> u64 {
    let input = input.for_part2();

    part1(&input)
}


#[cfg(test)]
mod tests {
    use std::fs;
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
        let test_string = "?#.?.????.#????#?? 2,3,1,4".to_string();
        assert_eq!(part1(&process_input(&test_string)), 4);
    }

    #[test]
    fn part1_works3() {
        let contents = fs::read_to_string("input").expect("Should have been able to read the file");
        let result = [2, 3, 6, 15, 6, 7, 1, 4, 4, 3, 4, 1, 1, 1, 3, 3, 21, 2, 4, 4, 3, 7, 2, 3, 8, 3, 2, 5, 1, 7, 6, 3, 314, 2, 4, 4, 7, 2, 2, 6, 3, 1, 2, 7, 10, 10, 5, 16, 9, 1, 4, 6, 1, 1, 4, 2, 2, 1, 1, 2, 3, 1, 1, 4, 15, 3, 2, 2, 3, 19, 2, 21, 15, 1, 2, 1, 9, 10, 58, 4, 1, 1, 1, 5, 8, 2, 3, 7, 3, 3, 3, 12, 2, 3, 4, 10, 8, 2, 1, 17, 10, 3, 17, 1, 2, 2, 26, 3, 2, 6, 14, 12, 1, 4, 3, 10, 7, 2, 8, 1, 1, 7, 8, 10, 2, 12, 3, 18, 6, 19, 4, 8, 1, 5, 15, 3, 2, 26, 6, 3, 4, 3, 4, 4, 8, 3, 10, 27, 1, 7, 1, 8, 6, 6, 3, 1, 2, 7, 2, 1, 3, 2, 2, 56, 4, 8, 3, 9, 6, 2, 1, 2, 8, 4, 2, 1, 5, 1, 12, 4, 7, 2, 2, 2, 18, 24, 4, 2, 5, 4, 15, 7, 26, 15, 3, 1, 6, 1, 2, 21, 21, 3, 3, 2, 9, 45, 1, 10, 1, 39, 2, 4, 2, 4, 5, 6, 6, 3, 2, 10, 13, 2, 7, 3, 6, 4, 3, 3, 35, 1, 3, 7, 2, 4, 4, 18, 3, 2, 52, 2, 4, 1, 8, 37, 2, 46, 6, 8, 4, 1, 1, 5, 1, 1, 1, 4, 12, 1, 3, 1, 3, 3, 2, 1, 16, 10, 1, 1, 21, 1, 1, 5, 6, 2, 3, 4, 4, 4, 14, 22, 14, 4, 2, 6, 21, 6, 2, 12, 5, 59, 1, 41, 20, 1, 4, 4, 6, 6, 22, 11, 6, 1, 4, 4, 1, 12, 5, 4, 11, 7, 24, 6, 7, 14, 7, 13, 3, 30, 5, 4, 15, 4, 4, 5, 16, 4, 1, 2, 8, 13, 20, 30, 3, 4, 2, 12, 13, 12, 1, 2, 4, 10, 1, 3, 1, 3, 1, 1, 9, 5, 5, 4, 6, 2, 3, 2, 6, 3, 2, 3, 2, 3, 28, 12, 2, 20, 52, 11, 81, 1, 12, 13, 5, 4, 6, 1, 4, 4, 16, 2, 3, 2, 2, 36, 61, 13, 24, 6, 2, 2, 4, 1, 1, 3, 13, 3, 3, 1, 4, 4, 4, 2, 1, 2, 2, 6, 3, 2, 4, 3, 10, 12, 2, 7, 64, 4, 2, 6, 3, 2, 24, 11, 1, 8, 4, 2, 4, 4, 2, 4, 20, 3, 3, 5, 8, 7, 6, 2, 1, 2, 1, 10, 10, 10, 1, 9, 4, 1, 10, 7, 2, 14, 1, 1, 5, 3, 12, 4, 12, 8, 1, 2, 19, 12, 1, 5, 26, 2, 2, 3, 6, 4, 2, 2, 2, 2, 4, 4, 4, 3, 4, 3, 4, 3, 2, 2, 2, 1, 2, 2, 31, 1, 44, 3, 3, 10, 3, 6, 1, 5, 6, 6, 6, 7, 94, 4, 2, 1, 4, 8, 17, 9, 1, 1, 3, 33, 4, 2, 1, 34, 16, 1, 3, 4, 2, 11, 1, 1, 3, 9, 1, 4, 14, 30, 3, 1, 3, 2, 2, 1, 1, 20, 18, 7, 7, 4, 4, 8, 1, 2, 1, 4, 2, 6, 7, 3, 6, 5, 3, 3, 1, 3, 1, 4, 2, 3, 6, 2, 3, 6, 6, 2, 9, 3, 17, 1, 3, 1, 3, 7, 11, 2, 2, 12, 5, 18, 3, 17, 1, 1, 4, 3, 1, 2, 5, 3, 7, 5, 28, 2, 2, 6, 2, 3, 1, 3, 1, 1, 55, 16, 10, 4, 22, 91, 2, 2, 2, 15, 4, 5, 6, 39, 3, 4, 24, 4, 7, 10, 2, 9, 3, 9, 3, 3, 2, 2, 12, 24, 3, 3, 1, 9, 5, 10, 2, 1, 6, 40, 13, 7, 3, 10, 2, 2, 10, 10, 3, 3, 6, 60, 2, 9, 3, 16, 24, 7, 22, 4, 12, 9, 2, 4, 3, 1, 6, 80, 7, 2, 5, 26, 252, 3, 1, 2, 2, 9, 1, 4, 9, 1, 8, 3, 8, 4, 11, 1, 3, 55, 9, 2, 3, 8, 3, 9, 4, 2, 1, 18, 1, 6, 1, 1, 2, 1, 8, 15, 2, 2, 29, 4, 1, 1, 8, 1, 2, 4, 191, 3, 1, 11, 6, 2, 1, 10, 4, 4, 1, 6, 2, 8, 10, 6, 8, 1, 1, 4, 4, 3, 4, 2, 12, 1, 2, 10, 8, 6, 3, 7, 1, 13, 2, 3, 6, 11, 6, 3, 2, 4, 23, 4, 106, 2, 4, 18, 3, 40, 4, 11, 2, 20, 2, 3, 9, 1, 4, 6, 3, 1, 3, 3, 17, 1, 4, 3, 1, 6, 3, 6, 1, 3, 4, 1, 1, 4, 12, 24, 1, 3, 1, 6, 8, 5, 3, 13, 2, 6, 1, 2, 5, 29, 4, 12, 7, 5, 12, 2, 5, 1, 10, 7, 1, 8, 11, 3, 18, 2, 4, 6, 2, 7, 2, 2, 1, 1, 4, 3, 4, 2, 3, 38, 2, 6, 2, 1, 1, 4, 3, 2, 8, 16, 3, 2, 1, 15, 6, 2, 7, 1, 12, 11, 9, 4, 6, 16, 6, 10, 4, 3, 1, 20, 3, 6, 4, 4, 16, 18, 1, 13, 10, 27, 22, 3, 9, 1, 6, 3, 27, 4, 2, 6, 13, 3, 1, 4, 2, 1, 3, 1, 6, 3, 7, 1, 1, 3, 2, 4, 2, 1, 6, 2, 2, 16, 8, 6, 9, 11, 12, 4, 6, 8, 1, 4, 3, 3, 3, 5, 1, 3, 4, 3, 10, 9, 6, 2, 2, 8, 3, 2, 4, 8, 3, 2, 28, 84, 14, 3, 1, 2, 86, 18, 5, 3, 4, 1, 1, 12, 37, 14, 9, 24, 19, 6, 3, 10, 14, 5, 18, 3, 1, 4, 18, 2, 1, 1, 5, 31, 1, 12, 2, 4, 1, 6, 1, 13, 3, 2, 2, 7, 2, 6];

        for (i, line) in contents.split("\n").enumerate() {
            println!("Testing {} ({}). Expect {}.", i, line, result[i]);
            assert_eq!(part1(&process_input(&line.to_string())), result[i]);
        }
    }

    // ============== P2
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

