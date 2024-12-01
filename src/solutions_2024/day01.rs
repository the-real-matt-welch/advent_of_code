use std::{collections::HashMap, fmt::Debug};

pub fn part1<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input {
        let mut numbers = line
            .split("   ")
            .map(|s| s.parse::<u32>());
        list1.push(numbers.next().unwrap().unwrap());
        list2.push(numbers.next().unwrap().unwrap());
    }

    list1.sort_unstable();
    list2.sort_unstable();

    list1
        .iter()
        .zip(list2)
        .map(|(a, b)| a.abs_diff(b))
        .sum::<u32>()
}

pub fn part2<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input {
        let mut numbers = line
            .split("   ")
            .map(|s| s.parse::<u32>());
        list1.push(numbers.next().unwrap().unwrap());
        list2.push(numbers.next().unwrap().unwrap());
    }

    let mut counts = HashMap::new();
    for num in &list2 {
        counts.entry(num).and_modify(|x| *x += 1).or_insert(1);
    }

    list1
        .iter()
        .map(|n| *n * *counts.get(n).unwrap_or(&0))
        .sum::<u32>()
}
