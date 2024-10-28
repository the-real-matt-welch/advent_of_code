use std::{collections::HashSet, fmt::Debug};

pub fn part1<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let mut total = 0;
    for line in input {
        let Some((first, second)) =
            line[(line.find(':').unwrap() + 2)..].split_once('|')
        else {
            panic!("😭 how?");
        };
        let winners: HashSet<u32> = first
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let my_nums: HashSet<u32> = second
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let n: u32 = winners
            .intersection(&my_nums)
            .collect::<Vec<&u32>>()
            .len() as u32;
        if n != 0 {
            total += (2 as u32).pow(n - 1);
        }
    }
    total
}

pub fn part2<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let mut counts = vec![1; input.len()];

    for (i, line) in input.into_iter().enumerate() {
        let Some((first, second)) =
            line[(line.find(':').unwrap() + 2)..].split_once('|')
        else {
            panic!("😭 how?");
        };
        let winners: HashSet<u32> = first
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let my_nums: HashSet<u32> = second
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let n: usize = winners
            .intersection(&my_nums)
            .collect::<Vec<&u32>>()
            .len();

        for j in (i+1)..=(i+n) {
            counts[j] += counts[i];
        }
    }
    counts.iter().sum::<u32>()
}
