use std::fmt::Debug;

pub fn part1<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let mut count = 0;
    let mut start: i32 = 50;
    input[..input.len()].iter().for_each(|line| {
        let (direction, n) = line.split_at(1);
        let n = n.parse::<i32>().unwrap();
        if direction == "R" {
            start = (start + n).rem_euclid(100);
        } else {
            start = (start - n).rem_euclid(100);
        }
        count += (start == 0 || start == -100) as i32;
    });
    count
}

pub fn part2<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let mut count = 0;
    let mut start: i32 = 50;
    input[..input.len()].iter().for_each(|line| {
        let (direction, n) = line.split_at(1);
        let n = n.parse::<i32>().unwrap();
        if direction == "R" {
            count += (start + n) / 100;
            start = (start + n).rem_euclid(100);
        } else {
            count += (n - start + 100) / 100 - (start == 0) as i32;
            start = (start - n).rem_euclid(100);
        }
    });
    count
}
