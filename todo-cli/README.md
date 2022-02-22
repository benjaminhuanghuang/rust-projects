## Rust 语言入门教程：从实战 To-Do App 开始
https://www.freecodecamp.org/news/how-to-build-a-to-do-app-with-rust/
https://chinese.freecodecamp.org/news/how-to-build-a-to-do-app-with-rust/


## Key 
Rust 中的错误处理。

Options 和 Null 类型。

Structs 和 impl。

终端 I/O。

文件系统处理。

Rust 中的所有权（Ownership）和借用（borrow）。

匹配模式。

迭代器和闭包。

使用外部的包（crates）。


## Setup
```
  cargo new todo-cli


  cargo run -- add "code rust"
```


## Borrow
```
fn main() {
  // String 的所有者是 x
  let x = String::from("Hello");

  // 我们将值移动到此函数中
  // 现在 doSomething 是 x 的所有者
  // 一旦超出 doSomething 的范围
  // Rust 将释放与 x 关联的内存。
  doSomething(x);

  // 由于我们尝试使用值 x，因此编译器将引发错误
  // 因为我们已将其移至 doSomething 内
  // 我们此时无法使用它，因为此时已经没有所有权
  // 并且该值可能已经被删除了
  println!("{}", x);
}
```