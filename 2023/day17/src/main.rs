use std::fs;
use std::time::Instant;
use pathfinding::prelude::dijkstra;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
    None,
}


#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    i: i32,
    j: i32,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    pos: Pos,
    dir: Direction,
    dist_in_dir: u32,
}


impl Node {
    fn successors(&self, n: usize, m: usize, max_in_same_dir: u32, min_in_same_dir: u32) -> Vec<Node> {
        vec![Node { pos: Pos { i: self.pos.i + 1, j: self.pos.j }, dir: Direction::Down, dist_in_dir: if self.dir == Direction::Down { self.dist_in_dir + 1 } else { 1 } },
             Node { pos: Pos { i: self.pos.i, j: self.pos.j - 1 }, dir: Direction::Left, dist_in_dir: if self.dir == Direction::Left { self.dist_in_dir + 1 } else { 1 } },
             Node { pos: Pos { i: self.pos.i, j: self.pos.j + 1 }, dir: Direction::Right, dist_in_dir: if self.dir == Direction::Right { self.dist_in_dir + 1 } else { 1 } },
             Node { pos: Pos { i: self.pos.i - 1, j: self.pos.j }, dir: Direction::Up, dist_in_dir: if self.dir == Direction::Up { self.dist_in_dir + 1 } else { 1 } }]
            .into_iter().filter(|node| node.pos.i >= 0 && node.pos.i < n as i32) // remove out of range
            .into_iter().filter(|node| node.pos.j >= 0 && node.pos.j < m as i32) // remove out of range
            .into_iter().filter(|node| { // remove direction modification that aren't 0° or +- 90 °
            return match self.dir {
                Direction::Up => { node.dir != Direction::Down }
                Direction::Down => { node.dir != Direction::Up }
                Direction::Right => { node.dir != Direction::Left }
                Direction::Left => { node.dir != Direction::Right }
                Direction::None => { true }
            };
        }).into_iter().filter(|node| node.dist_in_dir <= max_in_same_dir) // can't go too long in straight line
            .into_iter().filter(|node| self.dist_in_dir >= min_in_same_dir || node.dir == self.dir || self.dir == Direction::None
        ) // can't be too short in straight line
            .collect()
    }
}

struct Input {
    map: Vec<Vec<i32>>,
}

impl Input {
    fn get(&self, pos: &Pos) -> i32 {
        return self.map[pos.i as usize][pos.j as usize];
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

fn process_input(content: String) -> Input {
    let mut map: Vec<Vec<i32>> = vec![];

    for line in content.lines() {
        let mut l: Vec<i32> = vec![];

        for c in line.as_bytes() {
            let v = (*c as char).to_string().parse::<i32>().unwrap();
            l.push(v);
        }

        map.push(l);
    }

    return Input { map };
}

fn part1(input: &Input) -> u64 {
    let n = input.map.len();
    let m = input.map[0].len();


    let result = dijkstra(&Node { pos: Pos { i: 0, j: 0 }, dir: Direction::None, dist_in_dir: 0 },
                          |node| node.successors(n, m, 3, 0).into_iter().map(|n| {
                              let v = input.get(&n.pos);
                              (n, v)
                          }),
                          |node| node.pos == Pos { i: n as i32 - 1, j: m as i32 - 1 }).expect("no path found");

    result.1 as u64
}


fn part2(input: &Input) -> u64 {
    let n = input.map.len();
    let m = input.map[0].len();

    let min_in_same_dir: i32 = 4;

    let result = dijkstra(&Node { pos: Pos { i: 0, j: 0 }, dir: Direction::None, dist_in_dir: 0 },
                          |node| node.successors(n, m, 10, min_in_same_dir as u32).into_iter().map(|n| {
                              let v = input.get(&n.pos);
                              (n, v)
                          }),
                          |node| node.pos == Pos { i: n as i32 - 1 - min_in_same_dir, j: m as i32 - 1 }
                              || node.pos == Pos { i: n as i32 - 1, j: m as i32 - 1 - min_in_same_dir }).expect("no path found");

    let mut path = result.0;
    let mut dist = result.1;

    if path.last().unwrap().pos == (Pos { i: n as i32 - 1 - min_in_same_dir, j: m as i32 - 1 }) {
        for i in n as i32 - min_in_same_dir..n as i32 {
            path.push(Node { pos: Pos { i, j: m as i32 - 1 }, dir: Direction::Down, dist_in_dir: 1 });
            dist += input.get(&Pos { i, j: m as i32 - 1 });
        }
    } else {
        for j in m as i32 - min_in_same_dir..m as i32 {
            path.push(Node { pos: Pos { i: n as i32 - 1, j }, dir: Direction::Right, dist_in_dir: 1 });
            dist += input.get(&Pos { i: n as i32 - 1, j });
        }
    }

    for i in 0..n as i32 {
        for j in 0..m as i32 {
            let mut found = false;

            if (Pos { i, j }) == (Pos { i: n as i32 - 1 - min_in_same_dir, j: m as i32 - 1 }) {
                print!("o");
                continue;
            }

            if (Pos { i, j }) == (Pos { i: n as i32 - 1, j: m as i32 - 1 - min_in_same_dir }) {
                print!(".");
                continue;
            }

            for n in &path {
                if n.pos == (Pos { i, j }) {
                    found = true;
                    print!("{}", match n.dir {
                        Direction::Up => {
                            "^"
                        }
                        Direction::Down => {
                            "v"
                        }
                        Direction::Right => {
                            ">"
                        }
                        Direction::Left => {
                            "<"
                        }
                        Direction::None => {
                            "x"
                        }
                    });
                }
            }

            if !found {
                print!("{}", input.get(&Pos { i, j }))
            }
        }
        println!()
    }


    dist as u64
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, process_input};

    #[test]
    fn part1_works() {
        let test_string = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533".to_string();
        assert_eq!(part1(&process_input(test_string)), 102);
    }

    #[test]
    fn part2_works() {
        let test_string = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533".to_string();
        assert_eq!(part2(&process_input(test_string)), 94);
    }

    #[test]
    fn part2_works2() {
        let test_string = "111111111111
999999999991
999999999991
999999999991
999999999991".to_string();
        assert_eq!(part2(&process_input(test_string)), 71);
    }
}