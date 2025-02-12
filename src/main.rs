use std::io;
use std::io::Write;
use dotenv::dotenv;
use std::env;
use postgres::{Client, NoTls};

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
            None => summary()
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
    // Check dotenv and gather all PostgreSQL ENV variables.
    dotenv().ok();

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
    

    let _num_rows_inserted = pg_client().execute(
        "INSERT INTO summaries (summary) VALUES($1)",
        &[&summary.trim_end()],
    );

    println!("\x0a\x1b[36mSummary:\x1b[0m\x09\x09\x09{}", &summary);
    // println!("{} row(s) inserted.", num_rows_inserted);
}

fn list_summaries() {
    if let Ok(rows) = pg_client().query("SELECT * FROM summaries", &[]) {
        for row in rows {
            let text: &str = row.get(2);
            println!("{}", text);
        }
    }
}


//////////////////////////////////////////////////////////////////////////////
//
// Database functionality
//
fn pg_client() -> Client {
    let pg_host = match env::var("POSTGRES_HOST") {
        Ok(val) => val,
        Err(e) => panic!("could not find {}: {}", "POSTGRES_HOST", e),
    };
    let pg_port = match env::var("POSTGRES_PORT") {
        Ok(val) => val,
        Err(e) => panic!("could not find {}: {}", "POSTGRES_PORT", e),
    };
    let pg_database = match env::var("POSTGRES_DATABASE") {
        Ok(val) => val,
        Err(e) => panic!("could not find {}: {}", "POSTGRES_DATABASE", e),
    };
    let pg_user = match env::var("POSTGRES_USER") {
        Ok(val) => val,
        Err(e) => panic!("could not find {}: {}", "POSTGRES_USER", e),
    };
    let pg_password = match env::var("POSTGRES_PASSWORD") {
        Ok(val) => val,
        Err(e) => panic!("could not find {}: {}", "POSTGRES_PASSWORD", e),
    };

    let params = format!(
        "host={} port={} dbname={} user={} password={}", 
        pg_host,
        pg_port,
        pg_database,
        pg_user,
        pg_password
    );

    let client = match Client::connect(&params, NoTls) {
        Ok(client) => client,
        Err(e) => panic!("Could not connect to Postgres. {}", e)
    };
    
    return client;
}