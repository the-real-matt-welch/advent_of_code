use std::fmt::Debug;

pub fn part1<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let mut total = 0;
    for report in input {
        let levels: Vec<i32> = report
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let mut safe = 1;
        let mut direction = 0;
        for (i, level) in levels.windows(2).enumerate() {
            if i == 0 {
                direction = level[1] - level[0]
            }

            if (level[1] - level[0]) * direction < 0 {
                safe = 0;
            } else if level[1].abs_diff(level[0]) < 1
                || level[1].abs_diff(level[0]) > 3
            {
                safe = 0;
            }
        }
        total += safe;
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
            let mut safe = 1;
            let mut direction = 0;
            for (i, level) in thing.windows(2).enumerate() {
                if i == 0 {
                    direction = level[1] - level[0]
                }

                if (level[1] - level[0]) * direction < 0 {
                    safe = 0;
                } else if level[1].abs_diff(level[0]) < 1
                    || level[1].abs_diff(level[0]) > 3
                {
                    safe = 0;
                }
            }
            safes += safe;
        }
        if safes > 0 {
            total += 1;
        }
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
