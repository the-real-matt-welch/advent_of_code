use std::fmt::Debug;

pub fn part1<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let ids = input[0].split(',');
    ids.map(|range| {
        let (start, finish) = range.split_once('-').unwrap();
        let start = start.parse::<u64>().unwrap();
        let finish = finish.parse::<u64>().unwrap();

        let mut total = 0;
        for i in start..=finish {
            let n = i.to_string();
            if n.len() % 2 == 0 {
                if n[..n.len() / 2] == n[n.len() / 2..] {
                    total += i
                }
            }
        }

        total
    })
    .sum::<u64>()
}

pub fn part2<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let ids = input[0].split(',');
    ids.map(|range| {
        let (start, finish) = range.split_once('-').unwrap();
        let start = start.parse::<u64>().unwrap();
        let finish = finish.parse::<u64>().unwrap();

        let mut total = 0;
        for i in start..=finish {
            let n = i.to_string();
            for j in 1..=n.len() / 2 {
                if n.len() % j == 0 {
                    let works = || {
                        for k in 1..n.len()/j {
                            let a = &n[k*j..k*j+j];
                            let b = &n[..j];
                            if a != b {
                                return false;
                            }
                        }
                        return true;
                    };
                    if works() {
                        total += i;
                        break
                    }
                }
            }
        }

        total
    })
    .sum::<u64>()
}
