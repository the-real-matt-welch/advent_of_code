use range_union_find::RangeUnionFind;
use std::fmt::Debug;
use std::ops::RangeInclusive;

pub fn part1<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let blank = input.iter().position(|s| s.is_empty()).unwrap();
    let (fresh, ids) = input.split_at(blank);
    let ids = &ids[1..];

    let fresh: Vec<RangeInclusive<u64>> = fresh
        .iter()
        .map(|x| {
            x.split_once('-')
                .map(|(a, b)| a.parse::<u64>().unwrap()..=b.parse().unwrap())
                .unwrap()
        })
        .collect();
    ids.iter()
        .map(|n| n.parse::<u64>().unwrap())
        .filter(|n| fresh.iter().any(|r| r.contains(n)))
        .count()
}

pub fn part2<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let blank = input.iter().position(|s| s.is_empty()).unwrap();
    let (fresh, ids) = input.split_at(blank);

    let mut ranges = RangeUnionFind::new();
    let fresh = fresh
        .iter()
        .map(|x| {
            x.split_once('-')
                .map(|(a, b)| a.parse::<u64>().unwrap()..=b.parse().unwrap())
                .unwrap()
        })
        .for_each(|r| {
            let _ = ranges.insert_range(&r);
        });
    ranges
        .to_collection::<Vec<RangeInclusive<u64>>>()
        .into_iter()
        .map(|r| r.count())
        .sum::<usize>()
}
