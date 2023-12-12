use std::fs;
use std::str::FromStr;
use regex::Regex;

#[derive(PartialEq, Debug)]
struct Position {
    i: i32,
    j: i32,
}

impl Position {
    fn up(&self) -> Self {
        Position {i: self.i - 1, j: self.j}
    }
    fn down(&self) -> Self {
        Position {i: self.i + 1, j: self.j}
    }
    fn left(&self) -> Self {
        Position {i: self.i, j: self.j - 1}
    }
    fn right(&self) -> Self {
        Position {i: self.i, j: self.j + 1}
    }
}

struct Pipe {
    pos: Position,
    pipe_type: PipeType,
}

impl Pipe {
    fn find_next(&self, content: &String) -> Pipe {
        let position = match self.pipe_type {
            PipeType::Vertical => self.pos.down(),
            PipeType::Horizontal => self.pos.right(),
            PipeType::NorthEast => self.pos.right(),
            PipeType::NorthWest => self.pos.left(),
            PipeType::SouthWest => self.pos.down(),
            PipeType::SouthEast => self.pos.down(),
        };

        return Pipe {
            pipe_type: PipeType::from_str(content.get(&position).unwrap().as_str()).unwrap(),
            pos: position,
        };

    }

    fn find_other_next(&self, content: &String) -> Pipe {
        let position = match self.pipe_type {
            PipeType::Vertical => self.pos.up(),
            PipeType::Horizontal => self.pos.left(),
            PipeType::NorthEast => self.pos.up(),
            PipeType::NorthWest => self.pos.up(),
            PipeType::SouthWest => self.pos.left(),
            PipeType::SouthEast => self.pos.right(),
        };

        return Pipe {
            pipe_type: PipeType::from_str(content.get(&position).unwrap().as_str()).unwrap(),
            pos: position,
        };

    }
}

enum PipeType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl FromStr for PipeType {
    type Err = ();

    fn from_str(s: &str) -> Result<PipeType, ()> {
        match s {
            "|" => Ok(PipeType::Vertical),
            "-" => Ok(PipeType::Horizontal),
            "L" => Ok(PipeType::NorthEast),
            "J" => Ok(PipeType::NorthWest),
            "7" => Ok(PipeType::SouthWest),
            "F" => Ok(PipeType::SouthEast),
            _ => Err(())
        }
    }
}

impl ToString for PipeType {
    fn to_string(&self) -> String {
        match self {
            PipeType::Vertical => "|",
            PipeType::Horizontal => "-",
            PipeType::NorthEast => "L",
            PipeType::NorthWest => "J",
            PipeType::SouthWest => "7",
            PipeType::SouthEast => "F"
        }.to_string()
    }
}

trait PositionStr {
    fn get(&self, pos: &Position) -> Option<String>;
    fn set(&mut self, pos: &Position, str: String);
}

impl PositionStr for String {
    fn get(&self, pos: &Position) -> Option<String> {
        let lines: Vec<&str> = self.split("\n").collect();
        if 0 <= pos.i && pos.i < lines.len() as i32 && 0 <= pos.j && pos.j < lines[0].len() as i32 {
            return Some((lines[pos.i as usize].as_bytes()[pos.j as usize] as char).to_string());
        }
        None
    }

    fn set(&mut self, pos: &Position, str: String) {
        let n_col: i32 = self.split("\n").collect::<Vec<&str>>()[0].len() as i32;
        self.replace_range((pos.i * (n_col + 1) + pos.j) as usize..(pos.i * (n_col + 1) + pos.j + 1) as usize, &*str);
    }
}

fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");
    println!("Day 10");

    let data = process_input(&contents);

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data, &contents));
}

fn find_start_position(content: &String) -> (Position, String) {
    let lines: Vec<&str> = content.split("\n").collect();
    let start_re = Regex::new(r"(S)").unwrap();

    for i in 0..lines.len() {
        let start_m = start_re.find(lines[i]);

        if start_m.is_some() {
            let start_m = start_m.unwrap();
            let pos = Position {i: i as i32, j: start_m.start() as i32 };

            let coming_from_left = if pos.j == 0 { false } else { content.get(&pos.left()).unwrap() == "-" || content.get(&pos.left()).unwrap() == "L" || content.get(&pos.left()).unwrap() == "F" };
            let coming_from_right = if pos.j == (lines[i].len() - 1) as i32 { false } else { content.get(&pos.right()).unwrap() == "-" || content.get(&pos.right()).unwrap() == "7" || content.get(&pos.right()).unwrap() == "J"};
            let coming_from_top = if pos.i == 0 { false } else { content.get(&pos.up()).unwrap() == "|" || content.get(&pos.up()).unwrap() == "7" || content.get(&pos.up()).unwrap() == "F" };
            let coming_from_bottom = if pos.i == (lines.len() - 1) as i32 { false } else { content.get(&pos.down()).unwrap() == "|" || content.get(&pos.down()).unwrap() == "J" || content.get(&pos.down()).unwrap() == "L" };

            if coming_from_left && coming_from_right {
                return (pos, content.replace("S", "-")); // horizontal
            } else if coming_from_top && coming_from_bottom {
                return (pos, content.replace("S", "|")); // vertical
            } else if coming_from_top && coming_from_right {
                return (pos, content.replace("S", "L")); // north and east
            } else if coming_from_top && coming_from_left {
                return (pos, content.replace("S", "J")); // north and west
            } else if coming_from_bottom && coming_from_right {
                return (pos, content.replace("S", "F")); // south and west
            } else if coming_from_bottom && coming_from_left {
                return (pos, content.replace("S", "7")); // south and east
            }
        }
    }

    panic!("No start find");
}

fn process_input(content: &String) -> Vec<Pipe> {
    let (start, content) = find_start_position(content);

    let mut graph: Vec<Pipe> = vec![];
    graph.push(Pipe {
        pipe_type: PipeType::from_str(content.get(&start).unwrap().as_str()).unwrap(),
        pos: start,
    });

    while graph.len() == 1 || graph[graph.len() - 1].pos != graph[0].pos {

        if graph.len() == 1 {
            graph.push(graph[graph.len() - 1].find_next(&content));
        } else {
            let prev = &graph[graph.len() - 2];
            let current = &graph[graph.len() - 1];

            let next = current.find_next(&content);

            if prev.pos != next.pos {
                graph.push(next);
            } else {
                graph.push(current.find_other_next(&content));
            }
        }
    }

    graph.pop(); // remove last

    graph
}

fn part1(graph: &Vec<Pipe>) -> i32 {
    ((graph.len() + 1) / 2) as i32
}

fn part2(graph: &Vec<Pipe>, content: &String) -> i32 {

    let mut content = content.clone();

    for p in graph {
        content.set(&p.pos, "x".to_string());
    }
    for c in ['-', '|', 'L', '7', 'F', 'J'] {
        content = content.replace(c, ".");
    }
    for p in graph {
        content.set(&p.pos, p.pipe_type.to_string());
    }
    let mut sum = 0;
    let mut result: String = "".to_string();

    for line in content.split("\n").collect::<Vec<&str>>() {
        let mut parity = 0;
        let mut last_dir_change = -1; // 0 up 1 down -1 no

        for c in line.as_bytes() {
            let c = *c as char;

            match c {
                '|' => {
                    parity += 1;
                    result.push('│');
                },
                'L' => {
                    last_dir_change = 0;
                    result.push('└');
                },
                '7' => {
                    if last_dir_change == 0 {
                        parity += 1;
                    }
                    last_dir_change = -1;
                    result.push('┐');
                },
                'F' => {
                    last_dir_change = 1;
                    result.push('┌');
                },
                'J' => { // ↲
                    if last_dir_change == 1 {
                        parity += 1;
                    }
                    last_dir_change = -1;
                    result.push('┘');
                },
                '-' => {
                    result.push('─');
                }
                '.' => {
                    if parity % 2 == 1 {
                        sum += 1;
                        result.push('~');
                    } else {
                        result.push(' ');
                    }
                },
                _ => { result.push(' '); }
            }


        }

        result.push('\n');
    }
    println!("{}", result);

    sum
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, process_input};

    #[test]
    fn part1_works() {
        let test_string = ".....
.S-7.
.|.|.
.L-J.
.....".to_string();
        assert_eq!(part1(&process_input(&test_string)), 4);
    }

    #[test]
    fn part1_works2() {
        let test_string = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...".to_string();
        assert_eq!(part1(&process_input(&test_string)), 8);
    }

    #[test]
    fn part1_works3() {
        let test_string = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF".to_string();
        assert_eq!(part1(&process_input(&test_string)), 4);
    }

    #[test]
    fn part1_works4() {
        let test_string = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ".to_string();
        assert_eq!(part1(&process_input(&test_string)), 8);
    }

    #[test]
    fn part2_works() {
        let test_string = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........".to_string();

        assert_eq!(part2(&process_input(&test_string),&test_string), 4);
    }

    #[test]
    fn part2_works2() {
        let test_string = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...".to_string();

        assert_eq!(part2(&process_input(&test_string),&test_string), 8);
    }

    #[test]
    fn part2_works3() {
        let test_string = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L".to_string();

        assert_eq!(part2(&process_input(&test_string),&test_string), 10);
    }
}