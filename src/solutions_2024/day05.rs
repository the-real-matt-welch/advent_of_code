use std::{collections::HashMap, fmt::Debug};

pub fn part1<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let mut prereqs: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut blank = 0;
    for (i, line) in input.iter().enumerate() {
        if line.len() < 2 {
            blank = i;
            break;
        }
        let (value, key) = line.split_once('|').unwrap();
        prereqs
            .entry(key.parse().unwrap())
            .and_modify(|x| x.push(value.parse().unwrap()))
            .or_insert(vec![value.parse().unwrap()]);
    }

    let mut total = 0;
    for line in &input[blank + 1..] {
        let line: Vec<u32> =
            line.split(',').map(|s| s.parse().unwrap()).collect();
        if in_order(&line, &prereqs) {
            total += line[line.len() / 2];
        }
    }
    total
}

pub fn part2<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let mut prereqs: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut blank = 0;
    for (i, line) in input.iter().enumerate() {
        if line.len() < 2 {
            blank = i;
            break;
        }
        let (value, key) = line.split_once('|').unwrap();
        prereqs
            .entry(key.parse().unwrap())
            .and_modify(|x| x.push(value.parse().unwrap()))
            .or_insert(vec![value.parse().unwrap()]);
    }

    let mut total = 0;
    for line in &input[blank + 1..] {
        let line: Vec<u32> =
            line.split(',').map(|s| s.parse().unwrap()).collect();
        if !in_order(&line, &prereqs) {
            total += reordered(&line, &prereqs)[line.len() / 2];
        }
    }
    total
}

fn in_order(line: &Vec<u32>, prereqs: &HashMap<u32, Vec<u32>>) -> bool {
    for (i, num) in line.iter().enumerate() {
        for value in prereqs.get(num).unwrap_or(&Vec::new()) {
            let j = line.iter().position(|x| x == value).unwrap_or(0);
            if i < j {
                return false;
            }
        }
    }
    true
}

fn reordered(line: &Vec<u32>, prereqs: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let mut better = Vec::with_capacity(line.len());
    while better.len() < line.len() {
        for num in line {
            if better.contains(num) {
                continue;
            }
            let mut good = true;
            for value in prereqs.get(&num).unwrap_or(&vec![]) {
                if !better.contains(value) && line.contains(value) {
                    good = false;
                    break;
                }
            }
            if good {
                better.push(*num);
            }
        }
    }
    better
}
