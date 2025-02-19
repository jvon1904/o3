use chrono::offset::Local;
use chrono::DateTime;
use crate::database;
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

pub fn list() {
    if let Ok(rows) =
        database::client().query("SELECT * FROM summaries ORDER BY created_at DESC", &[])
    {
        for row in rows {
            let text: &str = row.get(2);
            let ts: SystemTime = row.get(1);
            let date: DateTime<Local> = ts.into();
            println!("{}\t{}", date.format("%a %l:%M%P"), text);
        }
    }
}
