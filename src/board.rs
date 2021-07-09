use petname;
use rand;
use rand_seeder::{Seeder, SipRng};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Cell {
    #[serde(skip)]
    mine: bool,
    flag: bool,
    reveal: bool,
    #[serde(skip)]
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

    fn toggle_mine(&mut self) {
        self.mine = !self.mine;
    }

    fn toggle_flag(&mut self) -> bool {
        if self.reveal {
            false
        } else {
            self.flag = !self.flag;
            true
        }
    }

    fn set_reveal(&mut self, value: bool) -> bool {
        if self.flag {
            false
        } else {
            self.reveal = value;
            true
        }
    }

    fn toggle_reveal(&mut self) -> bool {
        if self.flag {
            false
        } else {
            self.reveal = !self.reveal;
            true
        }
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

#[derive(Debug, Clone, Serialize)]
pub struct Board {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    difficulty: usize,
    pub seed: String,
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: Vec::new(),
            width: 0,
            height: 0,
            difficulty: 0,
            seed: "".to_string(),
        }
    }

    pub fn from(state: String) -> Board {
        let mut string_state = state.split_whitespace();
        let width: usize = string_state.next().unwrap().parse::<usize>().unwrap();
        let height: usize = string_state.next().unwrap().parse::<usize>().unwrap();
        let difficulty: usize = string_state.next().unwrap().parse::<usize>().unwrap();
        let seed: String = string_state.next().unwrap().to_string();
        let board: Vec<String> = string_state.into_iter().map(|x| x.to_string()).collect();
        let mut b = Board::new()
            .with_width(width)
            .with_height(height)
            .with_difficulty(difficulty)
            .with_seed(seed)
            .build();
        for (i, cell) in board.iter().enumerate() {
            let (reveal, flag): (bool, bool) = match cell.as_str() {
                "00" => (false, false),
                "01" => (false, true),
                "10" => (true, false),
                "11" => (true, true),
                _ => (false, false),
            };
            b.cells[i].reveal = reveal;
            b.cells[i].flag = flag;
        }
        b
    }

    pub fn export_state(&self) -> String {
        let mut state = String::new();
        state.push_str(&format!(
            "{} {} {}\n",
            self.width, self.height, self.difficulty
        ));
        state.push_str(&format!("{}\n", self.seed));
        state.push_str(
            &self
                .cells
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    let mut c: String = String::new();
                    if x.reveal {
                        c.push_str("1");
                    } else {
                        c.push_str("0");
                    }
                    if x.flag {
                        c.push_str("1");
                    } else {
                        c.push_str("0");
                    }
                    c.push_str(" ");
                    if (i + 1) % self.width == 0 {
                        c.push_str("\n");
                    }
                    c
                })
                .collect::<String>(),
        );
        state
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

    pub fn with_seed(mut self, seed: String) -> Self {
        self.seed = seed;
        self
    }

    pub fn build(&mut self) -> Self {
        let width = self.width.clone();
        let height = self.height.clone();
        let difficulty = self.difficulty.clone();

        if self.seed == "".to_string() {
            self.seed = petname::petname(4, "-")
        }

        let mut rng: SipRng = Seeder::from(self.seed.clone() + "my_salty_salt").make_rng();
        let mines = rand::seq::index::sample(&mut rng, width * height, difficulty).into_vec();
        let cells: Vec<Cell> = (0..self.width * self.height)
            .into_iter()
            .map(|i| {
                //println!("{:#?}", self.clone().adjacent_cells(i));
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
        self.clone()
    }

    fn adjacent_cells(self, i: usize) -> Vec<usize> {
        let width = self.width.clone();
        let height = self.height.clone();
        let mut tiles: Vec<usize> = Vec::new();
        // It's longer without branching logic, but it saves time.
        if i == 0 {
            // Top left corner
            tiles = [i + 1, i + self.height, i + 1 + self.height].to_vec();
        } else if width - 1 == i {
            // Top right corner
            tiles = [i - 1, i + self.height, i - 1 + self.height].to_vec();
        } else if i == width * height - width {
            // Bottom left corner
            tiles = [i + 1, i - self.width, i + 1 - self.width].to_vec();
        } else if i + 1 == width * height {
            // Bottom right corner
            tiles = [i - 1, i - self.width, i - 1 - self.width].to_vec();
        } else if i < width {
            // Top row
            tiles = [
                i - 1,
                i + 1,
                i + self.height,
                i - 1 + self.height,
                i + 1 + self.height,
            ]
            .to_vec();
        } else if i > width * (height - 1) {
            // Bottom row
            tiles = [
                i - 1,
                i + 1,
                i - self.width,
                i - 1 - self.width,
                i + 1 - self.width,
            ]
            .to_vec();
        } else if i % width == 0 {
            // Left column
            tiles = [
                i + 1,
                i - self.width,
                i + 1 - self.width,
                i + self.height,
                i + 1 + self.height,
            ]
            .to_vec();
        } else if (i + 1) % width == 0 {
            // Right column
            tiles = [
                i - 1,
                i - self.width,
                i - 1 - self.width,
                i + self.height,
                i - 1 + self.height,
            ]
            .to_vec();
        } else {
            // Middle of board
            tiles = [
                i - 1,
                i + 1,
                i - self.width,
                i - 1 - self.width,
                i + 1 - self.width,
                i + self.height,
                i - 1 + self.height,
                i + 1 + self.height,
            ]
            .to_vec();
        }
        // println!("{}:{:?}", i, tiles);
        tiles
    }

    pub fn flag(&mut self, x: usize, y: usize) -> bool {
        let i = (y * self.width) + x;
        self.flag_by_index(i)
    }

    pub fn flag_by_index(&mut self, i: usize) -> bool {
        self.cells[i].toggle_flag()
    }
    pub fn reveal(&mut self, x: usize, y: usize) -> bool {
        let i = (y * self.width) + x;
        self.reveal_by_index(i)
    }

    pub fn reveal_by_index(&mut self, i: usize) -> bool {
        match self.cells[i].set_reveal(true) {
            false => false,
            true => {
                if self.cells[i].neighbors == 0 {
                    let neighbors = self.clone().adjacent_cells(i);
                    for x in neighbors {
                        if !self.cells[x].reveal {
                            self.reveal_by_index(x);
                        }
                    }
                }
                true
            }
        }
    }

    pub fn reveal_all(&mut self) -> bool {
        for i in 0..self.width * self.height {
            self.cells[i].set_reveal(true);
        }
        true
    }

    pub fn lost(&self) -> bool {
        self.cells
            .iter()
            .fold(false, |acc, x| acc | x.mine & x.reveal)
    }

    pub fn won(&self) -> bool {
        self.cells
            .iter()
            .fold(true, |acc, x| acc & ((!x.mine) || x.flag))
    }

    pub fn export_board(&self) -> String {
        self.cells
            .iter()
            .enumerate()
            .map(|(i, x)| {
                let mut c: String = String::new();
                if !x.reveal {
                    c.push_str(".");
                } else if x.mine {
                    c.push_str("M");
                } else if x.flag {
                    c.push_str("F");
                } else {
                    c.push_str(&x.neighbors.to_string());
                }
                if (i + 1) % self.width == 0 {
                    c.push_str("\n");
                }
                c
            })
            .collect()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cell_board: String = self
            .cells
            .iter()
            .enumerate()
            .map(|(i, x)| {
                let mut c: String = String::new();
                if i % self.width == 0 {
                    c.push_str(&(i / self.width).to_string());
                    c.push_str("|");
                }
                if !x.reveal {
                    c.push_str(".");
                } else if x.mine {
                    c.push_str("M");
                } else if x.flag {
                    c.push_str("F");
                } else {
                    c.push_str(&x.neighbors.to_string());
                }
                if (i + 1) % self.width == 0 {
                    c.push_str("\n");
                }
                c
            })
            .collect();
        cell_board.push_str("  ");
        cell_board.push_str(&(0..self.width).into_iter().map(|_| "-").collect::<String>());
        cell_board.push_str("\n  ");
        cell_board.push_str(
            &(0..self.width)
                .into_iter()
                .map(|x| x.to_string())
                .collect::<String>(),
        );
        write!(f, "{}", cell_board)
    }
}
