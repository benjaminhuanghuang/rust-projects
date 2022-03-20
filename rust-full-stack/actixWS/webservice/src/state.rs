use std::sync::Mutex;

pub struct AppState {
  pub health_check_handler: String,
  pub visit_count: Mutex<u32>,
}