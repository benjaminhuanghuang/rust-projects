use chrono::NaiveDateTime;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::io;

#[derive(Debug)]
pub struct Course {
  pub id: i32,
  pub teacher_id: i32,
  pub name: String,
  pub time: Option<NaiveDateTime>,
}


#[active_rt::main]
aysnc fn main()->io::Result<()> {
  dotenv().ok();  // cover the error if .env not eixist

  // read .env config file
  let database_url = env::var("DATABASE_URL").expect("can not find DATABASE_URL in .env");

  // connect to db
  let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

  let course_rows = sqlx::query!(
    r#"select id, teachter_id, name, time, form course where id= $1"#, 1
  ).fetch_all(&db_pool).await.unwrap();


  let mut courses_list = vec![];

  for row in course_rows {
    courses_list.push(Course {
      id: row.id,
      teacher_id: row.teacher_id,
      name: row.name,
      time: Some(chrono::NaiveDateTime::from(row.time.unwrap())),
    })
  }

  println!("Courses = {:?}", course_list);

  Ok(())
}
