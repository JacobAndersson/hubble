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
    table.set_header(vec!["id", "middle game", "end game"]);

    for game in games {
        println!("{:?}", game);
        table.add_row(vec![
            game.id,
            convert_to_string(game.middle_game),
            convert_to_string(game.end_game)
        ]);
    }

    table
}
