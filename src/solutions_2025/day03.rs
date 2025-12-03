use std::fmt::Debug;

pub fn part1<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    input
        .iter()
        .map(|bank| {
            let bank: Vec<u64> = bank
                .chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect();
            let first = bank[..bank.len() - 1].iter().max().unwrap();
            let second = bank
                [bank.iter().position(|n| n == first).unwrap() + 1..]
                .iter()
                .max()
                .unwrap();
            10 * first + second
        })
        .sum::<u64>()
}

pub fn part2<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    input
        .iter()
        .map(|bank| {
            let bank: Vec<u64> = bank
                .chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect();
            let mut digits = [0; 12];
            let (mut idx, _) = bank[..bank.len() - 11]
                .iter()
                .enumerate()
                .max_by(|(i, a), (j, b)| {
                    (a, 100 - i).partial_cmp(&(b, 100 - j)).unwrap()
                })
                .unwrap();
            digits[0] = bank[idx];
            for i in 1..12 {
                idx += bank[idx + 1..bank.len() - 11 + i]
                    .iter()
                    .enumerate()
                    .max_by(|(i, a), (j, b)| {
                        (a, 100 - i).partial_cmp(&(b, 100 - j)).unwrap()
                    })
                    .unwrap()
                    .0
                    + 1;
                digits[i] = bank[idx];
            }
            digits
                .iter()
                .enumerate()
                .map(|(i, n)| *n * 10_u64.pow(11 - i as u32))
                .sum::<u64>()
        })
        .sum::<u64>()
}
