use regex::Regex;
use std::fmt::Debug;

pub fn part1<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let text = input.join("");
    let re = Regex::new("mul\\(\\d{1,3},\\d{1,3}\\)").unwrap();
    let instructions: Vec<&str> =
        re.find_iter(&text).map(|m| m.as_str()).collect();

    let number = Regex::new("\\d{1,3}").unwrap();
    let mut total = 0;
    for instruction in &instructions {
        let numbers: Vec<u32> = number
            .find_iter(instruction)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        let (first, second) = (numbers[0], numbers[1]);
        total += first * second;
    }

    total
}

pub fn part2<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let text = input.join("");
    let re =
        Regex::new("mul\\(\\d{1,3},\\d{1,3}\\)|do\\(\\)|don't\\(\\)").unwrap();
    let instructions: Vec<&str> =
        re.find_iter(&text).map(|m| m.as_str()).collect();

    let number = Regex::new("\\d{1,3}").unwrap();
    let mut total = 0;
    let mut switch = true;
    for instruction in &instructions {
        match *instruction {
            "do()" => switch = true,
            "don't()" => switch = false,
            _ => {
                let numbers: Vec<u32> = number
                    .find_iter(instruction)
                    .map(|m| m.as_str().parse().unwrap())
                    .collect();
                let (first, second) = (numbers[0], numbers[1]);
                total += first * second * (switch as u32);
            }
        }
    }

    total
}
