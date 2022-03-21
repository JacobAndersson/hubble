use comfy_table::Table;
use hubble::analysis::OpeningResult;

pub fn opening_report(count: &mut Vec<(String, OpeningResult)>) -> Table{
    let mut count_win_rate = count
        .iter()
        .filter_map(|row| {
            let tot = (row.1.won + row.1.lost + row.1.tie) as f64;
            if tot < 5. {
                None
            } else {
                Some((row.0.clone(), row.1, row.1.won as f64 / tot))
            }
        })
        .collect::<Vec<(String, OpeningResult, f64)>>();

    count_win_rate.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    let mut table = Table::new();

    table.set_header(vec!["name", "won", "tie", "lost", "win rate"]);

    for (name, op, win_rate) in count_win_rate.iter().take(10) {
        table.add_row(vec![
            name,
            &op.won.to_string(),
            &op.tie.to_string(),
            &op.lost.to_string(),
            &win_rate.to_string(),
        ]);
    }

    table.add_row(vec!["...", "...", "...", "..."]);
    for (name, op, win_rate) in count_win_rate.iter().rev().take(10).rev() {
        table.add_row(vec![
            name,
            &op.won.to_string(),
            &op.tie.to_string(),
            &op.lost.to_string(),
            &win_rate.to_string(),
        ]);
    }

    table
}
