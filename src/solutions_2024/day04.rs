use std::fmt::Debug;

use crate::helpers::grid::Grid;

pub fn part1<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let mut total = 0;
    for line in input {
        total += line.matches("XMAS").count();
        total += line.matches("SAMX").count();
    }

    let mut transposed = Vec::new();
    for i in 0..input[0].len() {
        let mut s = String::new();
        for j in 0..input.len() {
            s.push(input[j].chars().collect::<Vec<char>>()[i]);
        }
        transposed.push(s);
    }
    for line in transposed {
        total += line.matches("XMAS").count();
        total += line.matches("SAMX").count();
    }

    let n = input.len() + input[0].len() - 1;
    let mut diagonals: Vec<String> = vec![String::new(); n];
    let mut other_diagonals: Vec<String> = vec![String::new(); n];
    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let index = diagonals.len() + i - j - input[0].len();
            diagonals[index].push(c);
            other_diagonals[i + j].push(c);
        }
    }
    for line in diagonals {
        total += line.matches("XMAS").count();
        total += line.matches("SAMX").count();
    }
    for line in other_diagonals {
        total += line.matches("XMAS").count();
        total += line.matches("SAMX").count();
    }

    total
}

pub fn part2<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let mut total = 0;
    let grid = Grid::try_from(input).unwrap();

    for i in 0..grid.get_height() {
        for j in 0..grid.get_width() {
            if grid.at(i, j) == 'A' {
                if grid.top_left(i, j) == Some('M') {
                    total += (
                        grid.top_right(i, j) == Some('M') &&
                        grid.bottom_left(i, j) == Some('S') &&
                        grid.bottom_right(i, j) == Some('S')
                    ) as u32;

                    total += (
                        grid.bottom_left(i, j) == Some('M') &&
                        grid.top_right(i, j) == Some('S') &&
                        grid.bottom_right(i, j) == Some('S')
                    ) as u32;
                }
                if grid.top_left(i, j) == Some('S') {
                    total += (
                        grid.top_right(i, j) == Some('S') &&
                        grid.bottom_left(i, j) == Some('M') &&
                        grid.bottom_right(i, j) == Some('M')
                    ) as u32;

                    total += (
                        grid.bottom_left(i, j) == Some('S') &&
                        grid.top_right(i, j) == Some('M') &&
                        grid.bottom_right(i, j) == Some('M')
                    ) as u32;
                }
            }
        }
    }

    total
}
