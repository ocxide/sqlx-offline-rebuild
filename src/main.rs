mod cli;
mod common;
mod from;
mod to;

use std::{
    collections::HashMap,
    fs::{read_dir, read_to_string, File},
    io::Write,
};

use clap::Parser;
use from::FromOld;
use to::SqlxData;

use crate::cli::Args;

fn main() {
    let Args { target } = cli::Args::parse();

    let sqlx_dir = target.join("target/sqlx");
    let sqlx_file = target.join("sqlx-data.json");

    let mut sqlx_data = SqlxData {
        db: "PostgreSQL".to_owned(),
        entries: HashMap::new(),
    };

    for entry in read_dir(&sqlx_dir).expect("failed to read sqlx dir") {
        let entry = entry.expect("failed to read entry");
        if entry.file_type().expect("failed to get file type").is_dir() {
            let dir = sqlx_dir.join(entry.file_name());

            for subentry in read_dir(&dir).expect("failed to read subsqlx dir") {
                let subentry = subentry.expect("failed to read subentry");
                let filename = dir.join(subentry.file_name());

                let content = match read_to_string(&filename) {
                    Ok(content) => content,
                    Err(e) => {
                        panic!(
                            "failed to read {} with err {}",
                            filename.to_string_lossy(),
                            e
                        );
                    }
                };

                let old_query = match serde_json::from_str::<FromOld>(&content) {
                    Ok(old_query) => old_query,
                    Err(e) => {
                        panic!(
                            "failed to parse {} with err {}",
                            filename.to_string_lossy(),
                            e
                        );
                    }
                };

                sqlx_data.entries.insert(
                    old_query.hash,
                    to::ToEntry {
                        describe: old_query.describe,
                        query: old_query.query,
                    },
                );
            }
        }
    }

    dbg!(&sqlx_file);
    let mut file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(sqlx_file)
        .expect("failed to open sqlx-data.json file");

    let sqlx_data =
        serde_json::to_string_pretty(&sqlx_data).expect("failed to serialize sqlx-data.json");

    file.write_all(sqlx_data.as_bytes())
        .expect("failed to write sqlx-data.json file");
}
