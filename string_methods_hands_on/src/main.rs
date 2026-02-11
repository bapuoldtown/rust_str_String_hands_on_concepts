pub mod string_methods_demo;
fn main() {
    println!("Hello, world!");
    println!("The string is with to_str method: {}", string_methods_demo::convert_to_owned("Hello, Rust!"));
    println!("The string is with to owned: {}", string_methods_demo::convert_to_owned_using_method("Hello, Rust!"));
}
