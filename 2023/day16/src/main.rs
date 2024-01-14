use std::cmp::max;
use std::fs;
use std::str::FromStr;
use std::time::Instant;
use itertools::Itertools;


struct State {
    position: (i32, i32),
    direction: Direction,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        return self.position == other.position && self.direction == other.direction;
    }
}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum Direction {
    South,
    North,
    East,
    West,
}

impl Direction {
    fn next(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Direction::South => {
                (x, y + 1)
            }
            Direction::North => {
                (x, y - 1)
            }
            Direction::East => {
                (x + 1, y)
            }
            Direction::West => {
                (x - 1, y)
            }
        }
    }
}

#[derive(Debug)]
enum Type {
    Empty,
    Mirror,
    ReversedMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl FromStr for Type {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "/" => Ok(Type::Mirror),
            "\\" => Ok(Type::ReversedMirror),
            "." => Ok(Type::Empty),
            "|" => Ok(Type::VerticalSplitter),
            "-" => Ok(Type::HorizontalSplitter),
            _ => Err(())
        };
    }
}

struct Input {
    map: String,
}

impl Input {
    fn get(&self, (x, y): (i32, i32)) -> Type {
        let lines = self.map.split("\n").collect::<Vec<&str>>();
        return Type::from_str(&String::from(lines[y as usize].as_bytes()[x as usize] as char)).unwrap();
    }

    fn size(&self) -> (i32, i32) {
        return (self.map.split("\n").collect::<Vec<&str>>().len() as i32,
                self.map.split("\n").collect::<Vec<&str>>()[0].len() as i32);
    }
}


fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");

    let data = process_input(contents);

    let t1 = Instant::now();
    let p1 = part1(&data);
    let t1 = t1.elapsed();
    println!("Part 1 (in {}s): {} ", t1.as_nanos() as f32 / 1_000_000_000f32, p1);

    let t2 = Instant::now();
    let p2 = part2(&data);
    let t2 = t2.elapsed();
    println!("Part 2 (in {}s): {} ", t2.as_nanos() as f32 / 1_000_000_000f32, p2);
}

fn get_seen_positions(input: &Input, initial: State) -> u64 {
    let mut states: Vec<State> = vec![initial];
    let mut seen: Vec<State> = vec![];
    let (n, m) = input.size();

    while states.len() > 0 {
        let mut next_states: Vec<State> = vec![];

        for s in states {
            if 0 > s.position.0 || s.position.0 >= n || 0 > s.position.1 || s.position.1 >= m || seen.contains(&s) {
                continue;
            }

            match input.get(s.position) {
                Type::Empty => {
                    next_states.push(State { position: s.direction.next(s.position), direction: s.direction.clone() })
                }
                Type::Mirror => { // "/"
                    match s.direction {
                        Direction::South => {
                            next_states.push(State { position: Direction::West.next(s.position), direction: Direction::West })
                        }
                        Direction::North => {
                            next_states.push(State { position: Direction::East.next(s.position), direction: Direction::East })
                        }
                        Direction::East => {
                            next_states.push(State { position: Direction::North.next(s.position), direction: Direction::North })
                        }
                        Direction::West => {
                            next_states.push(State { position: Direction::South.next(s.position), direction: Direction::South })
                        }
                    }
                }
                Type::ReversedMirror => { // "\"
                    match s.direction {
                        Direction::South => {
                            next_states.push(State { position: Direction::East.next(s.position), direction: Direction::East })
                        }
                        Direction::North => {
                            next_states.push(State { position: Direction::West.next(s.position), direction: Direction::West })
                        }
                        Direction::East => {
                            next_states.push(State { position: Direction::South.next(s.position), direction: Direction::South })
                        }
                        Direction::West => {
                            next_states.push(State { position: Direction::North.next(s.position), direction: Direction::North })
                        }
                    }
                }
                Type::HorizontalSplitter => { // -
                    match s.direction {
                        Direction::South | Direction::North => {
                            next_states.push(State { position: Direction::West.next(s.position), direction: Direction::West });
                            next_states.push(State { position: Direction::East.next(s.position), direction: Direction::East });
                        }
                        _ => {
                            next_states.push(State { position: s.direction.next(s.position), direction: s.direction.clone() });
                        }
                    }
                }
                Type::VerticalSplitter => { // |
                    match s.direction {
                        Direction::East | Direction::West
                        => {
                            next_states.push(State { position: Direction::South.next(s.position), direction: Direction::South });
                            next_states.push(State { position: Direction::North.next(s.position), direction: Direction::North });
                        }
                        _ => {
                            next_states.push(State { position: s.direction.next(s.position), direction: s.direction.clone() });
                        }
                    }
                }
            }

            seen.push(s);
        }

        states = next_states;
    }

    let positions: Vec<(i32, i32)> = seen.iter().map(|s| (s.position.0, s.position.1)).collect::<Vec<(i32, i32)>>().into_iter().unique().collect();

    positions.len() as u64
}

fn process_input(content: String) -> Input {
    return Input { map: content };
}

fn part1(input: &Input) -> u64 {
    get_seen_positions(input, State { position: (0, 0), direction: Direction::East })
}


fn part2(input: &Input) -> u64 {
    let mut r = 0;
    let (n, m) = input.size();

    for i in 0..n {
        r = max(r, get_seen_positions(input, State { position: (i, 0), direction: Direction::South }));
        r = max(r, get_seen_positions(input, State { position: (i, m - 1), direction: Direction::North }));

        if i % 5 == 0 {
            println!("{}%", i * 100 / (n + m));
        }
    }


    for i in 0..m {
        r = max(r, get_seen_positions(input, State { position: (0, i), direction: Direction::East }));
        r = max(r, get_seen_positions(input, State { position: (n - 1, i), direction: Direction::West }));

        if i % 5 == 0 {
            println!("{}%", (n + i) * 100 / (n + m));
        }
    }

    r
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, process_input};

    #[test]
    fn part1_works() {
        let test_string = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....".to_string();
        assert_eq!(part1(&process_input(test_string)), 46);
    }

    #[test]
    fn part2_works() {
        let test_string = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....".to_string();
        assert_eq!(part2(&process_input(test_string)), 51);
    }
}