/*
*    My attempt with rust to create a tic-tac-toe game
*    that has AI implementations and multiplayer
*
*    This is just to test my skills and knowledge
*    on rust
*/

use rand::prelude::SliceRandom;
use rand::Rng;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    // Initialize the board
    let mut board = [' '; 9];
    let mut x_is_next = true;

    println!("\n Play Tic-Tac-Toe with \x1b[91mRust\x1b[0m");

    // Choosing difficulty level
    let rust_level = loop {
        println!("\nChoose your Difficulty: \n\n  E(easy)\n  M(medium)\n  H(hard) \n\nMake your choice:");

        let mut rust_level = String::new();

        io::stdin()
            .read_line(&mut rust_level)
            .expect("Unable to read line");

        let rust_level: char = rust_level.trim().to_ascii_uppercase().parse().unwrap_or(' ');

        if "EMH".contains(rust_level) {
            break rust_level;
        }
    };

    // Prompt user to choose play mode
    println!("Do you want to play as X? (y/n)");

    let mut player_is_x = String::new();

    io::stdin()
        .read_line(&mut player_is_x)
        .expect("Unable to read line");

    let player_is_x: char = player_is_x.trim().parse().expect("Invalid data received");

    let mut player_turn = player_is_x == 'y';

    clear_screen();
    display_board(board);

    loop {
        let mut index = String::new();

        if player_turn {
            println!("Make your move (1-9):");

            io::stdin()
                .read_line(&mut index)
                .expect("Unable to read line");

            let index = string_to_num(index.clone());

            if index < 1 || index > 9 || board[index - 1] != ' ' {
                println!("\n\x1b[91mInvalid move\nTry again\x1b[0m\n");
                continue;
            }

        } else {
            let actions = rust_actions(board);

            let move_index = match rust_level {
                'E' => easy_rust(&actions),
                'M' => mid_rust(&actions, board, player_is_x == 'y'),
                'H' => hard_rust(&actions, board, player_is_x == 'y'),
                _ => 0,
            };

            println!("Rust is thinking...");

            // Sleep for 3 seconds
            let d = Duration::from_secs(3);
            thread::sleep(d);

            index = (move_index + 1).to_string(); // AI move index
        }

        let index: usize = index.trim().parse().expect("Invalid data received");

        let player = if x_is_next { 'X' } else { 'O' };

        let new_board = make_move(board, index - 1, player);

        clear_screen();
        display_board(new_board);

        if check_game(new_board, player) {
            if player_turn {
                println!("\x1b[92m{} wins!\x1b[0m", player);
            } else {
                println!("\x1b[91m{} wins!\x1b[0m", player);
            }
            break;
        } else if new_board.iter().all(|&cell| cell != ' ') {
            println!("\x1b[93mIt's a Draw\x1b[0m");
            break;
        }

        board = new_board; // Update the board state

        x_is_next = !x_is_next;
        player_turn = !player_turn;
    }
}

// Display board
fn display_board(b: [char; 9]) {
    println!();
    for (i, cell) in b.iter().enumerate() {
        print!(" {} ", cell);
        if i == 2 || i == 5 {
            println!();
            println!("-----------");
        } else if i == 8 {
            println!();
        } else {
            print!("|");
        }
    }
    println!();
}

// Make a move on the board
fn make_move(mut b: [char; 9], i: usize, p: char) -> [char; 9] {
    b[i] = p;
    b
}

// Clear the screen
fn clear_screen() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
}

// Determine available moves for Rust in the board
fn rust_actions(b: [char; 9]) -> Vec<usize> {
    b.iter()
        .enumerate()
        .filter(|&(_, &cell)| cell == ' ')
        .map(|(i, _)| i)
        .collect()
}

// Check if the game has been won
fn check_game(b: [char; 9], p: char) -> bool {
    let combinations = [
        [0, 1, 2], /********/
        [3, 4, 5], /* rows */
        [6, 7, 8], /********/
        
        [0, 3, 6], /***********/
        [1, 4, 7], /* columns */
        [2, 5, 8], /***********/
        
        [0, 4, 8], /* diagonals */
        [2, 4, 6], /*************/
    ];

    for combo in combinations.iter() {
        if combo.iter().all(|&i| b[i] == p) {
            return true;
        }
    }
    false
}

// Convert string to number
fn string_to_num(s: String) -> usize {
    s.trim().parse().unwrap_or(0)
}


// Rust AI Difficulty levels...
fn hard_rust(a: &Vec<usize>, b: [char; 9], player_is_x: bool) -> usize {
    let human = if player_is_x { 'X' } else { 'O' };
    let hor_ver_arr = [1, 3, 5, 7]; // Defense cells
    let dia_arr = [0, 2, 6, 8]; // Attack cells

    if b.iter().filter(|&&cell| cell == 'X').count() == 1 && player_is_x && b[4] == ' ' {
        return 4;
    } else if b.iter().filter(|&&cell| cell == 'X').count() == 1 && player_is_x && b[4] != ' ' {
        let result = rand::thread_rng().gen_range(0..hor_ver_arr.len());
        return dia_arr[result];
    } else {
        return mid_rust(a, b, player_is_x);
    }

    if b.iter().filter(|&&cell| cell == 'O').count() == 1 && player_is_x {
        let result = rand::thread_rng().gen_range(0..hor_ver_arr.len());

        for &i in a {
            let mut test_board = b.clone();
            test_board[i] = human;
    
            if check_game(test_board, human) {
                return i;
            }
        }
        return hor_ver_arr[result];
    }

    if b.iter().filter(|&&cell| cell == 'X').count() < 2 && !player_is_x {
        let emp: Vec<usize> = dia_arr.iter().filter(|&&i| b[i] == ' ').cloned().collect();

        if !emp.is_empty() {
            let mut rng = rand::thread_rng();
            let mut shuff = emp;
            shuff.shuffle(&mut rng);

            return shuff[0];
        }
    }

    return mid_rust(a, b, player_is_x);
}

fn mid_rust(a: &Vec<usize>, b: [char; 9], player_is_x: bool) -> usize {
    let human = if player_is_x { 'X' } else { 'O' };
    let rust = if player_is_x { 'O' } else { 'X' };

    for &i in a {
        let mut test_board = b.clone(); // create clone board for testing
        test_board[i] = rust;

        if check_game(test_board, rust) {
            return i;
        }
    }

    for &i in a {
        let mut test_board = b.clone();
        test_board[i] = human;

        if check_game(test_board, human) {
            return i;
        }
    }

    return easy_rust(a);
}

fn easy_rust(a: &Vec<usize>) -> usize {
    let result = rand::thread_rng().gen_range(0..a.len());
    a[result]
}
