use anyhow::{Context, Result};
use chrono::Local;
use clap::Parser;
use screenshots::Screen;
use std::{fs, path::PathBuf, thread. time:Duration};

#{derive(Parser, Debug)}
struct Args {
    #[arg(long, default_value_t = 60)]
    interval_sec: u64,

    #[arg(long, default_value = "./captures")]
    output_dir: PathBuf
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("Hello, world!");
}
