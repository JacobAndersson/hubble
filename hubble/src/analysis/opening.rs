use shakmaty::{san::San, Chess, Position};
use hubble_db::models::Opening;

pub fn match_length(opening: &Opening, moves: Vec<String>) -> usize {
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
