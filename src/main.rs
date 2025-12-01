#![allow(dead_code)]
#![allow(unused_variables)]

use clap::Parser;
use std::fs;
use std::time::Instant;

mod helpers;
mod macros;
mod solutions_2023;
mod solutions_2024;
mod solutions_2025;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: Option<u8>,

    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,

    #[arg(short, long, default_value_t = 2025,
        value_parser = clap::value_parser!(u16).range(2023..=2025))]
    year: u16,

    #[arg(short, long)]
    test: bool,

    #[arg(long)]
    time: bool,
}

fn main() {
    let args = Arguments::parse();
    let parts: Vec<u8> = match args.part {
        Some(p) => vec![p],
        None => vec![1, 2],
    };

    let path = if args.test {
        format!("test_inputs/{}/day{:0>2}.txt", args.year, args.day)
    } else {
        format!("inputs/{}/day{:0>2}.txt", args.year, args.day)
    };
    // TODO: Refactor project so that parts take string input and return ()
    let raw = fs::read_to_string(path).expect("Error reading input file");
    let input: Vec<&str> = raw.lines().collect();

    for part in &parts {
        println!(
            "Part {part}\n\
                  -------"
        );
        let timer = Instant::now();
        let solution = solve!(args.year, args.day, *part, &input);
        let time = timer.elapsed();
        println!("answer: {solution}");
        if args.time { println!("time:   {time:?}"); }
        if args.part.is_none() && *part == 1 { println!(); }
    }
}
