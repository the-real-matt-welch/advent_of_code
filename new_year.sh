#!/usr/bin/env fish

set -l year $argv[1]
set -l contents "\
use std::fmt::Debug;

pub fn part1<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    \"Not implemented\"
}

pub fn part2<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    \"Not implemented\"
}"

set -l mod "\
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;"

mkdir src/solutions_$year
mkdir inputs/$year
echo $mod > src/solutions_$year/mod.rs

set -l day 1
while test $day -lt 26
    echo $contents > src/solutions_$year/day(string pad -w2 -c0 $day).rs
    echo :3 > inputs/$year/day(string pad -w2 -c0 $day).txt
    set day (math $day + 1)
end

echo "Remember to update the solve macro and declare the new module!"
