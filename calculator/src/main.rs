use std::env::{args, Args};

fn main() {
  let mut args: Args = args();

  let first = args.nth(1).unwrap();
  // nth() will call next() internally
  let operator = args.nth(0).unwrap().chars().next().unwrap();
  let second = args.nth(0).unwrap();
  println!("{} {} {}", first, operator, second);

  let first_num: f32 = first.parse::<f32>().unwrap();
  let second_num: f32 = second.parse().unwrap();
  let result = operate(operator, first_num, second_num);
  println!("{:?}", output(first_num, operator, second_num, result));
}

fn operate(operator: char, first_num: f32, second_num: f32) -> f32 {
  match operator {
    '+' => first_num + second_num,
    '-' => first_num - second_num,
    '*' | 'x' | 'X' => first_num * second_num,
    '/' => first_num / second_num,
    _ => panic!("Invalid operator used.")
  }
}

fn output(first_num: f32, operator: char, second_num: f32, result: f32) -> String {
  format!("{} {} {} = {}", first_num, operator, second_num, result)
}
