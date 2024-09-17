use std::io::{self, Write};
use std::process;

type BoardType = [[char; 3]; 3];

fn print_board(board: &BoardType) {
    println!("+-----------+");
    println!("| Rusty XO! |");
    println!("+-----------+");
    for row in board.iter() {
        println!("| {} | {} | {} |", row[0], row[1], row[2]);
        println!("+-----------+");
    }
    println!("| q to exit |");
    println!("+-----------+");
}

fn find_winner(board: &BoardType, round_count: &u8) {
    let winning_lines = [
        (0, 1, 2),
        (3, 4, 5),
        (6, 7, 8),
        (0, 3, 6),
        (1, 4, 7),
        (2, 5, 8),
        (0, 4, 8),
        (2, 4, 6),
    ];
    if *round_count == 9 {
        println!("It's a draw!");
        process::exit(0);
    } else {
        for &(a, b, c) in &winning_lines {
            let (row_a, col_a) = (a / 3, a % 3);
            let (row_b, col_b) = (b / 3, b % 3);
            let (row_c, col_c) = (c / 3, c % 3);

            let char_a = board[row_a][col_a];
            let char_b = board[row_b][col_b];
            let char_c = board[row_c][col_c];

            if char_a == char_b && char_b == char_c {
                let player = if char_a == 'X' { 1 } else { 2 };
                println!("Player {} wins!", player);
                process::exit(0);
            }
        }
    }
}

fn find(board: &BoardType, ind: &char) -> Option<(usize, usize)> {
    for (ir, row) in board.iter().enumerate() {
        for (ic, cell) in row.iter().enumerate() {
            if cell == ind {
                return Some((ir, ic));
            }
        }
    }
    None
}

fn round(board: &mut BoardType, round_count: &mut u8) {
    let mut inp = String::new();
    print!(
        "Player {} > 0-9 | q > ",
        if *round_count % 2 == 0 { 1 } else { 2 }
    );
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line");
    if inp.trim() == "q" {
        println!("Thank you!");
        process::exit(0);
    }
    let inp: char = inp.chars().next().expect("Not a valid input");
    if '1' <= inp && inp <= '9' {
        match find(board, &inp) {
            Some((ir, ic)) => {
                let cell = &mut board[ir][ic];
                *cell = if *round_count % 2 == 0 { 'X' } else { 'O' };
                *round_count += 1;
            }
            None => println!("Cell is already assigned"),
        }
    } else {
        println!("Enter index between 1 - 9");
    }
}

fn main() {
    let mut board: BoardType = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];
    let mut round_count = 0;
    print_board(&board);
    loop {
        round(&mut board, &mut round_count);
        print_board(&board);
        find_winner(&board, &round_count);
    }
}
