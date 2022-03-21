use super::models::Course;
use std::sync::Mutex;

pub struct AppState {
  pub health_check_response: String,
  pub visit_count: Mutex<u32>,
  // storage in memory
  pub courses: Mutex<Vec<Course>>,
}
