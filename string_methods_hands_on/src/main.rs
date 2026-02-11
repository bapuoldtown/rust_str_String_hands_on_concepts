pub mod string_methods_demo;
fn main() {
    println!("Hello, world!");
    println!("The string is with to_str method: {}", string_methods_demo::convert_to_owned("Hello, Rust!"));
    println!("The string is with to owned: {}", string_methods_demo::convert_to_owned_using_method("Hello, Rust!"));
    println!("The length of the string is: {}", string_methods_demo::string_len("Hello, Rust!"));
    println!("Is the string empty? {}", string_methods_demo::string_is_empty("Hello, Rust!"));
    println!("Does the string contain 'Rust'? {}", string_methods_demo::string_contains("Hello, Rust!", "Rust"));
    println!("Trimmed string: '{}'", string_methods_demo::string_trim("   Hello, Rust!   "));
    println!("Characters in the string: {:?}", string_methods_demo::string_bytes_collect("Hello, Rust!"));
    println!("Lines in the string: {:?}", string_methods_demo::string_lines_collect("Hello, 
    Rust!
    Hello 
    again"));
    println!("Split string by pattern: {:?}", string_methods_demo::split_string_pattern("Hello, Rust! Hello again!", " "));
}
