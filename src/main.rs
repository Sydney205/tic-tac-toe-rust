use rand::Rng;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    // Initialize the game's board
    let mut board: [char; 9] = [
        ' ', ' ', ' ', // Just a fun code format
        ' ', ' ', ' ', // in the structure of the board
        ' ', ' ', ' ', // lol... just for fun.
    ];

    // Prompt user to chose play mode
    println!("Do you want to play as X?");

    let mut player_is_x = String::new();

    io::stdin()
        .read_line(&mut player_is_x)
        .expect("Unable to readline");

    let player_is_x: char = player_is_x.trim().parse().expect("Invalid data received");

    let mut player_turn = if player_is_x == 'y' { true } else { false };

    let mut x_is_next = true;

    clear_screen();
    display_board(board);

    loop {
        let mut index = String::new();

        if player_turn {
            println!("Make your move:");

            io::stdin()
                .read_line(&mut index)
                .expect("Unable to readline");
        } else {
            let mut first_move = false;
            if board == [' '; 9] {
                first_move = true;
            }

            let a = rust_actions(board, if player_is_x == 'y' { 'O' } else { 'X' });

            let result = rust_move(&a, first_move);

            println!("Rust is thinking...");

            // Sleep for 3sec
            let d = Duration::from_secs(3);
            thread::sleep(d);

            index = result.to_string();
        }

        let index = index.trim().parse().expect("Invalid data received");

        let player = if x_is_next { 'X' } else { 'O' };

        let new_board = make_move(board, if player_turn { index - 1 } else { index }, player);

        clear_screen();
        display_board(new_board);

        board = new_board; // Update the board state

        x_is_next = !x_is_next;
        player_turn = !player_turn;
    }

    fn make_move(mut bd: [char; 9], a: usize, b: char) -> [char; 9] {
        bd[a] = b;
        bd
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

// Render a fresh screen
fn clear_screen() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
}

// Determine available moves for rust in the board
fn rust_actions(b: [char; 9], s: char) -> Vec<usize> {
    let mut indices = Vec::new();
    for (i, cell) in b.iter().enumerate() {
        if *cell == ' ' {
            if s == 'X' && (i + 1) % 2 != 0 {
                indices.push(i);
            }
        }
    }
    indices
}

fn rust_move(a: &Vec<usize>, f: bool) -> usize {
    let result = rand::thread_rng().gen_range(0..a.len());
    if f {
        if a[result] == 4 {
            rust_move(a, f)
        } else {
            a[result]
        }
    } else {
        a[result]
    }
}

fn check_game(b: [char; 9]) -> usize {}
