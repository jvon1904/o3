use crate::database;
use chrono::offset::Local;
use chrono::DateTime;
use std::io;
use std::io::Write;
use std::time::SystemTime;

pub fn summary() {
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

    let _num_rows_inserted = database::client().execute(
        "INSERT INTO summaries (summary) VALUES($1)",
        &[&summary.trim_end()],
    );

    println!("\x0a\x1b[36mSummary:\x1b[0m\x09\x09\x09{}", &summary);
    // println!("{} row(s) inserted.", num_rows_inserted);
}

pub fn list(args: Vec<String>) {
    let mut query = String::from("");

    if args.len() == 0 {
        query = String::from(
            "SELECT * FROM summaries \
                            ORDER BY created_at DESC",
        );
    } else {
        // Must be a number between 0 and 65535, else print an error.
        match args[0].parse::<u16>() {
            Ok(_) => {
                query = format!(
                    "SELECT * FROM summaries \
                                        WHERE created_at >= current_date - INTERVAL '{} days' \
                                        ORDER BY created_at DESC",
                    args[0]
                )
            }
            Err(_) => println!("{} is not a valid number of days.", args[0]),
        }
    }

    if let Ok(rows) = database::client().query(&query, &[]) {
        for row in rows {
            let text: &str = row.get(2);
            let ts: SystemTime = row.get(1);
            let date: DateTime<Local> = ts.into();
            println!("{}\t{}", date.format("%a %l:%M%P"), text);
        }
    }
}
