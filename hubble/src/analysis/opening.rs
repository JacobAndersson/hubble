use shakmaty::{san::San, Chess, Position};
use pgn_reader::BufferedReader;
use hubble_db::models::Opening;
use crate::lichess::get_games_player;
use hubble_db::PgConnection;
use crate::analysis::{OpeningCounter, OpeningResult};
use anyhow::Result;

pub fn match_length(opening: &Opening, moves: &Vec<String>) -> usize {
    //moves - vector with the moves in uci format
    let cleaned = opening.pgn.replace(".", "");
    let splits = cleaned.split(" ");
    let mut length = 0;
    let mut idx = 0;

    let mut board = Chess::default();

    for mv in splits {
        if mv.len() == 1 {
            continue;
        }
        if let Ok(san) = mv.parse::<San>() {
            if let Ok(parsed_move) = san.to_move(&board) {
                board.play_unchecked(&parsed_move);
                println!("{} {}", moves[idx], parsed_move.to_string());
                if moves[idx] == parsed_move.to_string().replace("-", "") {
                    length += 1;
                    idx += 1;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    length
}

pub fn match_length_sans(opening: &Opening, moves: &Vec<String> ) -> usize {
    //For when both opening and moves is in san format
    let cleaned = opening.pgn.replace(".", "");
    let splits = cleaned.split(" ");
    let mut length = 0;
    let mut idx = 0;

    for mv in splits {
        if idx >= moves.len() {
            break;
        }

        if mv.parse::<u32>().is_ok() {
            continue;
        }

        if moves[idx] == mv {
            length += 1;
        } else  {
            break;
        }
        idx += 1;
    }
    length
}

pub async fn best_opening(player_id: &str, conn: &PgConnection, num: usize, white: Option<bool>) -> Result<Vec<(String, OpeningResult)>>{
    //white - true if only to analyse games where white, false - black. None - both 
    let pgn = get_games_player(player_id, num).await?;
    let mut counter = OpeningCounter::new(conn, player_id.to_string(), white);
    let mut reader = BufferedReader::new_cursor(&pgn[..]);

    while let Some(_ok) = reader.read_game(&mut counter).unwrap() {
    }

    Ok(counter.openings.into_iter().collect::<Vec<(String,OpeningResult)>>())
}
