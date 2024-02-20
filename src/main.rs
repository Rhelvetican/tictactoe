use array2d::{Array2D, Error};
use rprompt::prompt_reply as input;
use std::fmt::Display;

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    X,
    O,
    Unoccupied,
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CellState::X => write!(f, "X"),
            CellState::O => write!(f, "O"),
            CellState::Unoccupied => write!(f, " "),
        }
    }
}

struct Board {
    cells: Array2D<CellState>,
}

impl Board {
    fn new() -> Self {
        Board {
            cells: Array2D::filled_with(CellState::Unoccupied, 3, 3),
        }
    }
    fn set_cell(&mut self, row: usize, col: usize, state: CellState) -> Result<(), Error> {
        if row > 2 || col > 2 {
            println!("Invalid row or column");
            Err(Error::IndexOutOfBounds(row))
        } else {
            if self.cells[(row, col)] != CellState::Unoccupied {
                println!("Cell already occupied");
                return Err(Error::IndexOutOfBounds(row));
            }
            self.cells[(row, col)] = state;
            Ok(())
        }
    }
    fn check_win(&self) -> Option<CellState> {
        // Check rows
        for i in 0usize..=2usize {
            let symbol = self.cells[(i, 0)];
            if symbol != CellState::Unoccupied
                && symbol == self.cells[(i, 1)]
                && symbol == self.cells[(i, 2)]
            {
                return Some(symbol);
            }
        }

        // Check columns
        for i in 0usize..=2usize {
            let symbol = self.cells[(0, i)];
            if symbol != CellState::Unoccupied
                && symbol == self.cells[(1, i)]
                && symbol == self.cells[(2, i)]
            {
                return Some(symbol);
            }
        }

        // Check diagonals
        let symbol_top_left = self.cells[(0, 0)];
        if symbol_top_left != CellState::Unoccupied
            && symbol_top_left == self.cells[(1, 1)]
            && symbol_top_left == self.cells[(2, 2)]
        {
            return Some(symbol_top_left);
        }
        let symbol_top_right = self.cells[(0, 2)];
        if symbol_top_right != CellState::Unoccupied
            && symbol_top_right == self.cells[(1, 1)]
            && symbol_top_right == self.cells[(2, 0)]
        {
            return Some(symbol_top_right);
        }

        None
    }
    fn print(&self) {
        for row in 0usize..=2usize {
            for col in 0usize..=2usize {
                print!("{}", self.cells[(row, col)]);
                if col < 2 {
                    print!("|");
                }
            }
            println!();
            if row < 2 {
                println!("-----");
            }
        }
    }
}

fn main() {
    let mut board = Board::new();
    let mut is_player1 = true;
    loop {
        board.print();
        let row = input("Enter row: ").unwrap().parse::<usize>().unwrap() - 1usize;
        let col = input("Enter column: ").unwrap().parse::<usize>().unwrap() - 1usize;
        if is_player1 {
            board.set_cell(row, col, CellState::X).unwrap();
        } else {
            board.set_cell(row, col, CellState::O).unwrap();
        }

        match board.check_win() {
            Some(CellState::X) => {
                println!("Player 1 wins!");
                break;
            }
            Some(CellState::O) => {
                println!("Player 2 wins!");
                break;
            }
            Some(CellState::Unoccupied) => (),
            None => (),
        };
        for (x, y) in [
            (0usize, 0usize),
            (1usize, 0usize),
            (2usize, 0usize),
            (0usize, 1usize),
            (1usize, 1usize),
            (2usize, 1usize),
            (0usize, 2usize),
            (1usize, 2usize),
            (2usize, 2usize),
        ] {
            if board.cells[(x, y)] == CellState::Unoccupied {
                break;
            }
            if x == 2 && y == 2 {
                println!("It's a draw!");
                return;
            }
        }
        is_player1 = !is_player1;
    }
}
