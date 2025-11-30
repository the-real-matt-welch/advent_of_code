#[macro_export]
macro_rules! solve {
    ($year:expr, $day:expr, $part:expr, $input:expr) => {
        match $year {
            2023 => day!(solutions_2023, $day, $part, $input),
            2024 => day!(solutions_2024, $day, $part, $input),
            2025 => day!(solutions_2025, $day, $part, $input),
            _ => unreachable!()
        }
    };
}

#[macro_export]
macro_rules! day {
    ($year:ident, $day:expr, $part:expr, $input:expr) => {
        match $day {
            1 => part!($year, day01, $part, $input),
            2 => part!($year, day02, $part, $input),
            3 => part!($year, day03, $part, $input),
            4 => part!($year, day04, $part, $input),
            5 => part!($year, day05, $part, $input),
            6 => part!($year, day06, $part, $input),
            7 => part!($year, day07, $part, $input),
            8 => part!($year, day08, $part, $input),
            9 => part!($year, day09, $part, $input),
            10 => part!($year, day10, $part, $input),
            11 => part!($year, day11, $part, $input),
            12 => part!($year, day12, $part, $input),
            13 => part!($year, day13, $part, $input),
            14 => part!($year, day14, $part, $input),
            15 => part!($year, day15, $part, $input),
            16 => part!($year, day16, $part, $input),
            17 => part!($year, day17, $part, $input),
            18 => part!($year, day18, $part, $input),
            19 => part!($year, day19, $part, $input),
            20 => part!($year, day20, $part, $input),
            21 => part!($year, day21, $part, $input),
            22 => part!($year, day22, $part, $input),
            23 => part!($year, day23, $part, $input),
            24 => part!($year, day24, $part, $input),
            25 => part!($year, day25, $part, $input),
            _ => unreachable!()
        }
    }
}

#[macro_export]
macro_rules! part {
    ($year:ident, $day:ident, $part:expr, $input:expr) => {
        match $part {
            1 => format!("{:?}", $year::$day::part1($input)),
            2 => format!("{:?}", $year::$day::part2($input)),
            _ => unreachable!()
        }
    };
}
