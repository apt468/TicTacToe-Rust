use std::io;
//use std::cmp::Ordering;

const BOARD_SIZE: usize = 3;
const BOARD_SIZE_MINUS_ONE: usize = BOARD_SIZE - 1;
const BOARD_SPACES: usize = BOARD_SIZE * BOARD_SIZE;
const EMPTY_SPACE: char = '-';
const PLAYER_X: char = 'X';
const PLAYER_Y: char = 'Y';

fn main() {
    //game();
    let debug_board_a = [['X', 'X', 'X'], ['X', '-', '-'], ['X', '-', '-']];
    draw_board(&debug_board_a);
    check_victory(&debug_board_a, PLAYER_X);
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
                new_board[x_turn[0]][x_turn[1]] = PLAYER_X;
                break;
            } else {
                println!("Invalid move, pls try again");
            }
        }

        draw_board(&new_board);
        check_victory(&new_board, PLAYER_X);

        loop {
            println!("Player Y:");
            let y_turn = get_input();
            if validate_move(&new_board, &y_turn) {
                new_board[y_turn[0]][y_turn[1]] = PLAYER_Y;
                break;
            } else {
                println!("Invalid move, pls try again");
            }
        }
        check_victory(&new_board, PLAYER_Y);

        if game_won {
            break;
        }

        turn += 1;
    }
}

fn check_victory(board: &[[char; BOARD_SIZE]; BOARD_SIZE], player: char) -> bool {
    let mut i = 0;
    let mut win = false;
    let mut horizontal_check = [0; BOARD_SIZE];
    let mut vertical_check = [0; BOARD_SIZE];

    println!("DEBUG");

    for i in 1..BOARD_SIZE - 1 {
        //while i < BOARD_SIZE {
        //let mut j = 0;
        for j in 1..BOARD_SIZE - 1 {
            //while j < BOARD_SIZE {
            if board[i][j] == player {
                horizontal_check[i] += 1;
                let found = board[i][j];
                let count = horizontal_check[i];
                print!("Horizontal Check: ");
                print!("Checking Coords {i} {j}");
                println!("Found {found}, count at {count}");
            }
            if board[j][i] == player {
                vertical_check[j] += 1;
                let found = board[j][i];
                let count = horizontal_check[j];
                print!("Vertical Check: ");
                print!("Checking Coords {j} {i}");
                println!("Found {found}, count at {count}");
            }
            //j += 1;
        }
        //i += 1;
    }
    for x in horizontal_check {
        println!("Horiz Total Check: {x}");
        if x >= BOARD_SIZE {
            win = true;
        }
    }
    for x in vertical_check {
        println!("Vert Total Check: {x}");
        if x >= BOARD_SIZE {
            win = true;
        }
    }
    win
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

fn validate_move(board: &[[char; BOARD_SIZE]; BOARD_SIZE], player_move: &[usize; 2]) -> bool {
    let mut valid_move = false;
    if player_move[0] < BOARD_SIZE {
        if player_move[1] < BOARD_SIZE {
            valid_move = true;
            if board[player_move[0]][player_move[1]] == EMPTY_SPACE {
                valid_move = true;
            }
        }
    }
    valid_move
}

fn initialize_board() -> [[char; BOARD_SIZE]; BOARD_SIZE] {
    let a = [EMPTY_SPACE; BOARD_SIZE];
    let b = [a; BOARD_SIZE];
    b
}

fn draw_board(board: &[[char; BOARD_SIZE]; BOARD_SIZE]) {
    let mut i = 0;
    print!("  -");
    while i < BOARD_SIZE {
        print!(" {i} -");
        i += 1;
    }
    println!("");

    i = 0;
    while i < BOARD_SIZE {
        let mut j = 0;
        print!("{i} ");
        while j < BOARD_SIZE {
            let squar = board[i][j];
            print!("| {squar} ");
            j += 1;
        }
        i += 1;
        println!("|");
    }
    println!("");
}
