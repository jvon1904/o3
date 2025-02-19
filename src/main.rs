mod commands;
mod database;
use std::env;

fn main() {
    // Declare variable for ARGV
    let args: Vec<String> = env::args().collect();

    // Check if args exist. If not, run summary procedure.
    if args.len() == 1 {
        commands::summary();
    // If arguments were supplied, handle the arguments.
    } else {
        match args.last() {
            Some(_arg) => handle_args(args[1..].to_vec()),
            None => commands::summary(),
        }
    }
}

fn handle_args(args: Vec<String>) {
    // Match the first argument and perform different functions.
    if args[0] == "ls" {
        commands::list();
    }
}
