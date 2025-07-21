use std::io;
//use std::cmp::Ordering;

const BOARD_SIZE: usize = 3;
const BOARD_SIZE_MINUS_ONE: usize = BOARD_SIZE - 1;
const BOARD_SPACES: usize = BOARD_SIZE * BOARD_SIZE;

fn main() {
    game();
}

fn game() {
    let mut new_board = initialize_board();
    let game_won = false;
    let mut turn = 1;

    loop {
        draw_board(&new_board);

        if turn >= BOARD_SPACES {
            println!("Game has been drawn, no winners...");
            break;
        }

        println!("Turn {turn}:");

        loop {
            println!("Player X:");
            let x_turn = get_input();
            if validate_move(&new_board, &x_turn) {
                new_board[x_turn[0]][x_turn[1]] = "X";
                break;
            } else {
                println!("Invalid move, pls try again");
            }
        }

        loop {
            println!("Player Y:");
            let y_turn = get_input();
            if validate_move(&new_board, &y_turn) {
                new_board[y_turn[0]][y_turn[1]] = "Y";
                break;
            } else {
                println!("Invalid move, pls try again");
            }
        }

        if game_won {
            break;
        }

        turn += 1;
    }
}

fn get_input() -> [usize; 2] {
    //loop {
    println!("Input row number (0-{BOARD_SIZE_MINUS_ONE}):");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line");

    println!("Input column number (0-{BOARD_SIZE_MINUS_ONE}):");
    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("Failed to read line");

    let x: usize = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => BOARD_SIZE,
    };
    let y: usize = match y.trim().parse() {
        Ok(num) => num,
        Err(_) => BOARD_SIZE,
    };

    let result = [x, y];
    //     continue;
    // }
    result
}

fn validate_move(
    board: &[[&'static str; BOARD_SIZE]; BOARD_SIZE],
    player_move: &[usize; 2],
) -> bool {
    let mut valid_move = false;
    if player_move[0] >= 0 && player_move[0] < BOARD_SIZE {
        if player_move[1] >= 0 && player_move[1] < BOARD_SIZE {
            valid_move = true;
            if board[player_move[0]][player_move[1]].trim() == "-" {
                valid_move = true;
            }
        }
    }
    valid_move
}

fn initialize_board() -> [[&'static str; BOARD_SIZE]; BOARD_SIZE] {
    let a = ["-"; BOARD_SIZE];
    let b = [a; BOARD_SIZE];
    b
}

fn draw_board(board: &[[&'static str; BOARD_SIZE]; BOARD_SIZE]) {
    const BOARDER_SIZE: usize = BOARD_SIZE * 2;
    let mut top_bottom = String::from("");
    let mut i = 0;
    while i < BOARDER_SIZE * 2 + 1 {
        top_bottom.push_str("-");
        i += 1;
    }

    println!("{top_bottom}");
    i = 0;
    while i < BOARD_SIZE {
        let mut j = 0;
        while j < BOARD_SIZE {
            let squar = board[i][j];
            print!("| {squar} ");
            j += 1;
        }
        i += 1;
        println!("|");
    }
    println!("{top_bottom}");
}
