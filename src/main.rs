mod postgres;
use chrono::offset::Local;
use chrono::DateTime;
use std::env;
use std::io;
use std::io::Write;
use std::time::SystemTime;

fn main() {
    // Declare variable for ARGV
    let args: Vec<String> = env::args().collect();

    // Check if args exist. If not, run summary procedure.
    if args.len() == 1 {
        summary();
    // If arguments were supplied, handle the arguments.
    } else {
        match args.last() {
            Some(_arg) => handle_args(args[1..].to_vec()),
            None => summary(),
        }
    }
}

fn handle_args(args: Vec<String>) {
    // Match the first argument and perform different functions.
    if args[0] == "ls" {
        list_summaries();
    }
}

fn summary() {
    // Print greeting
    //
    // 0x1b (27) is ASCII escape (0b0011011)
    // 36 is cyan
    // 0x0a (10) is line feed
    println!("\x0a\x1b[1;36mWelcome to o3!\x1b[0m\x0a");

    // Print prompt
    print!("\x1b[36mWhat did you work on today?\x1b[0m\x09");
    io::stdout().flush().unwrap();

    // Declare variable for input
    let mut summary = String::new();

    // Get input
    io::stdin()
        .read_line(&mut summary)
        .expect("Failed to read input summary");

    let _num_rows_inserted = postgres::client().execute(
        "INSERT INTO summaries (summary) VALUES($1)",
        &[&summary.trim_end()],
    );

    println!("\x0a\x1b[36mSummary:\x1b[0m\x09\x09\x09{}", &summary);
    // println!("{} row(s) inserted.", num_rows_inserted);
}

fn list_summaries() {
    if let Ok(rows) =
        postgres::client().query("SELECT * FROM summaries ORDER BY created_at DESC", &[])
    {
        for row in rows {
            let text: &str = row.get(2);
            let ts: SystemTime = row.get(1);
            let date: DateTime<Local> = ts.into();
            println!("{}\t{}", date.format("%a %l:%M%P"), text);
        }
    }
}
