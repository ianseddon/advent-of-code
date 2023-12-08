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
        dbg!(line_sum);
        sum += line_sum;
    }

    return sum.to_string();
}


fn parse_line(line: &str) -> Option<u32> {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    let (_, games) = line.split_once(": ")?;
    let games: Vec<&str> = games.split("; ").collect();

    for game in games {
        let results = game.split(", ");
        for result in results {
            let (count, color) = result.split_once(" ")?;
            let count: u32 = count.parse().unwrap();

            if color == "red" && count > max_red {
                max_red = count;
            } else if color == "green" && count > max_green {
                max_green = count;
            } else if color == "blue" && count > max_blue {
                max_blue = count;
            }
        }
    }

    Some(max_red * max_green * max_blue)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "2286");
    }
}