// building a simple tic tac to game

fn main() {
    let board: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    draw_board(&board);
}

// defining board layout
fn draw_board(board: &[char]) {
    for (i, cell) in board.iter().enumerate() {
        print!(" {} ", cell);

        if (i + 1) % 3 == 0 {
            println!();

            if i < 8 {
                println!("---+---+---")
            };
        } else {
            print!("|");
        }
    }
}
