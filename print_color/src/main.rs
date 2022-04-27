use std::fmt::{self, Display, Formatter, UpperHex};
// 在std中fmt::LowerHex和fmt::UpperHex可以用来输出16进制，分别对应的格式化符号是{:x}、{:X}。

struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

impl Display for Color {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "RGB ({}, {}, {})", self.red, self.green, self.blue)
  }
}

impl UpperHex for Color {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
  }
}

fn main() {
  for color in [
    Color {
      red: 128,
      green: 255,
      blue: 90,
    },
    Color {
      red: 0,
      green: 3,
      blue: 254,
    },
    Color {
      red: 0,
      green: 0,
      blue: 0,
    },
  ]
  .iter()
  {
    println!("{} 0x{:X}", *color, color)
  }
}
