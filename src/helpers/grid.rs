use std::{fmt::{Debug, Display}, usize};

pub struct Grid {
    width: usize,
    height: usize,
    data: Vec<char>,
}

impl Grid {
    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn at(&self, i: usize, j: usize) -> char {
        self.data[self.width * i + j]
    }

    pub fn replace(&mut self, i: usize, j: usize, new: char) {
        self.data[self.width * i + j] = new;
    }

    pub fn above(&self, i: usize, j: usize) -> Option<char> {
        if i == 0 {
            None
        } else {
            Some(self.at(i - 1, j))
        }
    }

    pub fn below(&self, i: usize, j: usize) -> Option<char> {
        if i == self.height - 1 {
            None
        } else {
            Some(self.at(i + 1, j))
        }
    }

    pub fn left(&self, i: usize, j: usize) -> Option<char> {
        if j == 0 {
            None
        } else {
            Some(self.at(i, j - 1))
        }
    }

    pub fn right(&self, i: usize, j: usize) -> Option<char> {
        if j == self.width - 1 {
            None
        } else {
            Some(self.at(i, j + 1))
        }
    }

    pub fn neighbours_4n(&self, i: usize, j: usize) -> Vec<Option<char>> {
        vec![
            self.above(i, j),
            self.below(i, j),
            self.left(i, j),
            self.right(i, j),
        ]
    }

    pub fn neighbours_4n_index(
        &self,
        i: usize,
        j: usize,
    ) -> Vec<(Option<char>, usize, usize)> {
        let mut result = Vec::with_capacity(8);

        if let Some(thing) = self.above(i, j) {
            result.push((Some(thing), i - 1, j));
        } else {
            result.push((None, 0, 0));
        }

        if let Some(thing) = self.below(i, j) {
            result.push((Some(thing), i + 1, j));
        } else {
            result.push((None, 0, 0));
        }

        if let Some(thing) = self.left(i, j) {
            result.push((Some(thing), i, j - 1));
        } else {
            result.push((None, 0, 0));
        }

        if let Some(thing) = self.right(i, j) {
            result.push((Some(thing), i, j + 1));
        } else {
            result.push((None, 0, 0));
        }

        result
    }

    pub fn top_left(&self, i: usize, j: usize) -> Option<char> {
        if i == 0 || j == 0 {
            None
        } else {
            Some(self.at(i - 1, j - 1))
        }
    }

    pub fn bottom_left(&self, i: usize, j: usize) -> Option<char> {
        if i == self.height - 1 || j == 0 {
            None
        } else {
            Some(self.at(i + 1, j - 1))
        }
    }

    pub fn top_right(&self, i: usize, j: usize) -> Option<char> {
        if i == 0 || j == self.width - 1 {
            None
        } else {
            Some(self.at(i - 1, j + 1))
        }
    }

    pub fn bottom_right(&self, i: usize, j: usize) -> Option<char> {
        if i == self.height - 1 || j == self.width - 1 {
            None
        } else {
            Some(self.at(i + 1, j + 1))
        }
    }

    pub fn neighbours_8n(&self, i: usize, j: usize) -> Vec<Option<char>> {
        vec![
            self.above(i, j),
            self.below(i, j),
            self.left(i, j),
            self.right(i, j),
            self.top_left(i, j),
            self.top_right(i, j),
            self.bottom_left(i, j),
            self.bottom_right(i, j),
        ]
    }

    pub fn neighbours_8n_index(
        &self,
        i: usize,
        j: usize,
    ) -> Vec<(Option<char>, usize, usize)> {
        let mut result = Vec::with_capacity(8);

        if let Some(thing) = self.above(i, j) {
            result.push((Some(thing), i - 1, j));
        } else {
            result.push((None, 0, 0));
        }

        if let Some(thing) = self.below(i, j) {
            result.push((Some(thing), i + 1, j));
        } else {
            result.push((None, 0, 0));
        }

        if let Some(thing) = self.left(i, j) {
            result.push((Some(thing), i, j - 1));
        } else {
            result.push((None, 0, 0));
        }

        if let Some(thing) = self.right(i, j) {
            result.push((Some(thing), i, j + 1));
        } else {
            result.push((None, 0, 0));
        }

        if let Some(thing) = self.top_left(i, j) {
            result.push((Some(thing), i - 1, j - 1));
        } else {
            result.push((None, 0, 0));
        }

        if let Some(thing) = self.top_right(i, j) {
            result.push((Some(thing), i - 1, j + 1));
        } else {
            result.push((None, 0, 0));
        }

        if let Some(thing) = self.bottom_left(i, j) {
            result.push((Some(thing), i + 1, j - 1));
        } else {
            result.push((None, 0, 0));
        }

        if let Some(thing) = self.bottom_right(i, j) {
            result.push((Some(thing), i + 1, j + 1));
        } else {
            result.push((None, 0, 0));
        }

        result
    }
}

#[derive(Debug)]
pub enum GridCreationError {
    DimensionMismatch,
    Empty,
}

impl TryFrom<&Vec<&str>> for Grid {
    type Error = GridCreationError;

    fn try_from(lines: &Vec<&str>) -> Result<Self, Self::Error> {
        let height = lines.len();
        if height == 0 {
            return Err(Self::Error::Empty);
        }
        let width = lines[0].len();
        let mut data = vec![];

        for line in lines {
            if line.len() != width {
                return Err(Self::Error::DimensionMismatch);
            }
            for chr in line.chars() {
                data.push(chr);
            }
        }
        Ok(Self {
            width,
            height,
            data,
        })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                write!(f, "[{}]", self.at(i, j))?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
