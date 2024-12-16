use std::io;

const EMPTY: char = ' ';

fn print_board(board: &[Vec<char>]) {
    println!("\n");
    for row in board {
        println!(" {} | {} | {} ", row[0], row[1], row[2]);
        println!("---+---+---");
    }
    println!("\n");
}

fn is_winner(board: &[Vec<char>], player: char) -> bool {
    // Check rows, columns, and diagonals
    for i in 0..3 {
        if board[i][0] == player && board[i][1] == player && board[i][2] == player {
            return true;
        }
        if board[0][i] == player && board[1][i] == player && board[2][i] == player {
            return true;
        }
    }
    if board[0][0] == player && board[1][1] == player && board[2][2] == player {
        return true;
    }
    if board[0][2] == player && board[1][1] == player && board[2][0] == player {
        return true;
    }
    false
}

fn is_draw(board: &[Vec<char>]) -> bool {
    for row in board {
        if row.contains(&EMPTY) {
            return false;
        }
    }
    true
}

fn main() {
    let mut board = vec![vec![EMPTY; 3]; 3];
    let mut current_player = 'X';

    loop {
        print_board(&board);
        println!("Player {}'s turn!", current_player);
        
        // Get user input
        println!("Enter your move (row and column, 0-2 separated by a space): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let inputs: Vec<usize> = input
            .trim()
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();

        if inputs.len() != 2 {
            println!("Invalid input. Please enter two numbers (row and column).");
            continue;
        }

        let row = inputs[0];
        let col = inputs[1];

        if row > 2 || col > 2 || board[row][col] != EMPTY {
            println!("Invalid move. The cell is either occupied or out of bounds.");
            continue;
        }

        // Update the board
        board[row][col] = current_player;

        // Check for a winner
        if is_winner(&board, current_player) {
            print_board(&board);
            println!("Player {} wins!", current_player);
            break;
        }

        // Check for a draw
        if is_draw(&board) {
            print_board(&board);
            println!("It's a draw!");
            break;
        }

        // Switch player
        current_player = if current_player == 'X' { 'O' } else { 'X' };
    }
}
