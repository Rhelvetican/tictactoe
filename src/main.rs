mod game;
use game::{Board, CellState};
use rand::{thread_rng, Rng};
fn main() {
    let mut board = Board::new();
    let mut is_player1 = thread_rng().gen_bool(0.5);
    board.print();

    // Game loop
    loop {
        if is_player1 {
            board.set_cell(CellState::X).unwrap();
        } else {
            board.set_cell(CellState::O).unwrap();
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
