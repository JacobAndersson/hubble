use comfy_table::Table;
use hubble_db::models::game::Game;

fn convert_to_string(input: Option<i32>) -> String {
    match input {
        Some(a) => format!("{}", a),
        None => String::from(" "),
    }
}

pub fn blunder_report(games: Vec<Game> ) -> Table {
    let mut table = Table::new(); 
    table.set_header(vec!["id", "middle game start", "end game start", "blunders opening", "blunders middle game", "blunders end game", "total number blunders"]);

    for game in games {
        let opening = game.blunders.opening.len();
        let middle_game = game.blunders.middle_game.len();
        let end_game = game.blunders.end_game.len();
        let tot = opening + middle_game + end_game;

        table.add_row(vec![
            game.id,
            convert_to_string(game.middle_game),
            convert_to_string(game.end_game),
            opening.to_string(),
            middle_game.to_string(),
            end_game.to_string(),
            tot.to_string()
        ]);
    }

    table
}
