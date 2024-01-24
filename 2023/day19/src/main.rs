use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::ops::Range;
use std::str::FromStr;
use std::time::Instant;
use regex::Regex;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Operation {
    Greater,
    Less,
}

impl Operation {
    fn apply(&self, a: i32, b: i32) -> bool {
        return match self {
            Operation::Greater => {
                a > b
            }
            Operation::Less => {
                a < b
            }
        };
    }

    fn get_apply_range(&self, r: &Range<i32>, n: &i32) -> Option<Range<i32>> {
        return match self {
            Operation::Greater => {
                // [n + 1, r.end[
                if r.end - 1 <= *n {
                    return None;
                }
                Some((*n + 1)..r.end)
            }
            Operation::Less => {
                // [r.start, n - 1]
                if r.start >= *n {
                    return None;
                }
                Some(r.start..*n)
            }
        };
    }

    fn get_else_range(&self, r: &Range<i32>, n: &i32) -> Option<Range<i32>> {
        return match self {
            Operation::Greater => {
                // [r.start, n]
                if r.start > *n {
                    return None;
                }
                Some(r.start..(*n + 1))
            }
            Operation::Less => {
                // [n, r.end[
                if *n > r.end - 1 {
                    return None;
                }
                Some(*n..r.end)
            }
        };
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Name {
    Name(String),
    Accepted,
    Rejected,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Ins {
    Cond(String, Operation, i32, Name),
    Default(Name),
}

impl FromStr for Ins {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name_re: Regex = Regex::new(r"^([a-z]+)$").unwrap();
        let name = name_re.captures(s);

        let ins_re: Regex = Regex::new(r"^([a-z])([<>])(\d+):([a-zAR]+)$").unwrap();
        let ins = ins_re.captures(s);//a<2006:qkq,m>2090:A


        if s == "A" {
            return Ok(Ins::Default(Name::Accepted));
        } else if s == "R" {
            return Ok(Ins::Default(Name::Rejected));
        } else if name.is_some() {
            return Ok(Ins::Default(Name::Name(name.unwrap().get(1).unwrap().as_str().to_string())));
        } else if ins.is_some() {
            let ins = ins.unwrap();
            let result: Name = if ins.get(4).unwrap().as_str().to_string() == "A" { Name::Accepted } else if ins.get(4).unwrap().as_str().to_string() == "R" { Name::Rejected } else { Name::Name(ins.get(4).unwrap().as_str().to_string()) };

            return Ok(Ins::Cond(ins.get(1).unwrap().as_str().to_string(),
                                if ins.get(2).unwrap().as_str() == ">" { Operation::Greater } else { Operation::Less },
                                ins.get(3).unwrap().as_str().to_string().parse::<i32>().unwrap(),
                                result,
            ));
        }

        return Err(());
    }
}

#[derive(Clone, Debug)]
struct Process {
    name: String,
    ins: Vec<Ins>,
}

#[derive(Clone, Debug)]
struct ProcessResult {
    next: Name,
    range: HashMap<String, Range<i32>>,
}

impl Process {
    fn exec(&self, v: &HashMap<String, i32>) -> Name {
        for ins in &self.ins {
            match ins {
                Ins::Cond(var, ope, n, result) => {
                    if ope.apply(*v.get(var).unwrap(), *n) {
                        return result.clone();
                    }
                }
                Ins::Default(result) => {
                    return result.clone();
                }
            }
        }

        return Name::Rejected;
    }

    fn exec_range(&self, v: &ProcessResult) -> Vec<ProcessResult> {
        let mut results: Vec<ProcessResult> = vec![];

        let mut todo: Vec<ProcessResult> = vec![v.clone()];

        for ins in &self.ins {
            let mut next_todo: Vec<ProcessResult> = vec![];

            for r in todo {
                match ins {
                    Ins::Cond(var, ope, n, result) => {
                        let apply_range = ope.get_apply_range(r.range.get(var).unwrap(), n);
                        let else_range = ope.get_else_range(r.range.get(var).unwrap(), n);

                        if apply_range.is_some() {
                            let mut range = r.range.clone();
                            range.insert(var.to_string(), apply_range.unwrap());
                            results.push(ProcessResult { next: result.clone(), range })
                        }

                        if else_range.is_some() {
                            let mut range = r.range.clone();
                            range.insert(var.to_string(), else_range.unwrap());
                            next_todo.push(ProcessResult { next: Name::Rejected, range })
                        }
                    }
                    Ins::Default(result) => {
                        results.push(ProcessResult { next: result.clone(), range: r.range })
                    }
                }
            }
            todo = next_todo;
        }

        results
    }
}

impl FromStr for Process {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line_re: Regex = Regex::new(r"([a-z]+)\{(.+)}").unwrap();
        let line = line_re.captures(s).unwrap();

        return Ok(Process {
            name: line.get(1).unwrap().as_str().to_string(),
            ins: line.get(2).unwrap().as_str().to_string().split(",").map(|s| Ins::from_str(s).unwrap()).collect(),
        });
    }
}


struct Input {
    processes: Vec<Process>,
    values: Vec<HashMap<String, i32>>,
}

fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");

    let data = process_input(contents);

    let t1 = Instant::now();
    let p1 = part1(&data);
    let t1 = t1.elapsed();
    println!("Part 1 (in {}s): {} ", t1.as_nanos() as f32 / 1_000_000_000f32, p1);
    assert_eq!(p1, 492702);

    let t2 = Instant::now();
    let p2 = part2(&data);
    let t2 = t2.elapsed();
    println!("Part 2 (in {}s): {} ", t2.as_nanos() as f32 / 1_000_000_000f32, p2);
}

fn process_input(content: String) -> Input {
    let mut processes: Vec<Process> = vec![];
    let mut values: Vec<HashMap<String, i32>> = vec![];

    let mut part: bool = true;

    for line in content.lines() {
        if part {
            if line == "" {
                part = false;
            } else {
                processes.push(Process::from_str(line).unwrap());
            }
        } else {
            let mut hashmap: HashMap<String, i32> = HashMap::new();

            line.replace("{", "").replace("}", "").split(",").for_each(|s| {
                let var = s[0..1].to_string();
                let n = s.replace(&s[0..1], "").replace("=", "").parse::<i32>().unwrap();
                hashmap.insert(var, n);
            });

            values.push(hashmap);
        }
    }

    return Input { processes, values };
}

fn part1(input: &Input) -> u64 {
    let mut r = 0;

    let mut i = 0;

    for values in &input.values {
        let mut current_process = input.processes.iter().find(|p| p.name == "in");

        'exec: while current_process.is_some() {
            let process = current_process.unwrap();
            i += process.ins.len() - 1;
            let result: Name = process.exec(&values);

            match result {
                Name::Name(name) => {
                    current_process = input.processes.iter().find(|p| p.name == name);
                }
                Name::Accepted => {
                    r += values.iter().map(|v| v.1).sum::<i32>();
                    break 'exec;
                }
                Name::Rejected => {
                    break 'exec;
                }
            }
        }
    }

    println!("{}", i);

    r as u64
}

fn part2(input: &Input) -> u64 {
    let mut hashmap: HashMap<String, Range<i32>> = HashMap::new();
    hashmap.insert("x".to_string(), 1..4001);
    hashmap.insert("m".to_string(), 1..4001);
    hashmap.insert("a".to_string(), 1..4001);
    hashmap.insert("s".to_string(), 1..4001);

    let mut r = 0;

    let mut values: Vec<ProcessResult> = vec![ProcessResult { next: Name::Name("in".to_string()), range: hashmap }];

    while values.len() > 0 {
        let mut new_value: Vec<ProcessResult> = vec![];

        for value in values {
            let next = value.next.clone();

            match next {
                Name::Name(name) => {
                    let process = input.processes.iter().find(|p| p.name == name).unwrap();
                    new_value.append(&mut process.exec_range(&value));
                }
                Name::Accepted => {
                    r += value.range.iter().map(|v| v.1.len() as u64).product::<u64>();
                }
                _ => {}
            }
        }

        values = new_value;
    }

    r
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, process_input};

    #[test]
    fn part1_works() {
        let test_string = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}".to_string();
        assert_eq!(part1(&process_input(test_string)), 19114);
    }

    #[test]
    fn part2_works() {
        let test_string = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}".to_string();
        assert_eq!(part2(&process_input(test_string)), 167409079868000);
    }

    #[test]
    fn part2_works2() {
        let test_string = "in{s<1351:A,R}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}".to_string();
        assert_eq!(part2(&process_input(test_string)), 4000 * 4000 * 4000 * 1350);
    }
}