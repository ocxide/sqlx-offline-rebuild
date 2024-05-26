use std::path::PathBuf;

use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Target directory
    pub target: PathBuf,
}

