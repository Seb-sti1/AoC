use std::fs;
use std::time::Instant;
use regex::Regex;


struct Input {
    sequences: Vec<String>,
}

struct Lens {
    label: String,
    focal: u64,
}

struct Slot {
    lenses: Vec<Lens>,
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

fn hash(s: &String) -> u64 {
    let mut h: u64 = 0;

    for i in 0..s.len() {
        let c = s.as_bytes()[i];

        h += c as u64;
        h = (h * 17) % 256;
    }

    h
}


fn process_input(content: String) -> Input {
    let t: Vec<String> = content.split(",").collect::<Vec<&str>>().iter().map(|s| s.to_string()).collect();
    return Input { sequences: t };
}

fn part1(input: &Input) -> u64 {
    let mut r: u64 = 0;

    for seq in &input.sequences {
        r += hash(seq);
    }

    r
}


fn part2(input: &Input) -> u64 {
    let mut slots: Vec<Slot> = vec![];
    for _ in 0..256 {
        slots.push(Slot { lenses: vec![] });
    }


    'seq: for seq in &input.sequences {
        let seq_regex = Regex::new("([a-z]+)([=-])(\\d?)").unwrap();

        let m = seq_regex.captures(seq).unwrap();

        let label = m.get(1).unwrap().as_str().to_string();
        let slot = hash(&label) as usize;
        let ope = m.get(2).unwrap().as_str();

        if ope.eq("=") {
            let focal = m.get(3).unwrap().as_str().parse::<u64>().unwrap();

            for (i, lens) in slots[slot].lenses.iter().enumerate() {
                if lens.label == label {
                    slots[slot].lenses.remove(i);
                    slots[slot].lenses.insert(i, Lens { label, focal });
                    continue 'seq;
                }
            }

            slots[slot].lenses.push(Lens { label, focal });
        } else {
            for (i, lens) in slots[slot].lenses.iter().enumerate() {
                if lens.label == label {
                    slots[slot].lenses.remove(i);
                    break;
                }
            }
        }
    }

    let mut r = 0;

    for (i, s) in slots.iter().enumerate() {
        for (j, lens) in s.lenses.iter().enumerate() {
            r += (i as u64 + 1) * (j as u64 + 1) * lens.focal;
        }
    }

    r
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, process_input};

    #[test]
    fn part1_works() {
        let test_string = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string();
        assert_eq!(part1(&process_input(test_string)), 1320);
    }

    #[test]
    fn part1_works1() {
        let test_string = "HASH".to_string();
        assert_eq!(part1(&process_input(test_string)), 52);
    }

    #[test]
    fn part1_works2() {
        let test_string = "rn=1".to_string();
        assert_eq!(part1(&process_input(test_string)), 30);
    }

    #[test]
    fn part2_works() {
        let test_string = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string();
        assert_eq!(part2(&process_input(test_string)), 145);
    }

    #[test]
    fn part2_works1() {
        let test_string = "rn=1".to_string();
        assert_eq!(part2(&process_input(test_string)), 1);
    }
}