use std::io::{stdin, BufRead};

use clap::Parser;
use color_eyre::Result;
use indicatif::ProgressStyle;
use regex::Regex;

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

    /// Regex applied to the input stream.
    /// In `lines` mode: All lines not matching regex are ignored.
    /// In `numbers` mode: If the regex matches,
    /// content of first capture group is treated as the integer input.
    #[arg(long)]
    regex: Option<String>,

    /// Prints out debug information.
    /// This is especially useful for constructing regexes.
    #[arg(long, default_value_t = false)]
    debug: bool,
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
        let mut line = line?;

        if args.lines {
            if let Some(re) = &args.regex {
                if Regex::new(re).unwrap().is_match(&line) {
                    progress.inc(1);
                } else {
                    if args.debug {
                        eprintln!("Did not match on line: {}", line);
                    }
                }
                continue;
            }

            progress.inc(1);
            continue;
        }

        if let Some(re) = &args.regex {
            if let Some(captures) = Regex::new(re).unwrap().captures(&line) {
                line = captures[1].to_string();
            } else {
                if args.debug {
                    eprintln!("Did not match on line: {}", line);
                }
            }
        }

        let res = line.parse();
        if res.is_err() {
            if args.debug {
                eprintln!("Failed to parse as int: {}", line);
            }
            continue;
        }
        let number: u64 = res.unwrap();
        if number < args.min || number > args.max {
            if args.debug {
                eprintln!("Integer out of min/max bounds: {}", number);
            }
            continue;
        }
        progress.set_position(number - args.min);
    }

    progress.finish();

    Ok(())
}
