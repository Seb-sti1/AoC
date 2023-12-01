use std::fs;
use regex::{Regex};


fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");

    println!("Day 1");
    println!("Part 1: {}", part1(contents.clone()));
    println!("Part 2: {}", part2(contents));
}

fn part1(contents: String) -> i32 {
    let mut calibration_value_sum = 0;

    for line in contents.split("\n") {
        let re: Regex = Regex::new(r"(\d)").unwrap();
        let digits: Vec<&str> = re.find_iter(&*line).map(|m| m.as_str()).collect();

        if digits.len() > 0 {
            let first = digits.get(0).unwrap().parse::<i32>().unwrap();
            let last = digits.get(digits.len() - 1).unwrap().parse::<i32>().unwrap();

            calibration_value_sum += first * 10 + last;
        }
    }

    calibration_value_sum
}

fn find(line: &str, digits: [&str; 18], from_beginning: bool) -> i32 {
    let find = digits.clone().map(|digit| if from_beginning { line.find(digit) } else { line.rfind(digit)});
    let mut first_digit_idx: usize = 0;

    for pattern_idx in 0..find.len() {
        let f = find[pattern_idx];
        let candidate = find[first_digit_idx];

        if f.is_some() &&
            (candidate.is_none()
                || (from_beginning && f.unwrap() < candidate.unwrap())
                || (!from_beginning && f.unwrap() > candidate.unwrap())) {
            first_digit_idx = pattern_idx;
        }
    }

    if find[first_digit_idx].is_some() {
        return first_digit_idx as i32;
    }

    -1
}

fn part2(contents: String) -> i32 {
    let mut calibration_value_sum = 0;
    let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    for line in contents.split("\n") {
        // find the first digit of the string
        let first_digit_idx = find(line, digits, true);

        // find the last digit of the string
        let last_digit_idx = find(line, digits, false);

        let first = if first_digit_idx >= 0 { (first_digit_idx % 9) + 1 } else { -1 };
        let last = if last_digit_idx >= 0 { (last_digit_idx % 9) + 1 } else { -1 };

        calibration_value_sum += first * 10 + last;
    }

    calibration_value_sum
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn part1_works() {
        let test_string = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet".to_string();
        assert_eq!(part1(test_string), 142);
    }

    #[test]
    fn part2_works() {
        let test_string = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen".to_string();
        assert_eq!(part2(test_string), 281);
    }
}