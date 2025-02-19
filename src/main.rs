mod commands;
mod database;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        handle_args(args[1..].to_vec())
    } else {
        commands::add();
    }
}

fn handle_args(args: Vec<String>) {
    match args[0].as_str() {
        "l" | "ls" | "list" => commands::list(args[1..].to_vec()),
        "a" | "add" => commands::add(),
        _ => println!("{} is not a valid command", args[0]),
    }
}
