use hubble_db::models::game::Game;

pub fn find_blunder(game: &Game) -> Vec<(usize, String)> {
    let mut prev_score = 0.;
    let mut is_white = true;

    let mut blunders = Vec::new();

    for ((idx, mv), sc) in game.moves.iter().enumerate().zip(game.scores.iter()) {
        let score = sc.parse::<f32>().unwrap();
        if prev_score - score > 300. {
            blunders.push((idx, mv.clone()));
        }

        prev_score = score;
        is_white = !is_white;
    }

    blunders
}
