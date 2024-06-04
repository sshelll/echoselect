use std::io::{self, Read};

use clap::Parser;
use dialoguer::Select;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    default: Option<String>,

    #[clap(name = "INPUT")]
    input: Vec<String>,
}

impl Args {
    fn is_available(&self) -> bool {
        !self.input.is_empty() || self.default.is_some()
    }
}

fn main() {
    if handle_by_call() {
        return;
    }
    handle_by_pipe();
}

fn handle_by_call() -> bool {
    let args = Args::parse();
    if !args.is_available() {
        return false;
    }
    select_input(args.input, args.default);
    true
}

fn handle_by_pipe() -> bool {
    let stdin = io::stdin();
    let mut buffer = String::new();
    if stdin.lock().read_to_string(&mut buffer).is_ok() {
        let input = buffer
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        select_input(input, None);
        return true;
    }
    false
}

fn select_input(input: Vec<String>, default: Option<String>) {
    if input.is_empty() {
        eprintln!("no input provided");
        std::process::exit(1);
    }
    let selection = Select::new()
        .default(0)
        .items(&input)
        .interact_opt()
        .unwrap_or_else(|e| {
            eprintln!("unexpected error: {}", e);
            std::process::exit(1);
        });
    match selection {
        Some(selection) => println!("{}", input[selection]),
        None if default.is_some() => println!("{}", default.unwrap()),
        _ => (),
    }
}
