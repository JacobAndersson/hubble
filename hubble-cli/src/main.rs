mod reports;

use reports::{opening_report, blunder_report};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    player: String,

    #[clap(long)]
    only_white: Option<bool>,

    #[clap(long)]
    opening_report: bool,

    #[clap(long, default_value_t = 10)]
    num_games: usize
}



#[tokio::main]
async fn main() {
    dotenv::from_filename("../.env").ok();
    let args = Args::parse();
    let pool = hubble_db::establish_connection();
    let conn = pool.get().unwrap();

    if args.opening_report {
        match hubble::analysis::best_opening(&args.player, &conn, 1000, args.only_white).await {
            Ok(mut opening_count) => {
                let report = opening_report(&mut opening_count);
                println!("{report}");
            }
            Err(_) => {
                println!("COULD NOT FETCH REPORT");
            }
        }
    } else {
        match hubble::lichess::analyse_player(conn, &args.player, args.num_games).await {
            Ok(games) => {
                let report = blunder_report(games);
                println!("{report}");
            },
            Err(e) => println!("{:?}", e),
        }
    }
}
