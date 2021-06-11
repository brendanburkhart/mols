#[derive(Clone)]
pub struct Permutations<T: Copy> {
    state: Vec<u8>,
    index: usize,
    data: Vec<T>,
    original_data: Vec<T>,
    pub position: u32,
}

impl<T: Copy> Permutations<T> {
    pub fn of(array: Vec<T>) -> Permutations<T> {
        Permutations {
            state: vec![0; array.len()],
            index: 0,
            data: array.to_owned(),
            original_data: array.to_owned(),
            position: 0,
        }
    }
}

impl<T: Copy> Permutations<T> {
    fn swap(&mut self) {
        let low_index = match self.index % 2 == 0 {
            true => 0,
            false => self.state[self.index] as usize,
        };

        let temp = self.data[self.index];
        self.data[self.index] = self.data[low_index];
        self.data[low_index] = temp;
    }

    fn rotate_right(&mut self) {
        let mut index = self.data.len() - 1;
        let last = self.data[index];

        while index > 0 {
            self.data[index] = self.data[index - 1];
            index -= 1;
        }

        self.data[0] = last;
    }

    pub fn get(&self) -> Vec<T> {
        self.data.to_owned()
    }

    pub fn at(&self, index: usize) -> T {
        return self.data[index];
    }

    pub fn reset(&mut self) {
        self.state = vec![0; self.data.len()];
        self.index = 0;
        self.position = 0;
        self.data = self.original_data.to_owned();
    }

    // Generate permutations of given array
    // Uses an iterative version of Heap's algorithm
    pub fn advance(&mut self) -> bool {
        while self.index < self.data.len() {
            if self.state[self.index] < self.index as u8 {
                self.swap();

                self.state[self.index] += 1;
                self.index = 0;
                self.position += 1;

                return false;
            } else {
                self.state[self.index] = 0;
                self.index += 1;
            }
        }

        // All permutations generated.
        // Reset to initial state and start over
        self.index = 0;
        self.position = 0;

        let n = self.data.len();
        if n % 2 == 0 {
            self.rotate_right();
        } else {
            let temp = self.data[n - 1];
            self.data[n - 1] = self.data[0];
            self.data[0] = temp;
        }

        return true;
    }
}
