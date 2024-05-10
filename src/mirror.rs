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

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Unable to readline");

    let input: char = input.trim().parse().expect("Invalid data received");

    let mut player_turn = if input == 'y' { true } else { false };

    let mut x_is_next = true;

    clear_screen();
    display_board(board);

    loop {
        let mut index = String::new();

        if player_turn {
            println!("Make your move:");

            io::stdin().read_line(&mut index).expect("Unable to readline");
                
        } else {
            let a = rust_actions(board);

            let result = rust_move(&a);

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

        board = new_board; // Update the board

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

// Scroll to the top
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

fn rust_move(a: &Vec<usize>) -> usize {
    let result = rand::thread_rng().gen_range(0..a.len());
    if a[result] == 5 {
        rust_move(a);
    } else {
        a[result]
    }
}

