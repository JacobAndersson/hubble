use clap::Parser;
use comfy_table::Table;
use hubble::analysis::OpeningResult;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    player: String,

    #[clap(long)]
    only_white: Option<bool>,

    #[clap(short, long)]
    game_id: Option<String>,
}

fn gen_report(count: &mut Vec<(String, OpeningResult)>) {
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
    println!("{table}");
}


#[tokio::main]
async fn main() {
    dotenv::from_filename("../.env").ok();
    let args = Args::parse();
    let conn = hubble_db::get_connection();
    /*
    match hubble::analysis::best_opening(&args.player, &conn, 1000, args.only_white).await {
        Ok(mut opening_count) => {
            gen_report(&mut opening_count);
        }
        Err(_) => {
            println!("COULD NOT FETCH REPORT");
        }
    }
    */

    match hubble::lichess::analyse_player(&conn, &args.player).await {
        Ok(games) => println!("{:?}", games),
        Err(_) => println!("FAILED"),
    }
}
