use std::{collections::HashSet, fmt::Debug};

use crate::helpers::grid::Grid;

pub fn part1<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let mut positions = HashSet::new();
    let mut current = Some((0, 0));
    let mut direction = Direction::Up;
    let map = Grid::try_from(input).unwrap();

    for i in 0..map.get_height() {
        for j in 0..map.get_width() {
            if map.at(i, j) == '^' {
                current = Some((i, j));
            }
        }
    }
    positions.insert(current);

    while let Some((i, j)) = current {
        let next = match direction {
            Direction::Up => map.above(i, j),
            Direction::Right => map.right(i, j),
            Direction::Down => map.below(i, j),
            Direction::Left => map.left(i, j),
        };
        match next {
            Some('#') => direction = turn_90(direction),
            None => current = None,
            _ => {
                match direction {
                    Direction::Up => current = Some((i - 1, j)),
                    Direction::Right => current = Some((i, j + 1)),
                    Direction::Down => current = Some((i + 1, j)),
                    Direction::Left => current = Some((i, j - 1)),
                }
                positions.insert(current);
            }
        }
    }
    positions.len()
}

pub fn part2<'a>(input: &'a Vec<&'a str>) -> impl Debug + 'a {
    let mut map = Grid::try_from(input).unwrap();
    let mut start = (0, 0, Direction::Up);
    for i in 0..map.get_height() {
        for j in 0..map.get_width() {
            if map.at(i, j) == '^' {
                start = (i, j, Direction::Up);
            }
        }
    }

    let mut total = 0;
    for i in 0..map.get_height() {
        for j in 0..map.get_width() {
            if map.at(i, j) == '.' && creates_loop(&mut map, i, j, start) {
                total += 1
            }
        }
    }
    total
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn turn_90(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn creates_loop(
    map: &mut Grid,
    y: usize,
    x: usize,
    start: (usize, usize, Direction),
) -> bool {
    map.replace(y, x, '#');
    let mut positions = HashSet::new();
    let (mut i, mut j, mut direction) = start;
    loop {
        let next = match direction {
            Direction::Up => map.above(i, j),
            Direction::Right => map.right(i, j),
            Direction::Down => map.below(i, j),
            Direction::Left => map.left(i, j),
        };
        match next {
            Some('#') => direction = turn_90(direction),
            None => break,
            _ => {
                match direction {
                    Direction::Up => (i, j) = (i - 1, j),
                    Direction::Right => (i, j) = (i, j + 1),
                    Direction::Down => (i, j) = (i + 1, j),
                    Direction::Left => (i, j) = (i, j - 1),
                }
                if positions.contains(&(i, j, direction)) {
                    map.replace(y, x, '.');
                    return true;
                }
                positions.insert((i, j, direction));
            }
        }
    }
    map.replace(y, x, '.');
    false
}
