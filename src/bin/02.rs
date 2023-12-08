use std::{collections::HashMap, ops::Add};

use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map_res, IResult,
};

advent_of_code::solution!(2);

#[derive(Debug)]
struct Game {
    id: usize,
    subsets: Vec<Vec<SubSet>>,
}

#[derive(Debug)]
enum SubSet {
    Red(usize),
    Green(usize),
    Blue(usize),
}

fn is_possible(subsets: &Vec<Vec<SubSet>>) -> bool {
    for subset in subsets {
        let mut sums: HashMap<&str, usize> = HashMap::new();
        for s in subset {
            match s {
                SubSet::Red(v) => sums.entry("red").or_insert(*v).add(*v),
                SubSet::Green(v) => sums.entry("green").or_insert(*v).add(*v),
                SubSet::Blue(v) => sums.entry("blue").or_insert(*v).add(*v),
            };
        }

        match sums.get("red") {
            Some(v) => {
                if *v > 12 {
                    return false;
                }
            }
            None => {}
        }

        match sums.get("green") {
            Some(v) => {
                if *v > 13 {
                    return false;
                }
            }
            None => {}
        }

        match sums.get("blue") {
            Some(v) => {
                if *v > 14 {
                    return false;
                }
            }
            None => {}
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let games: u32 = input
        .lines()
        .map(|line| parse_row(line))
        .filter(|game| is_possible(&game.subsets))
        .map(|game| game.id as u32)
        .sum();

    Some(games)
}

#[allow(dead_code, unused_variables)]
pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_game_id(input: &str) -> IResult<&str, Token> {
    let v = tag("Game ")(input)?;
    let v3 = map_res(digit1, |s: &str| s.parse::<usize>())(v.0)?;

    Ok((v3.0, Token::Game(v3.1)))
}

fn parse_color(input: &str) -> IResult<&str, Token> {
    let v = alt((tag("red"), tag("blue"), tag("green")))(input)?;
    let token = match v.1 {
        "red" => Token::Color(Color::Red),
        "green" => Token::Color(Color::Green),
        "blue" => Token::Color(Color::Blue),
        _ => panic!("invalid color"),
    };
    Ok((v.0, token))
}

fn parse_semicolon(input: &str) -> IResult<&str, Token> {
    let v = tag(";")(input)?;
    Ok((v.0, Token::SemiColon))
}

fn parse_colon(input: &str) -> IResult<&str, Token> {
    let v = tag(":")(input)?;
    Ok((v.0, Token::Colon))
}

fn parse_comma(input: &str) -> IResult<&str, Token> {
    let v = tag(",")(input)?;
    Ok((v.0, Token::Comma))
}

fn parse_space(input: &str) -> IResult<&str, Token> {
    let v = tag(" ")(input)?;
    Ok((v.0, Token::WhiteSpace))
}

fn parse_number(input: &str) -> IResult<&str, Token> {
    let v = digit1(input)?;
    let vv = map_res(digit1, |s: &str| s.parse::<usize>())(v.1)?;

    Ok((v.0, Token::Number(vv.1)))
}

fn parse_row(input: &str) -> Game {
    let mut tokens = vec![];
    let mut parsed = input;

    while !parsed.is_empty() {
        let (left, token) = alt((
            parse_game_id,
            parse_number,
            parse_color,
            parse_space,
            parse_semicolon,
            parse_colon,
            parse_comma,
        ))(parsed)
        .unwrap();

        if token != Token::WhiteSpace && token != Token::Colon && token != Token::Comma {
            tokens.push(token);
        }
        parsed = left;
    }

    let mut game = Game {
        id: match tokens.first().unwrap() {
            Token::Game(v) => *v,
            _ => panic!("invalid game id"),
        },
        subsets: vec![],
    };

    let output = tokens.clone().into_iter().fold(Vec::new(), |mut acc, x| {
        if x == Token::SemiColon || acc.is_empty() {
            acc.push(Vec::new());
        } else {
            acc.last_mut().unwrap().push(x.clone());
        }
        acc
    });

    for set in output {
        let colors = set
            .chunks(2)
            .map(|w| {
                let value = match w[0] {
                    Token::Number(v) => v,
                    _ => panic!("color value"),
                };
                let color = match &w[1] {
                    Token::Color(c) => match c {
                        Color::Red => SubSet::Red(value),
                        Color::Green => SubSet::Green(value),
                        Color::Blue => SubSet::Blue(value),
                    },
                    _ => panic!("inavlid color"),
                };
                color
            })
            .collect::<Vec<SubSet>>();

        game.subsets.push(colors);
    }

    return game;
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Token {
    Colon,
    Color(Color),
    Comma,
    Game(usize),
    Number(usize),
    SemiColon,
    WhiteSpace,
}

#[cfg(test)]
mod tests {
    use crate::parse_game_id;

    use super::*;
    const INPUT: &str =
        "Game 1: 1 red, 5 blue, 1 green; 16 blue, 3 red; 6 blue, 5 red; 4 red, 7 blue, 1 green";
    #[test]
    fn parse_game_id_test() {
        let (_, id) = parse_game_id(INPUT).unwrap();
        assert_eq!(id, Token::Game(1))
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
