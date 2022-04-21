use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use serde::{Deserialize, Serialize};
use std::time::Duration;

mod credentials;

use credentials::*;

pub type Db = Pool<Postgres>;


#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
	pub id: i64,
	pub cid: i64, // creator id
	pub title: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let db = new_db_pool(PG_HOST, PG_APP_DB, PG_APP_USER, PG_APP_PWD, 1).await?;

  let result = sqlx::query("SELECT * from todo").fetch_all(&db).await?;

  assert_eq!(2, result.len());


  let sql = "SELECT id, cid, title FROM todo ORDER BY id DESC LIMIT 1";
  let query = sqlx::query_as::<_, Todo>(&sql);
  let todos = query.fetch_all(&db).await?;


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
