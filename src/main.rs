// building a simple tic tac to game
use colored::*;
use std::io::{self, Write};

fn main() {
    println!("\n{}", "tic tac toe".blue());

    let mut board: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut is_x_turn = true;

    loop {
        draw_board(&board);
        let player = if is_x_turn { "X" } else { "O" };

        println!("\n{}'s turn.\n", player.blue());
        print!("Select a position or type '{}' to quit: ", "q".red());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // quitting the game...
        if input.trim() == "q" {
            println!("\nSee you next time!😊");
            break;
        };

        let val = match input.trim().parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "please input a valid number".yellow());
                continue;
            }
        };

        if val < 1 || val > 9 {
            println!("{}", "Please enter a value between 1 and 9".yellow());
            continue;
        }

        let index = val - 1;

        if board[index] == 'X' || board[index] == 'O' {
            println!("{}", "That spot is already taken.".yellow());
            continue;
        };

        board[index] = player.chars().next().unwrap();
        is_x_turn = !is_x_turn; // switching turns here...
    }
}

// defining board layout

fn draw_board(board: &[char]) {
    println!();

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
