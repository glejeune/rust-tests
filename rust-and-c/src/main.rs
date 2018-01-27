extern {
  fn add(a: u32, b: u32) -> u32;
}

fn main() {
  unsafe {
    println!("7 + 4 = {}", add(7, 4));
  }
}
