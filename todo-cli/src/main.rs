use std::collections::HashMap;

fn main() {
  let action = std::env::args().nth(1).expect("Please specify an action");
  let item = std::env::args().nth(2).expect("Please specify an item");
  let mut todo = Todo {
    map: HashMap::new(),
  };

  if action == "add" {
    todo.insert(item);
    match todo.save() {
      Ok(_) => println!("todo saved"),
      Err(why) => println!("An error occurred: {}", why),
    }
  }
}

struct Todo {
  map: HashMap<String, bool>,
}

impl Todo {
  fn new() -> Result<Todo, std::io::Error> {
    // 打开 db.json
    let f = std::fs::OpenOptions::new()
      .write(true)
      .create(true)
      .read(true)
      .open("db.json")?;
    // 序列化 json 为 HashMap
    match serde_json::from_reader(f) {
      Ok(map) => Ok(Todo { map }),
      Err(e) if e.is_eof() => Ok(Todo {
        map: HashMap::new(),
      }),
      Err(e) => panic!("An error occurred: {}", e),
    }
  }

  // borrow, change
  fn insert(&mut self, key: String) {
    self.map.insert(key, true);
  }

  // 在 Todo impl 代码块中
  fn save(self) -> Result<(), Box<dyn std::error::Error>> {
    // 打开 db.json
    let f = std::fs::OpenOptions::new()
      .write(true)
      .create(true)
      .open("db.json")?;
    // 通过 Serde 写入文件
    serde_json::to_writer_pretty(f, &self.map)?;
    Ok(())
  }

  fn complete(&mut self, key: &String) -> Option<()> {
    match self.map.get_mut(key) {
      Some(v) => Some(*v = false),
      None => None,
    }
  }
}
