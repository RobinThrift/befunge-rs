
mod loader;
mod parser;

#[cfg(not(test))]
fn main() {
    let code = loader::load_file("./src/main.rs");
    println!("{}", code);
}
