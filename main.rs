use std::{io, process::exit};
fn main() {
    let mut board = vec![vec![0; 3]; 3];
    imprimir(&board);
    loop {
        escolher(1, &mut board);
        imprimir(&board);
        match is_solved(&board) {
            1 => {
                println!("X venceu");
                exit(1);
            }
            2 => {
                println!("O venceu");
                exit(1);
            }
            -1 => (),
            _ => {
                println!("Deu velha");
                exit(1);
            }
        }
        escolher(2, &mut board);
        imprimir(&board);
        match is_solved(&board) {
            1 => {
                println!("X venceu");
                exit(1);
            }
            2 => {
                println!("O venceu");
                exit(1);
            }
            -1 => (),
            _ => {
                println!("Deu velha");
                exit(1);
            }
        }
    }
}

fn imprimir(board: &[Vec<u8>]) {
    for row in board {
        for col in row {
            if *col == 1u8 {
                print!(" X |");
            } else if *col == 2u8 {
                print!(" O |");
            } else {
                print!("   |");
            }
        }
        println!("\n");
    }
}

fn escolher(jogador: u8, board: &mut [Vec<u8>]) {
    println!("Jogador {}:\n", jogador);
    println!("Escolha sua linha e coluna\n");
    let mut linha = String::new();
    io::stdin()
        .read_line(&mut linha)
        .expect("Failed to read line");
    let x: usize = linha.trim().parse().expect("Input not a integer");
    let mut coluna = String::new();
    io::stdin()
        .read_line(&mut coluna)
        .expect("Failed to read line");
    let y: usize = coluna.trim().parse().expect("Input not a integer");
    board[x - 1][y - 1] = jogador;
}

fn is_solved(board: &[Vec<u8>]) -> i8 {
    let mut r = vec![];
    r.push(board[0][0] * board[1][1] * board[2][2]);
    r.push(board[0][2] * board[1][1] * board[2][0]);
    r.push(board[0][0] * board[0][1] * board[0][2]);
    r.push(board[1][0] * board[1][1] * board[1][2]);
    r.push(board[2][0] * board[2][1] * board[2][2]);
    r.push(board[0][0] * board[1][0] * board[2][0]);
    r.push(board[0][1] * board[1][1] * board[2][1]);
    r.push(board[0][2] * board[1][2] * board[2][2]);
    if r.contains(&1) {
        1
    } else if r.contains(&8) {
        2
    } else if r.contains(&0) {
        -1
    } else {
        0
    }
}
