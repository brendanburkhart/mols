use std::fmt;
use crate::permutations;

fn factorial(x: u32) -> u32 {
    match x {
        0 => 1,
        _ => x * factorial(x - 1),
    }
}

pub struct Square {
    size: usize,
    data: Vec<u8>,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.size {
            for j in 0..self.size {
                let value = self.data[i * self.size + j];
                write!(f, "{}, ", value + 1)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl Square {
    fn column_is_valid(&self, column_index: usize) -> bool {
        let mut elements = vec![false; self.size];

        for i in 0..self.size {
            let value = self.data[i * self.size + column_index] as usize;

            if elements[value] {
                return false;
            }

            elements[value] = true;
        }

        return true;
    }

    pub fn is_latin(&self) -> bool {
        for column_index in 0..self.size {
            if !self.column_is_valid(column_index) {
                return false;
            }
        }

        return true;
    }
}

pub struct Generator {
    size: usize,
    state: Vec<u32>,
    max_permutations: u32,
    done: bool,
    permutations: Vec<permutations::Permutations<u8>>,
}

impl Generator {
    pub fn new(size: usize) -> Generator {
        let elements = (0..size as u8).collect();
        let permutations = vec![permutations::Permutations::of(elements); size];
        let state = vec![0; size];

        Generator {
            size,
            state,
            max_permutations: factorial(size as u32),
            done: false,
            permutations,
        }
    }
}

impl Generator {
    fn advance(&mut self) -> bool {
        let mut index = self.size - 1;

        loop {
            let done = self.permutations[index].advance();
            if done && index == 1 {
                return true;
            }

            self.state[index] += 1;
            if self.state[index] < self.max_permutations {
                break;
            }

            if index > 1 {
                self.state[index] = 0;
                index -= 1;
            } else {
                break;
            }
        }

        return false;
    }

    fn advance_row(&mut self, row_index: usize) -> bool {
        let mut index = self.size - 1;

        while index > row_index {
            self.permutations[index].reset();
            self.state[index] = 0;
            index -= 1;
        }

        loop {
            let done = self.permutations[index].advance();
            if done && index == 1 {
                return true;
            }

            self.state[index] += 1;
            if self.state[index] < self.max_permutations {
                break;
            }

            if index > 1 {
                self.state[index] = 0;
                index -= 1;
            } else {
                break;
            }
        }

        return false;
    }

    fn invalid_row(&mut self) -> usize {
        for column_index in 0..self.size {
            let mut elements = vec![false; self.size];

            for i in 0..self.size {
                let value = self.permutations[i].at(column_index) as usize;

                if elements[value] {
                    return i;
                }

                elements[value] = true;
            }
        }

        return self.size;
    }

    fn construct_square(&self) -> Square {
        let mut data = vec![0; self.size * self.size];

        for i in 0..self.size {
            let permutation = self.permutations[i].get();

            for j in 0..self.size {
                data[i * self.size + j] = permutation[j];
            }
        }

        Square {
            size: self.size,
            data
        }
    }

    fn column_is_valid(&self, column_index: usize) -> bool {
        let mut elements = vec![false; self.size];

        for i in 0..self.size {
            let value = self.permutations[i].at(column_index) as usize;

            if elements[value] {
                return false;
            }

            elements[value] = true;
        }

        return true;
    }

    fn is_latin(&self) -> bool {
        for column_index in 0..self.size {
            if !self.column_is_valid(column_index) {
                return false;
            }
        }

        return true;
    }

    fn full_advance(&mut self) -> bool {
        self.done = self.advance();
        if self.done {
            return true;
        }

        loop {
            let row_index = self.invalid_row();
            if row_index == self.size {
                return false;
            }

            self.done = self.advance_row(row_index);
            if self.done {
                return true;
            }
        }
    }
}

impl Iterator for Generator {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        if self.done {
            return None;
        }

        if self.size == 1 {
            self.done = true;
            return Some(self.construct_square());
        }

        self.done = self.full_advance();

        return if self.is_latin() { Some(self.construct_square()) } else { None };
    }
}
