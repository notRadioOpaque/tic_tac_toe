// building a simple tic tac to game
use colored::*;
use std::io::{self, Write};

fn main() {
    println!("\n{}", "

████████╗██╗ ██████╗      ████████╗ █████╗  ██████╗      ████████╗ ██████╗ ███████╗
╚══██╔══╝██║██╔════╝      ╚══██╔══╝██╔══██╗██╔════╝      ╚══██╔══╝██╔═══██╗██╔════╝
   ██║   ██║██║     █████╗   ██║   ███████║██║     █████╗   ██║   ██║   ██║█████╗  
   ██║   ██║██║     ╚════╝   ██║   ██╔══██║██║     ╚════╝   ██║   ██║   ██║██╔══╝  
   ██║   ██║╚██████╗         ██║   ██║  ██║╚██████╗         ██║   ╚██████╔╝███████╗
   ╚═╝   ╚═╝ ╚═════╝         ╚═╝   ╚═╝  ╚═╝ ╚═════╝         ╚═╝    ╚═════╝ ╚══════╝
                                                                                   

".blue());

    let mut board: [String; 9] = [
        "1".into(),
        "2".into(),
        "3".into(),
        "4".into(),
        "5".into(),
        "6".into(),
        "7".into(),
        "8".into(),
        "9".into(),
    ];

    let mut is_x_turn = true;
    let mut is_game_won = false;
    let mut turns = 9;

    draw_board(&board);

    while !is_game_won {
        let player_symbol = if is_x_turn { "X" } else { "O" };
        let player_display = if is_x_turn {
            player_symbol.blue()
        } else {
            player_symbol.green()
        };

        println!("\n{}'s turn.\n", player_display);
        print!("Select a position or type '{}' to quit: ", "q".red());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

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

        if board[index] == "X" || board[index] == "O" {
            println!("{}", "That spot is already taken.".yellow());
            continue;
        };

        board[index] = player_symbol.to_string();

        turns -= 1;

        draw_board(&board);

        match check_winner(&board) {
            Some(winner) => {
                is_game_won = true;

                if winner == "X" {
                    println!();
                    println!("{}", "X wins ✨".bold().blue());
                    println!();
                } else {
                    println!();
                    println!("{}", "O wins ✨".bold().green());
                    println!();
                }
            }
            None => {
                if turns == 0 {
                    println!();
                    println!("{}", "Draw! Well played, go harder next time 💪".yellow());
                    println!();
                    break;
                }

                is_x_turn = !is_x_turn;
            }
        }
    }
}

// defining board layout

fn draw_board(board: &[String]) {
    println!();

    for (i, cell) in board.iter().enumerate() {
        let display = match cell.as_str() {
            "X" => cell.blue().bold().to_string(),
            "O" => cell.green().bold().to_string(),
            _ => cell.clone(),
        };

        print!(" {} ", display);

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

fn check_winner(board: &[String; 9]) -> Option<String> {
    let wins = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
        [1, 4, 7],
        [2, 5, 8],
        [3, 6, 9],
        [1, 5, 9],
        [3, 5, 7],
    ];

    for &line in &wins {
        let [a, b, c] = line;
        let (i, j, k) = (a - 1, b - 1, c - 1); // converting to 0 based index

        if board[i] == board[j] && board[j] == board[k] && (board[i] == "X" || board[i] == "O") {
            return Some(board[i].clone());
        }
    }

    None
}
