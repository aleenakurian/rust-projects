// Connect 4: implementing game AI using Minimax and alpha-beta pruning
use std::io;

fn print_board(game_board: &[[char; 7]; 6]) {
    let mut i = 0;
    let mut col = 1;

    //to print column numbers
    while col <= 7 {
        print!(" {}  ", col);
        col += 1;
    }
    println!();

    //printing the board

    //printing the border between each row
    while i < 6 { 
        print!("+");
        let mut j = 0;
        while j < 7 {
            print!("---+");
            j += 1;
        }
        println!();

        //printing the cells
        j = 0;
        while j < 7 {
            print!("| {} ", game_board[i][j]);
            j += 1;
        }

        println!("|"); //the very right border

        i += 1; //increment by row
    }

    // the bottom border
    print!("+");
    let mut j = 0;
    while j < 7 {
        print!("---+");
        j += 1;
    }
    println!();

}

fn insert_board(game_board: &mut [[char; 7]; 6], input:String, current_player:char)
{
    let mut row:usize=5;
    let col: usize = input.trim().parse::<usize>().unwrap() - 1;

loop {
        if game_board[row][col]==' '
        {
            game_board[row][col] = current_player;
            break;
        }
        if row == 0{
            break;
        }
        row -= 1;
}
}

fn check_win(game_board: &[[char; 7]; 6]){

}
fn main() {
    let mut game_board:[[char;7];6]=[[' ';7];6];
    let mut current_player: char='X';

    println!("CONNECT 4");
    loop{
    print_board(&mut game_board);

    println!("Enter the column to drop to (1-7):");
    let mut input: String = String::new();
    let _ = io::stdin().read_line(&mut input).expect("Failed to read input.");

    insert_board(&mut game_board, input, current_player);
    check_win(&mut game_board);

    if current_player=='X'{
        current_player='O';
    }
    else {
        current_player='X';
    }

}
}