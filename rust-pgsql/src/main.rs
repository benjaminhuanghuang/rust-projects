use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::time::Duration;

mod credentials;

use credentials::*;

pub type Db = Pool<Postgres>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let db = new_db_pool(PG_HOST, PG_APP_DB, PG_APP_USER, PG_APP_PWD, 1).await?;

  let result = sqlx::query("SELECT * from todo").fetch_all(&db).await?;

  assert_eq!(2, result.len());

  Ok(())
}

async fn new_db_pool(
  host: &str,
  db: &str,
  user: &str,
  pwd: &str,
  max_con: u32,
) -> Result<Db, sqlx::Error> {
  let con_string = format!("postgres://{}:{}@{}/{}", user, pwd, host, db);
  //println!("con string: {}", con_string);

  PgPoolOptions::new()
    .max_connections(max_con)
    .connect_timeout(Duration::from_millis(5000))
    .connect(&con_string)
    .await
}
