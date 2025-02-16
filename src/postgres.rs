use dotenv::dotenv;
use std::env;
use postgres::{Client, NoTls};

pub fn client() -> Client {
  // Check dotenv and gather all PostgreSQL ENV variables.
  dotenv().ok();

  let pg_host = match env::var("POSTGRES_HOST") {
      Ok(val) => val,
      Err(_) => String::from("localhost")
  };
  let pg_port = match env::var("POSTGRES_PORT") {
      Ok(val) => val,
      Err(_) => String::from("5432"),
  };
  let pg_database = match env::var("POSTGRES_DATABASE") {
      Ok(val) => val,
      Err(_) => String::from("o3_database"),
  };
  let pg_user = match env::var("POSTGRES_USER") {
      Ok(val) => val,
      Err(_) => String::from("postgres"),
  };
  let pg_password = match env::var("POSTGRES_PASSWORD") {
      Ok(val) => val,
      Err(_) => String::from("postgres"),
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