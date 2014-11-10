
mod loader;

#[cfg(not(test))]
fn main() {
    loader::load_file("./src/main.rs");
}
