use std::{collections::HashSet, fmt::Debug};

use crate::helpers::grid::Grid;

pub fn part1<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let engine = Grid::try_from(input).unwrap();

    let mut total = 0;
    let mut part_num: String = String::new();
    let mut touching = false;

    for i in 0..engine.get_height() {
        for j in 0..engine.get_width() {
            if engine.at(i, j).is_digit(10) {
                part_num.push(engine.at(i, j));
                for part in engine.neighbours_8n(i, j) {
                    if let Some(part) = part {
                        if !part.is_digit(10) && part != '.' {
                            touching = true;
                        }
                    }
                }
            } else {
                if touching {
                    total += part_num.parse::<u32>().unwrap();
                    touching = false;
                }
                part_num.clear();
            }
        }
    }
    total
}

pub fn part2<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let engine = Grid::try_from(input).unwrap();

    let mut total = 0;
    for i in 0..engine.get_height() {
        for j in 0..engine.get_width() {
            if engine.at(i, j) != '*' {
                continue;
            }
            // kind of cheating, we assume the 2 parts will have different
            // numbers
            let mut neighbouring_parts: HashSet<u32> =
                HashSet::with_capacity(2);
            for (neighbour, x, y) in engine.neighbours_8n_index(i, j) {
                if neighbour.is_none() {
                    continue;
                }
                if neighbour.unwrap().is_digit(10) {
                    let mut part_num: String = String::new();
                    let mut first = y;
                    while let Some(thing) = engine.left(x, first) {
                        if thing.is_digit(10) {
                            first -= 1
                        } else {
                            break;
                        }
                    }

                    let mut next = first;
                    part_num.push(engine.at(x, next));
                    while let Some(thing) = engine.right(x, next) {
                        if thing.is_digit(10) {
                            part_num.push(thing);
                            next += 1
                        } else {
                            break;
                        }
                    }
                    neighbouring_parts.insert(part_num.parse().unwrap());
                }
            }
            if neighbouring_parts.len() == 2 {
                total += neighbouring_parts.iter().product::<u32>();
            }
        }
    }

    total
}
