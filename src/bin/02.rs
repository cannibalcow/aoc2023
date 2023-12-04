use nom::{
    bytes::complete::{is_not, tag, take_till, take_until},
    character::{
        complete::{alphanumeric0, digit0, digit1},
        is_digit,
    },
    sequence::delimited,
    IResult, InputTakeAtPosition,
};

advent_of_code::solution!(2);

#[allow(dead_code)]
struct Game {
    id: usize,
    green: u32,
    blue: u32,
    red: u32,
}

#[allow(dead_code)]
enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[allow(dead_code, unused_variables)]
pub fn part_one(input: &str) -> Option<u32> {
    None
}

#[allow(dead_code, unused_variables)]
pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[allow(dead_code)]
fn game_id_parser(input: &str) -> IResult<&str, &str> {
    let (a, b) = take_until(":")(input)?;
    println!("a: {}", a);
    println!("b: {}", b);
    let (c, d) = take_till(|c| c.is_digit())(b)?;
    println!("c: {:?}", c);
    println!("d: {:?}", d);

    let e = digit0(c)?;

    println!("e: {:?}", e);

    digit0(b)
}

#[allow(dead_code, unused_variables)]
fn parse_line(input: &str) -> Game {
    Game {
        id: 1,
        green: 1,
        blue: 1,
        red: 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game_id() {
        let result = game_id_parser(
            "Game 1: 1 red, 5 blue, 1 green; 16 blue, 3 red; 6 blue, 5 red; 4 red, 7 blue, 1 green",
        );
        match result {
            Ok(v) => println!("FUck: {:?}", v),
            Err(e) => println!("Error: {:?}", e),
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
