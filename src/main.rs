use rprompt::prompt_reply as input;

mod game;
use game::{Board, CellState};
fn main() {
    let mut board = Board::new();
    let mut is_player1 = true;
    board.print();
    loop {
        if is_player1 {
            board.set_cell(row, col, CellState::X).unwrap();
        } else {
            board.set_cell(row, col, CellState::O).unwrap();
        }

        board.print();
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
