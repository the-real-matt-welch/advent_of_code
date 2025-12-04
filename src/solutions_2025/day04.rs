use std::fmt::Debug;

use crate::helpers::grid::Grid;

pub fn part1<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let mut total = 0;
    let grid: Grid = input.into();
    for i in 0..grid.get_height() {
        for j in 0..grid.get_width() {
            total += (grid
                .neighbours_8n(i, j)
                .iter()
                .filter(|x| **x == Some('@'))
                .count()
                < 4 && grid.at(i, j) == '@') as u64;
        }
    }
    total
}

pub fn part2<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let mut grid = input.into();
    let mut old = remove_and_count(&mut grid);
    let mut new = 0;
    while old != new {
        new = old;
        old += remove_and_count(&mut grid);
    }
    new
}

pub fn remove_and_count(grid: &mut Grid) -> u64 {
    let mut total = 0;
    for i in 0..grid.get_height() {
        for j in 0..grid.get_width() {
            if grid
                .neighbours_8n(i, j)
                .iter()
                .filter(|x| **x == Some('@'))
                .count()
                < 4 && grid.at(i, j) == '@'{
                total += 1;
                grid.replace(i, j, '.');
            }
        }
    }
    total
}
