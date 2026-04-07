use anyhow::{Context,Result};
use chrono::Local;
use clap::Parser;
use screenshots::Screen;
use std::{fs, path::PathBuf, thread, time::Duration};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value_t = 60)]
    interval_sec: u64,

    #[arg(long, default_value = "./temp/captures")]
    output_dir: PathBuf
}

fn main() -> Result<()> {
    let args = Args::parse();
    fs::create_dir_all(&args.output_dir).context("failed to create output directory")?;

    let screens = Screen::all().context("failed to enumare screens");
    let primary = screens?.first().context("no screens found")?.to_owned();

    loop {
        let now = Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!("capture_{}.png", now);
        let path = args.output_dir.join(filename);
        match primary.capture() {
            Ok(image) => {
                if let Err(e) = image.save(&path) {
                    eprintln!("save error: {e}");
                } else {
                    println!("saved: {}", path.display());
                }
            }
            Err(e) => eprintln!("capture error: {e}"),
        }

        thread::sleep(Duration::from_secs(args.interval_sec));

    }
}
