use std::path::PathBuf;

use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, version)]
pub struct Args {
    /// Target directory where the .sqlx directory is located
    pub target: PathBuf,
}

