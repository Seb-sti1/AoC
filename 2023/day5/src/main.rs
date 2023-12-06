use std::cmp::min;
use std::fs;
use std::str::FromStr;
use regex::{Regex};

#[derive(Debug)]
enum Type {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl FromStr for Type {
    type Err = ();
    fn from_str(input: &str) -> Result<Type, Self::Err> {
        match input {
            "seed" => Ok(Type::Seed),
            "soil" => Ok(Type::Soil),
            "fertilizer" => Ok(Type::Fertilizer),
            "water" => Ok(Type::Water),
            "light" => Ok(Type::Light),
            "temperature" => Ok(Type::Temperature),
            "humidity" => Ok(Type::Humidity),
            "location" => Ok(Type::Location),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Range {
    src: u64,
    dest: u64,
    len: u64,
}

impl Range {
    pub(crate) fn clone(&self) -> Range {
        Range {
            src: self.src,
            dest: self.dest,
            len: self.len,
        }
    }
}

struct Transformation {
    from: Type,
    to: Type,
    ranges: Vec<Range>,
}

fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");

    println!("Day 5");

    let cards = process_input(contents);

    println!("Part 1: {}", part1(&cards));
    println!("Part 2: {}", part2(&cards));
}

/// Process the input and convert it to a more intelligible structure
///
/// # Arguments
/// * `content` - the content of the input file
fn process_input(content: String) -> (Vec<u64>, Vec<Transformation>) {
    let lines: Vec<&str> = content.split("\n").collect();

    let seeds_re: Regex = Regex::new(r"seeds: ([\d ]+)").unwrap();
    let number_re: Regex = Regex::new(r"(\d+)").unwrap();
    let transformation_re: Regex = Regex::new(r"([a-z]*)-to-([a-z]*)").unwrap();
    let range_re: Regex = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();

    let mut transformations: Vec<Transformation> = vec![];

    //deals with first line
    let seeds = seeds_re.find(lines[0]).unwrap().as_str();
    let seeds: Vec<u64> = number_re.find_iter(seeds).map(|m| m.as_str().parse::<u64>().unwrap()).collect();

    let mut tr: Option<Transformation> = None;
    let mut ranges: Vec<Range> = vec![];

    for line_idx in 2..lines.len() {
        let line = lines[line_idx];

        let tr_captures = transformation_re.captures(line);

        if tr_captures.is_some() {
            let tr_captures = tr_captures.unwrap();

            if tr.is_some() {
                let mut tr = tr.unwrap();
                ranges.sort_by_key(|r| r.src);
                tr.ranges = ranges;
                ranges = vec![];

                transformations.push(tr);
            }

            tr = Option::from(Transformation {
                from: Type::from_str(tr_captures.get(1).unwrap().as_str()).unwrap(),
                to: Type::from_str(tr_captures.get(2).unwrap().as_str()).unwrap(),
                ranges: vec![],
            })
        } else if tr.is_some() {
            let range_captures = range_re.captures(line);

            if range_captures.is_some() {
                let range_captures = range_captures.unwrap();

                ranges.push(Range {
                    src: range_captures.get(2).unwrap().as_str().parse::<u64>().unwrap(),
                    dest: range_captures.get(1).unwrap().as_str().parse::<u64>().unwrap(),
                    len: range_captures.get(3).unwrap().as_str().parse::<u64>().unwrap(),
                })
            }
        }
    }

    // last transformation
    if tr.is_some() {
        let mut tr = tr.unwrap();
        ranges.sort_by_key(|r| r.src);
        tr.ranges = ranges;
        transformations.push(tr);
    }

    (seeds, transformations)
}

fn do_transformation(n: u64, t: &Transformation) -> u64 {
    let mut n_t = n;

    for r in &t.ranges {
        if r.src <= n && n < r.src + r.len {
            n_t = r.dest + (n - r.src);
            break;
        }
    }

    n_t
}

fn part1(data: &(Vec<u64>, Vec<Transformation>)) -> i32 {
    let (seeds, transformation) = data;

    let mut locations: Vec<u64> = vec![];

    for n in seeds {
        let n = *n;
        let mut location = n;

        for t in transformation {
            location = do_transformation(location, t);
        }

        locations.push(location);
    }

    *locations.iter().min().unwrap() as i32
}

fn do_range_transformation(r: Range, t: &Transformation) -> Vec<Range> {
    let mut ranges: Vec<Range> = Vec::from([r]);

    for tr in &t.ranges {
        let mut new_ranges: Vec<Range> = vec![];

        for r in ranges {

            // r     |-----------|
            // tr |----|
            // or
            // r     |-----------|
            // tr |---------------|
            if tr.src <= r.src && r.src < tr.src + tr.len {
                new_ranges.push(Range {
                    src: r.src,
                    dest: tr.dest + (r.src - tr.src),
                    len: min(tr.src + tr.len - r.src, r.len),
                });

                // r     |-----------|
                // tr |----|
                if tr.src + tr.len < r.src + r.len {
                    new_ranges.push(Range {
                        src: tr.src + tr.len,
                        dest: tr.src + tr.len,
                        len: r.src + r.len - (tr.src + tr.len),
                    });
                }
            }
            // r     |-----------|
            // tr      |----|
            // or
            // r     |-----------|
            // tr      |---------------|
            else if r.src < tr.src && tr.src < r.src + r.len {
                new_ranges.push(Range {
                    src: r.src,
                    dest: r.src,
                    len: tr.src - r.src + 1,
                });

                new_ranges.push(Range {
                    src: tr.src,
                    dest: tr.dest,
                    len: min(r.src + r.len - tr.src, tr.len),
                });

                // r     |-----------|
                // tr      |----|
                if tr.src + tr.len < r.src + r.len {
                    new_ranges.push(Range {
                        src: tr.src + tr.len,
                        dest: tr.src + tr.len,
                        len: r.src + r.len - (tr.src + tr.len),
                    });
                }
            } else {
                new_ranges.push(r);
            }
        }

        ranges = new_ranges;
    }

    ranges.into_iter().map(|r| Range {
        src: r.dest,
        dest: r.dest,
        len: r.len,
    }).collect()
}

fn part2(data: &(Vec<u64>, Vec<Transformation>)) -> i32 {
    let (seeds, transformation) = data;

    let mut ranges: Vec<Range> = vec![];

    // convert seeds to ranges
    for i in (0..seeds.len()).step_by(2) {
        ranges.push(Range {
            src: seeds[i],
            dest: seeds[i],
            len: seeds[i + 1],
        })
    }

    let mut next_ranges: Vec<Range> = vec![];

    for t in transformation {
        for range in ranges {
            next_ranges.append(&mut do_range_transformation(range.clone(), t));
        }

        ranges = next_ranges;
        next_ranges = vec![];
    }


    (*ranges.iter().map(|r| r.src).collect::<Vec<u64>>().iter().min().unwrap()) as i32
}

#[cfg(test)]
mod tests {
    use crate::{part1, process_input};
    use crate::part2;

    #[test]
    fn part1_works() {
        let test_string = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4".to_string();

        assert_eq!(part1(&process_input(test_string)), 35);
    }

    #[test]
    fn part2_works() {
        let test_string = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4".to_string();

        assert_eq!(part2(&process_input(test_string)), 46);
    }
}