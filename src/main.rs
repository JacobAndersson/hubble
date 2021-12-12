use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;
use shakmaty::{fen, Chess, Position, san::San};
use shakmaty::san::ParseSanError;
use shakmaty::Setup;

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn apply_move(board: Chess, mv: &str) -> Result<Chess, ParseSanError> {
    let san: San  = mv.parse()?;
    println!("SAN {}", &san);
    //let m = .unwrap();
    //let m = san.to_mov
    if let Ok(m) = san.to_move(&board) {
        return Ok(board.play(&m).unwrap())
    } else {
        println!("{}", &board.turn());
        println!("{:?}", san);
        println!("{:?}", &board);
        println!("{}", fen::epd(&board));
        //panic!("WHAT");
        return Err(ParseSanError {})
    }
}

fn split_moves(board: Chess, mv: &str) -> Result<Chess, String> {
    for i in 0..mv.len() {
        match apply_move(board.clone(), &mv[0..(i+1)]) {
            Ok(b) => {
                if let Ok(bd) = apply_move(b, &mv[i+1..]) {
                    return Ok(bd);
                }
            },
            Err(_) => {}
        }
    }
    Err("Could not split moves".to_string())
}

fn handle_moves(move_string: &String) -> Result<(), ParseSanError>{
    println!("#####NEW GAME#####");
    let moves = move_string.split(" ");
    
    let number_check = Regex::new(r"[0-9]\.").unwrap();
    let mut board = Chess::default();

    for m in moves {
        if !m.is_empty(){
            let mut mv = match m.find('.') {
                Some(idx) => {
                    &m[(idx+1)..]
                },
                None => {
                    m
                }
            };

            mv = match mv.find("O-") {
                Some(idx) => {
                    let first_move = &mv[0..idx];
                    board = apply_move(board, first_move)?;
                    &mv[idx..]
                },
                None => {
                    mv
                }
            };

            if mv.len() < 2 {
                continue
            }

            println!("og {} after {:?}", m, mv);
            board = match apply_move(board.clone(), mv) {
                Ok(b) => b,
                Err(_) => {
                    split_moves(board, mv).unwrap()
                }
            }
        } 
    }
    Ok(())
}

fn main() {
    let file = read_lines("lichess_elite.pgn").unwrap();

    let mut game_lines = String::from("");
    for (idx, l) in file.enumerate() {
        let line = l.unwrap();
        if !line.is_empty() && !line.contains("[") {
            game_lines.push_str(&line.replace("\n", ""))
        } else if line.contains("[Event") {
            match handle_moves(&game_lines) {
                Err(e) => { 
                    println!("{}", game_lines);
                    println!("{}", e);
                    break;
                },
                _ => {}
            }
            game_lines = String::from("");
        }
    }
}
