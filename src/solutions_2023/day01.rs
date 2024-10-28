use regex::Regex;
use std::fmt::Debug;

pub fn part1<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let mut total: u32 = 0;

    for line in input {
        let mut number = String::with_capacity(2);

        for c in line.chars() {
            if c.is_digit(10) {
                number.push(c);
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_digit(10) {
                number.push(c);
                break;
            }
        }

        total += number.parse::<u32>().unwrap();
    }

    total
}

pub fn part2<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let re = Regex::new(
        "1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine",
    )
    .unwrap();

    let mut total: u32 = 0;

    for line in input {
        let mut number = String::with_capacity(2);

        let mat1 = re.find(line).unwrap();
        number.push(convert_to_digit(mat1.as_str()));

        let mat2s: Vec<&str> = re.find_iter(line).map(|x| x.as_str()).collect();
        let mat2 = mat2s[mat2s.len() - 1];
        number.push(convert_to_digit(mat2));

        total += number.parse::<u32>().unwrap();
    }

    // this is a bit hacky. the regex doesn't cover cases like oneight so I
    // made a script to find them and manually adjusted the total
    total - 2
}

fn convert_to_digit(s: &str) -> char {
    match s {
        "one" | "1" => '1',
        "two" | "2" => '2',
        "three" | "3" => '3',
        "four" | "4" => '4',
        "five" | "5" => '5',
        "six" | "6" => '6',
        "seven" | "7" => '7',
        "eight" | "8" => '8',
        "nine" | "9" => '9',
        _ => '0',
    }
}
