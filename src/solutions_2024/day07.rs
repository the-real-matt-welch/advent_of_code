use std::fmt::Debug;

pub fn part1<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let mut total = 0;
    for line in input {
        let (solution, rest) = line.split_once(": ").unwrap();
        let solution = solution.parse::<u64>().unwrap();
        let nums = rest
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        if valid(&nums, solution) {
            total += solution;
        }
    }
    total
}

pub fn part2<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let mut total = 0;
    for line in input {
        let (solution, rest) = line.split_once(": ").unwrap();
        let solution = solution.parse::<u64>().unwrap();
        let nums = rest
            .split_whitespace()
            .collect();
        if valid_2(&nums, solution) {
            total += solution;
        }
    }
    total
}

fn valid(nums: &Vec<u64>, solution: u64) -> bool {
    for comb in 0..(2_u64.pow(nums.len() as u32 - 1)) {
        let mut total = nums[0];
        for (i, num) in nums[1..].iter().enumerate() {
            if comb & 2_u64.pow(i as u32) == 0 {
                total += num
            } else {
                total *= num
            }
        }
        if total == solution {
            return true;
        }
    }
    false
}

fn valid_2(nums: &Vec<&str>, solution: u64) -> bool {
    for comb in 0..(3_u64.pow(nums.len() as u32 - 1)) {
        let mut total = String::from(nums[0]);
        for (i, num) in nums[1..].iter().enumerate() {
            match (comb / 3_u64.pow(i as u32)) % 3 {
                0 => {
                    total = (total.parse::<u64>().unwrap()
                        + num.parse::<u64>().unwrap())
                    .to_string()
                }
                1 => {
                    total = (total.parse::<u64>().unwrap()
                        * num.parse::<u64>().unwrap())
                    .to_string()
                }
                2 => total.push_str(num),
                _ => panic!("how'd this get here?"),
            }
        }
        if total.parse::<u64>().unwrap() == solution {
            return true;
        }
    }
    false
}
