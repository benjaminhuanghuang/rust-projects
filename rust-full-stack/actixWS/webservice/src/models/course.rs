use crate::errors::MyError;
use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

// db -> json
// can use query_as! with sqlx::FromRow
#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course {
  pub teacher_id: i32,
  pub id: i32,
  pub name: String,
  pub time: Option<NaiveDateTime>,

  pub descripton: Option<String>,
  pub format: Option<String>,
  pub structure: Option<String>,
  pub duration: Option<String>,
  pub price: Option<String>,
  pub language: Option<String>,
  pub level: Option<String>,
}
//  json -> db
#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourse {
  pub teacher_id: i32,
  pub name: String,
  pub descripton: Option<String>,
  pub format: Option<String>,
  pub structure: Option<String>,
  pub duration: Option<String>,
  pub price: Option<String>,
  pub language: Option<String>,
  pub level: Option<String>,
}

impl TryFrom<web::Json<CreateCourse>> for CreateCourse {
  type Error = MyError;

  fn try_from(course: web::Json<CreateCourse>) -> Result<Self, Self::Error> {
    Ok(CreateCourse {
      teacher_id: course.teacher_id,
      name: course.name.clone(),
      descripton: course.descripton.clone(),
      format: course.format.clone(),
      structure: course.structure.clone(),
      duration: course.duration.clone(),
      price: course.price.clone(),
      language: course.language.clone(),
      level: course.level.clone(),
    })
  }
}

//  json -> db
#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCourse {
  pub name: Option<String>,
  pub description: Option<String>,
  pub format: Option<String>,
  pub structure: Option<String>,
  pub duration: Option<String>,
  pub price: Option<String>,
  pub language: Option<String>,
  pub level: Option<String>,
}

impl From<web::Json<UpdateCourse>> for UpdateCourse {
  fn from(course: web::Json<UpdateCourse>) -> Self {
    UpdateCourse {
      name: course.name.clone(),
      description: course.description.clone(),
      format: course.format.clone(),
      structure: course.structure.clone(),
      duration: course.duration.clone(),
      price: course.price.clone(),
      language: course.language.clone(),
      level: course.level.clone(),
    }
  }
}
