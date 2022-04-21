use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
  x: i32,
  y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
  name: String,
  age: i32,
}

fn main() {
  let point = Point { x: 1, y: 2 };

  // Convert the Point to a JSON string.
  let serialized = serde_json::to_string(&point).unwrap();

  // Prints serialized = {"x":1,"y":2}
  println!("serialized = {}", serialized);

  // Convert the JSON string back to a Point.
  let deserialized: Point = serde_json::from_str(&serialized).unwrap();

  // Prints deserialized = Point { x: 1, y: 2 }
  println!("deserialized = {:?}", deserialized);


  // ------------------------------
  let json_person = r#"
  {
    "name": "ben",
    "age": 99,
    "address": {
      "address1": "abd 1234",
      "address2": "abd 1234"
    }
  }
  "#;

  let person: Person = serde_json::from_str(json_person).unwrap();
  println!("person is  {:?}", person);
}
