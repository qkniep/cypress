use std::io::{stdin, BufRead};

use clap::Parser;
use color_eyre::Result;
use indicatif::ProgressStyle;

/// Simple program to turn any output into a progress bar
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// ...
    #[arg(long, default_value_t = false)]
    lines: bool,

    /// ...
    #[arg(long, default_value_t = 0)]
    min: u64,

    /// ...
    #[arg(long)]
    max: u64,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.min > args.max {
        eprintln!("min must be less than max");
        std::process::exit(1);
    }

    let style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] {wide_bar:.blue} {pos}/{len} ({eta})")?;

    let progress = indicatif::ProgressBar::new(args.max - args.min + 1);
    progress.set_style(style);

    let stdin = stdin();
    for line in stdin.lines() {
        let line = line?;

        if args.lines {
            progress.inc(1);
            continue;
        }

        let res = line.parse();
        if res.is_err() {
            continue;
        }
        let number: u64 = res.unwrap();
        if number < args.min || number > args.max {
            continue;
        }
        progress.set_position(number - args.min);
    }

    progress.finish();

    Ok(())
}
