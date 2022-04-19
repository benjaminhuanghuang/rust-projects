use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::time::Duration;

mod credentials;

use credentials::*;

pub type Db = Pool<Postgres>;

#[tokio::main]
async fn main() {
  let root_db = new_db_pool(PG_HOST, PG_ROOT_DB, PG_ROOT_USER, PG_ROOT_PWD, 1).await;

  println!("{:?}", root_db);
}

async fn new_db_pool(
  host: &str,
  db: &str,
  user: &str,
  pwd: &str,
  max_con: u32,
) -> Result<Db, sqlx::Error> {
  let con_string = format!("postgres://{}:{}@{}/{}", user, pwd, host, db);
  println!("con string: {}", con_string);

  PgPoolOptions::new()
    .max_connections(max_con)
    .connect_timeout(Duration::from_millis(5000))
    .connect(&con_string)
    .await
}
