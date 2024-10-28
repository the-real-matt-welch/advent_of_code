use std::fmt::Debug;

pub fn part1<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    let mut total = 0;
    for (id, line) in input.iter().enumerate() {
        let line = &line[(line.find(':').unwrap()+2)..];
        let mut valid = true;
        for draw in line.split(&[';', ',']) {
            let info: Vec<&str> = draw.trim().split(' ').collect();
            let num: u32 = info[0].parse().unwrap();
            let colour = info[1];
            match colour {
                "red" => { if num > MAX_RED { valid = false} },
                "green" => { if num > MAX_GREEN { valid = false} },
                "blue" => { if num > MAX_BLUE { valid = false} },
                _ => unreachable!()
            }
        }
        total += (id+1)*(valid as usize);
    }
    total
}

pub fn part2<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let mut total = 0;
    for (id, line) in input.iter().enumerate() {
        let mut max_red: u32 = 0;
        let mut max_green: u32 = 0;
        let mut max_blue: u32 = 0;

        let line = &line[(line.find(':').unwrap()+2)..];
        for draw in line.split(&[';', ',']) {
            let info: Vec<&str> = draw.trim().split(' ').collect();
            let num: u32 = info[0].parse().unwrap();
            let colour = info[1];
            match colour {
                "red" => { if num > max_red { max_red = num } },
                "green" => { if num > max_green { max_green = num } },
                "blue" => { if num > max_blue { max_blue = num } },
                _ => unreachable!()
            }
        }
        total += max_red*max_green*max_blue;
    }
    total
}
