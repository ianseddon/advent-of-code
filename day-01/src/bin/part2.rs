fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    // Split the input into lines
    let lines: std::str::Lines<'_> = input.lines();

    let mut sum: u32 = 0;
    for line in lines {
        let line_sum: u32 = parse_line(line).unwrap_or(0);
        sum += line_sum;
    }

    return sum.to_string();
}

fn parse_line(line: &str) -> Option<u32> {
    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;
    let mut first_pos: usize = line.len();
    let mut last_pos: usize = 0;

    let words: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for (i, &word) in words.iter().enumerate() {
        if let Some(pos) = line.find(word) {
            if pos <= first_pos {
                first_pos = pos;
                first_digit = Some((i + 1) as u32);
            }
        }
        if let Some(pos) = line.rfind(word) {
            if pos >= last_pos {
                last_pos = pos;
                last_digit = Some((i + 1) as u32);
            }
        }
    }

    // Now check for numeric characters
    for (i, c) in line.chars().enumerate() {
        if c.is_numeric() {
            if i <= first_pos {
                first_pos = i;
                first_digit = Some(c.to_digit(10)?);
            }
            if i >= last_pos {
                last_pos = i;
                last_digit = Some(c.to_digit(10)?);
            }
        }
    }

    Some((10 * first_digit?) + last_digit?)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, "281");
    }
}