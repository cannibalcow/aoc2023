advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(parse_line).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(parse_line_2)
            .map(|line| parse_line(&line))
            .sum(),
    )
}

pub fn parse_line(str: &str) -> u32 {
    let result: Vec<char> = str.chars().filter(|&ch| ch.is_numeric()).collect();

    format!(
        "{}{}",
        result.first().unwrap_or(&'0'),
        result.last().unwrap_or(&'0')
    )
    .parse::<u32>()
    .unwrap()
}

pub fn parse_line_2(str: &str) -> String {
    str.replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "tree3tree")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }
}
