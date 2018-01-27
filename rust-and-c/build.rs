extern crate gcc;

fn main() {
  gcc::Build::new()
    .file("src/c/hello.c")
    .compile("hello");
}
