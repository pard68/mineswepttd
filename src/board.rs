use fmt;
use rand;

#[derive(Debug, Clone)]
pub struct Cell {
    mine: bool,
    flag: bool,
    reveal: bool,
    neighbors: u8,
}

impl Cell {
    fn new_empty() -> Cell {
        Cell {
            mine: false,
            flag: false,
            reveal: false,
            neighbors: 0,
        }
    }

    fn toggle_mine(mut self) -> Self {
        self.mine = !self.mine;
        self
    }

    fn toggle_flag(mut self) -> Self {
        self.flag = !self.flag;
        self
    }

    fn toggle_reveal(mut self) -> Self {
        self.reveal = !self.reveal;
        self
    }

    fn with_mine(mut self) -> Self {
        self.mine = true;
        self
    }

    fn with_neighbors(mut self, n: u8) -> Self {
        self.neighbors = n;
        self
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    difficulty: usize,
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: Vec::new(),
            width: 0,
            height: 0,
            difficulty: 0,
        }
    }

    pub fn with_height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn with_difficulty(mut self, difficulty: usize) -> Self {
        self.difficulty = difficulty;
        self
    }

    pub fn build(mut self) -> Self {
        let width = self.width.clone();
        let height = self.height.clone();
        let difficulty = self.difficulty.clone();

        let mut rng = rand::thread_rng();
        let mines = rand::seq::index::sample(&mut rng, width * height, difficulty).into_vec();
        let cells: Vec<Cell> = (0..self.width * self.height)
            .into_iter()
            .map(|i| {
                let neighbor_bombs: u8 = self
                    .clone()
                    .adjacent_cells(i)
                    .iter()
                    .fold(0, |acc, x| acc + mines.contains(&x) as u8);
                Cell {
                    mine: mines.contains(&i),
                    flag: false,
                    reveal: false,
                    neighbors: neighbor_bombs,
                }
            })
            .collect();
        self.cells = cells;
        self
    }

    fn adjacent_cells(self, i: usize) -> Vec<usize> {
        let width = self.width.clone();
        let height = self.height.clone();
        // It's longer without branching logic, but it saves time.
        if i == 0 {
            // Top left corner
            [i + 1, i + self.height, i + 1 + self.height].to_vec()
        } else if width - 1 == i {
            // Top right corner
            [i - 1, i + self.height, i - 1 + self.height].to_vec()
        } else if i + 1 == width * height - width {
            // Bottom left corner
            [i + 1, i - self.width, i + 1 - self.width].to_vec()
        } else if i + 1 == width * height {
            // Bottom right corner
            [i - 1, i - self.width, i - 1 - self.width].to_vec()
        } else if i < width {
            // Top row
            [
                i - 1,
                i + 1,
                i + self.height,
                i - 1 + self.height,
                i + 1 + self.height,
            ]
            .to_vec()
        } else if i > width * (height - 1) {
            // Bottom row
            [
                i - 1,
                i + 1,
                i - self.width,
                i - 1 - self.width,
                i + 1 - self.width,
            ]
            .to_vec()
        } else if i % width == 0 {
            // Left column
            [
                i + 1,
                i - self.width,
                i + 1 - self.width,
                i + self.height,
                i + 1 + self.height,
            ]
            .to_vec()
        } else if (i + 1) % 5 == 0 {
            // Right column
            [
                i - 1,
                i - self.width,
                i - 1 - self.width,
                i + self.height,
                i - 1 + self.height,
            ]
            .to_vec()
        } else {
            // Middle of board
            [
                i - 1,
                i + 1,
                i - self.width,
                i - 1 - self.width,
                i + 1 - self.width,
                i + self.height,
                i - 1 + self.height,
                i + 1 + self.height,
            ]
            .to_vec()
        }
    }
}
