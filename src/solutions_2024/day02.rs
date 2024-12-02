use std::fmt::Debug;

pub fn part1<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let mut total = 0;
    for report in input {
        let levels: Vec<i32> = report
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        total += safe(levels) as i32;
    }
    total
}

pub fn part2<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let mut total = 0;
    for report in input {
        let levels: Vec<i32> = report
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let mut safes = 0;
        for thing in variations(levels) {
            safes += safe(thing) as i32;
        }
        total += (safes > 0) as i32;
    }
    total
}

fn variations(levels: Vec<i32>) -> Vec<Vec<i32>> {
    let l = levels.len();
    let mut vars = Vec::with_capacity(l);
    for i in 0..l {
        let mut new = Vec::with_capacity(l - 1);
        for (j, level) in levels.iter().enumerate() {
            if j != i {
                new.push(*level);
            }
        }
        vars.push(new);
    }
    vars
}

fn safe(levels: Vec<i32>) -> bool {
    let direction = levels[1] - levels[0];
    for (i, level) in levels.windows(2).enumerate() {
        if (level[1] - level[0]) * direction < 0 {
            return false;
        } else if !(1..=3).contains(&level[1].abs_diff(level[0])) {
            return false;
        }
    }
    return true;
}
