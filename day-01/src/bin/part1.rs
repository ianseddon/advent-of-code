fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
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
    let first_index = line.find(|c: char| c.is_numeric());
    let last_index = line.rfind(|c: char| c.is_numeric());

    let char1 = line.chars().nth(first_index?)?;
    let char2 = line.chars().nth(last_index?)?;

    let digit1 = char1.to_digit(10)?;
    let digit2 = char2.to_digit(10)?;

    Some((10 * digit1) + digit2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, "142");
    }
}