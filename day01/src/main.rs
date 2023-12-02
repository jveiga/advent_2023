fn main() {
    let input_1 = include_str!("../input1.txt");
    let mut sum = 0;
    for line in input_1.lines() {
        let (first_digit, last_digit) = (first_digit_line(line), last_digit_line(line));
        sum += first_digit * 10 + last_digit;
    }
    println!("{sum}");
    let sum = part2(input_1);

    println!("{sum}");
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut first_found = false;
        let mut vals = 0;
        for first_idx in 0..line.len() {
            if let Ok(n) = line[first_idx..first_idx + 1].parse::<u32>() {
                vals = n;
                if !first_found {
                    first_found = true;
                    sum += n * 10;
                }
            }
            for last_idx in first_idx..line.len() + 1 {
                if let Some(n) = string_to_digit(&line[first_idx..last_idx]) {
                    vals = n;
                    if !first_found {
                        first_found = true;
                        // println!("{sum} += {}", n * 10);
                        sum += n * 10;
                    }
                }
            }
        }

        // println!("{sum} += {vals}");
        sum += vals;
    }
    sum
}

fn string_to_digit(input: &str) -> Option<u32> {
    match input {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn first_digit_line(line: &str) -> u32 {
    line.chars().find_map(|c| c.to_digit(10)).unwrap()
}

fn last_digit_line(line: &str) -> u32 {
    line.chars().rev().find_map(|c| c.to_digit(10)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_digit_in_line() {
        let s1 = "1abc2";
        assert_eq!(first_digit_line(s1), 1);
        let s2 = "pqr3stu8vwx";
        assert_eq!(first_digit_line(s2), 3);
        let s3 = "a1b2c3d4e5f";
        assert_eq!(first_digit_line(s3), 1);
        let s4 = "treb7uchet";
        assert_eq!(first_digit_line(s4), 7);
    }

    #[test]
    fn last_digit_in_line() {
        let s1 = "1abc2";
        assert_eq!(last_digit_line(s1), 2);
        let s2 = "pqr3stu8vwx";
        assert_eq!(last_digit_line(s2), 8);
        let s3 = "a1b2c3d4e5f";
        assert_eq!(last_digit_line(s3), 5);
        let s4 = "treb7uchet";
        assert_eq!(last_digit_line(s4), 7);
    }

    #[test]
    fn test_string_to_digit() {
        let mini_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(part2(mini_input), 281);
    }
}
