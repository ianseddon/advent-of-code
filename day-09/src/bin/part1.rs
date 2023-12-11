fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    // Split the input into lines
    let lines: std::str::Lines<'_> = input.lines();

    let mut sum: i64 = 0;
    for line in lines {
        let line_sum = process(line).unwrap_or(0);
        dbg!(line_sum);
        sum += line_sum;
    }

    return sum.to_string();
}

fn process(line: &str) -> Option<i64> {
    // Split the line into numbers
    let numbers: Vec<i64> = parse_line(line);
    let mut last_numbers: Vec<i64> = Vec::new();
    last_numbers.push(*numbers.last().expect("No last number"));

    // Compute the difference until we get 0
    let mut difference: Vec<i64> = compute_difference(numbers);

    while difference.last() != Some(&0) {
        last_numbers.push(*difference.last().expect("No last number"));
        difference = compute_difference(difference);
    }

    last_numbers.push(*difference.last().expect("No last number"));

    let mut next_history: Vec<i64> = Vec::new();
    last_numbers.iter().rev().for_each(|number| {
        next_history.push(*number + next_history.last().unwrap_or(&0));
    });

    next_history.last().copied()
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|number| {
            number.parse::<i64>().unwrap()
        }).collect()
}

fn compute_difference(line: Vec<i64>) -> Vec<i64> {
    line.windows(2)
        .map(|window| {
            window[1] - window[0]
        }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "10  13  16  21  30  45
0   3   6   9  12  15
1   3   6  10  15  21"
        );
        assert_eq!(result, "114");
    }

    #[test]
    fn test_compute_difference() {
        let result = compute_difference(
            vec![10, 13, 16, 21, 30, 45]
        );
        assert_eq!(result, vec![3, 3, 5, 9, 15]);
    }
}