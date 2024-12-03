use regex::Regex;
use std::fmt::Debug;

pub fn part1<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let text = input.join("");
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let instructions = re.find_iter(&text).map(|m| m.as_str());

    let number = Regex::new(r"\d{1,3}").unwrap();
    let mut total = 0;
    for instruction in instructions {
        total += number
            .find_iter(instruction)
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .product::<u32>();
    }

    total
}

pub fn part2<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let text = input.join("");
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let instructions = re.find_iter(&text).map(|m| m.as_str());

    let number = Regex::new(r"\d{1,3}").unwrap();
    let mut total = 0;
    let mut switch = true;
    for instruction in instructions {
        match instruction {
            "do()" => switch = true,
            "don't()" => switch = false,
            _ => {
                total += number
                    .find_iter(instruction)
                    .map(|m| m.as_str().parse::<u32>().unwrap())
                    .fold(switch as u32, |a, b| a * b);
            }
        }
    }

    total
}
