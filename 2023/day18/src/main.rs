use std::collections::HashMap;
use std::fs;
use std::ops::{Add, Div};
use std::str::FromStr;
use std::time::Instant;
use regex::Regex;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "3" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "1" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "2" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            "0" => Ok(Direction::Right),
            _ => Err(())
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    i: isize,
    j: isize,
}

impl Pos {
    fn manhattan_distance(&self, other: &Self) -> isize {
        ((other.i - self.i).abs() + (other.j - self.j).abs()) as isize
    }
}

impl Pos {
    fn execute(&self, instruction: &Instruction) -> Vec<Pos> {
        let mut result: Vec<Pos> = vec![];

        let mut current = self.clone();

        for _ in 0..instruction.size {
            match instruction.dir {
                Direction::Up => {
                    current.i -= 1;
                }
                Direction::Down => {
                    current.i += 1;
                }
                Direction::Right => {
                    current.j += 1;
                }
                Direction::Left => {
                    current.j -= 1;
                }
            }
            result.push(current.clone());
        }

        result
    }

    fn next(&self, instruction: &Instruction) -> Pos {
        let mut next = self.clone();

        match instruction.dir {
            Direction::Up => {
                next.i -= instruction.size as isize;
            }
            Direction::Down => {
                next.i += instruction.size as isize;
            }
            Direction::Right => {
                next.j += instruction.size as isize;
            }
            Direction::Left => {
                next.j -= instruction.size as isize;
            }
        }

        next
    }

    fn neighbours(&self, i_min: isize, i_max: isize, j_min: isize, j_max: isize) -> Vec<Pos> {
        vec![Pos { i: self.i - 1, j: self.j },
             Pos { i: self.i, j: self.j - 1 }, Pos { i: self.i, j: self.j + 1 },
             Pos { i: self.i + 1, j: self.j }]
            .into_iter().filter(|p| p.i >= i_min && p.i <= i_max)
            .into_iter().filter(|p| p.j >= j_min && p.j <= j_max)
            .collect()
    }
}


#[derive(Debug)]
struct Instruction {
    dir: Direction,
    size: u32,
    color: String,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line_re: Regex = Regex::new(r"([LRDU]) (\d+) \((#[0-9a-f]{6})\)").unwrap();
        let m = line_re.captures(s).unwrap();

        return Ok(Instruction {
            dir: Direction::from_str(m.get(1).unwrap().as_str()).unwrap(),
            size: m.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            color: m.get(3).unwrap().as_str().to_string(),
        });
    }
}

struct Input {
    instructions: Vec<Instruction>,
}

fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");

    let data = process_input(contents);

    let t1 = Instant::now();
    let p1 = part1(&data);
    let t1 = t1.elapsed();
    println!("Part 1 (in {}s): {} ", t1.as_nanos() as f32 / 1_000_000_000f32, p1);
    assert_eq!(p1, 62500);

    let t2 = Instant::now();
    let p2 = part2(&data);
    let t2 = t2.elapsed();
    println!("Part 2 (in {}s): {} ", t2.as_nanos() as f32 / 1_000_000_000f32, p2);
}

fn process_input(content: String) -> Input {
    let mut instructions: Vec<Instruction> = vec![];

    for line in content.lines() {
        instructions.push(Instruction::from_str(line).unwrap())
    }

    return Input { instructions };
}

fn part1(input: &Input) -> u64 {
    let debug = false;
    let mut frontier: HashMap<Pos, bool> = HashMap::new();

    let mut pos: Pos = Pos { i: 0, j: 0 };
    frontier.insert(pos.clone(), true);

    for instruction in &input.instructions {
        let mut r: Vec<Pos> = pos.execute(&instruction).into_iter().filter(|p| !frontier.contains_key(&p)).collect();
        pos = r.last().unwrap().clone();
        r.into_iter().for_each(|p| { frontier.insert(p, true); });
    }

    let i_min = frontier.iter().map(|p| p.0.i).into_iter().min().unwrap();
    let i_max = frontier.iter().map(|p| p.0.i).into_iter().max().unwrap();
    let j_min = frontier.iter().map(|p| p.0.j).into_iter().min().unwrap();
    let j_max = frontier.iter().map(|p| p.0.j).into_iter().max().unwrap();

    println!("{}x{}", i_max as i64 - i_min as i64 + 1, j_max as i64 - j_min as i64 + 1);
    println!("Flooding...");

    let mut flooded: HashMap<Pos, bool> = HashMap::new();
    let mut to_flood: Vec<Pos> = vec![Pos { i: 1, j: 1 }];

    while to_flood.len() > 0 {
        let mut new_to_flood: Vec<Pos> = vec![];

        for pos in to_flood.clone() {
            new_to_flood.append(&mut pos.neighbours(i_min, i_max, j_min, j_max)
                .into_iter().filter(|p| !to_flood.contains(p) && !new_to_flood.contains(p))
                .into_iter().filter(|p| !frontier.contains_key(p))
                .into_iter().filter(|p| !flooded.contains_key(p))
                .collect::<Vec<Pos>>());
            flooded.insert(pos, true);
        }
        to_flood = new_to_flood;
    }

    let area = (flooded.len() + frontier.len()) as u64;

    if debug {
        for i in i_min..i_max + 1 {
            for j in j_min..j_max + 1 {
                if (frontier.contains_key(&Pos { i, j }) && flooded.contains_key(&Pos { i, j })) {
                    print!("0")
                } else if frontier.contains_key(&Pos { i, j }) {
                    print!("#")
                } else if flooded.contains_key(&Pos { i, j }) {
                    print!("X")
                } else {
                    print!(".")
                }
            }
            println!()
        }
    }


    area
}


fn shoelace_formula(points: &Vec<Pos>) -> isize {
    let len = points.len();

    let (area, perimeter) =
        points
            .iter()
            .enumerate()
            .fold((0isize, 0isize), |(sum, perimeter), (i, p1)| {
                let l = (i + 1) % len;
                let p2 = points[l].clone();

                let new_perimeter = perimeter + p1.manhattan_distance(&p2);
                let new_area = sum + (p1.j * p2.i) - (p1.i * p2.j);

                (new_area, new_perimeter)
            });

    area.abs().add(perimeter).div(2).add(1)
}

fn part2(input: &Input) -> u64 {
    let mut frontier: Vec<Pos> = vec![];


    let mut pos: Pos = Pos { i: 0, j: 0 };
    frontier.push(pos.clone());

    for instruction in &input.instructions {
        let dir = Direction::from_str(&instruction.color[6..7]).unwrap();
        let size = i32::from_str_radix(&instruction.color[1..6], 16).unwrap();
        let new_instruction: Instruction = Instruction {
            dir,
            size: size as u32,
            color: "".to_string(),
        };

        pos = pos.next(&new_instruction);
        frontier.push(pos.clone());
    }

    shoelace_formula(&frontier) as u64
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, process_input};

    #[test]
    fn part1_works() {
        let test_string = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)".to_string();
        assert_eq!(part1(&process_input(test_string)), 62);
    }

    #[test]
    fn part2_works() {
        let test_string = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)".to_string();
        assert_eq!(part2(&process_input(test_string)), 952408144115);
    }
}