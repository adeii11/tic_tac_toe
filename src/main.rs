use std::io::{self, Write};

const EMPTY: char = ' ';
const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';

struct Board {
    grid: [[char; 3]; 3],
}

impl Board {
    fn new() -> Self {
        Board {
            grid: [[EMPTY; 3]; 3],
        }
    }

    fn print(&self) {
        println!("-------------");
        for row in &self.grid {
            for &cell in row {
                print!("| {} ", cell);
            }
            println!("|\n-------------");
        }
    }

    fn place_marker(&mut self, row: usize, col: usize, marker: char) -> bool {
        if row < 3 && col < 3 && self.grid[row][col] == EMPTY {
            self.grid[row][col] = marker;
            true
        } else {
            false
        }
    }

    fn check_winner(&self) -> Option<char> {
        // Check rows and columns
        for i in 0..3 {
            if self.grid[i][0] == self.grid[i][1] && self.grid[i][1] == self.grid[i][2] && self.grid[i][0] != EMPTY {
                return Some(self.grid[i][0]);
            }
            if self.grid[0][i] == self.grid[1][i] && self.grid[1][i] == self.grid[2][i] && self.grid[0][i] != EMPTY {
                return Some(self.grid[0][i]);
            }
        }

        // Check diagonals
        if self.grid[0][0] == self.grid[1][1] && self.grid[1][1] == self.grid[2][2] && self.grid[0][0] != EMPTY {
            return Some(self.grid[0][0]);
        }
        if self.grid[0][2] == self.grid[1][1] && self.grid[1][1] == self.grid[2][0] && self.grid[0][2] != EMPTY {
            return Some(self.grid[0][2]);
        }

        None
    }

    fn is_full(&self) -> bool {
        self.grid.iter().all(|row| row.iter().all(|&cell| cell != EMPTY))
    }
}

fn main() {
    let mut board = Board::new();
    let mut current_player = PLAYER_X;

    loop {
        board.print();
        println!("Player {}, enter your move (row and column): ", current_player);

        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let mut coords = input.trim().split_whitespace();
        if let (Some(row), Some(col)) = (coords.next(), coords.next()) {
            if let (Ok(row), Ok(col)) = (row.parse::<usize>(), col.parse::<usize>()) {
                if board.place_marker(row, col, current_player) {
                    if let Some(winner) = board.check_winner() {
                        board.print();
                        println!("Player {} wins!", winner);
                        break;
                    } else if board.is_full() {
                        board.print();
                        println!("It's a draw!");
                        break;
                    }
                    current_player = if current_player == PLAYER_X { PLAYER_O } else { PLAYER_X };
                } else {
                    println!("Invalid move! Try again.");
                }
            } else {
                println!("Invalid input! Please enter numbers.");
            }
        } else {
            println!("Please enter both row and column numbers.");
        }
    }
}
