use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use hubble_db::models::{Opening, insert_openings};
use hubble_db::get_connection;
use diesel::pg::PgConnection;

fn format_row(row: &str) -> Vec<String> {
    row.split("\t").map(str::to_string).collect()
}

fn open_file(filename: &str) -> Vec<Vec<String>> {
    let mut contents = String::new();
    let file = File::open(filename).unwrap();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut contents).unwrap();

    contents
        .split("\n")
        .map(format_row)
        .collect::<Vec<Vec<String>>>()
}

fn insert_file(filename: &str, conn: &PgConnection, id: &mut i32) {
    let data = open_file(filename);
    let mut openings = Vec::new();

    for row in data {
        if row.len() < 3 || row[0] == "eco" {
            continue;
        }

        let eco = &row[0];
        let name = &row[1];
        let moves = &row[2];

        openings.push(Opening::new(*id, eco.to_string(), name.to_string(), moves.to_string()));
        *id += 1;
    }

    insert_openings(&conn, openings).unwrap();
}

fn main() {
    dotenv::from_filename("../.env").ok();
    let conn = get_connection();
    let files = ["a", "b", "c", "d", "e"];
    let mut id = 0;

    for f in files {
        let filename = format!("data/{}.tsv", f);
        println!("filename {}", &filename);
        insert_file(&filename, &conn, &mut id);
    }
}
