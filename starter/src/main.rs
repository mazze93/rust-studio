use signal_notebook::{mean, parse_scores};
use std::{env, error::Error, fs, process::ExitCode};

fn run() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os().skip(1);
    let path = args.next().ok_or("usage: signal-notebook <scores.txt>")?;
    if args.next().is_some() {
        return Err("expected exactly one file path".into());
    }
    let input = fs::read_to_string(path)?;
    let scores = parse_scores(&input)?;
    match mean(&scores) {
        Some(value) => println!("{} scores · mean {value:.2}", scores.len()),
        None => println!("No scores yet."),
    }
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
