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
    println!("Find substring 'Rust' in main string: {:?}", string_methods_demo::find_substring("Hello, Rust! Hello again!", "Rust"));  
    match string_methods_demo::find_substring("Hello Rust World","Rust"){
        Some(index) => println!("Substring found at index: {}", index),
        None => println!("Substring not found"),
    }
    //check if for match with Some else None
    if let Some(index) = string_methods_demo::rfind_demo("Hello World Guru", "Guru"){
        println!("Substring found at index: {}", index);
    } else {
        println!("Substring not found");    
    }
    println!("Does the string start with 'Hello'? {}", string_methods_demo::starts_with_demo("Hello, Rust!", "Hello")); 
    println!("Does the string end with 'Rust!'? {}", string_methods_demo::ends_with_demo("Hello, Rust!", "Rust!"));
    println!("Trimmed start: '{}'", string_methods_demo::trim_start_demo("   Hello, Rust!   "));
    println!("Trimmed end: '{}'", string_methods_demo::trim_end_demo("   Hello, Rust!   "));
    println!("Trimmed matches: '{}'", string_methods_demo::trim_matches_demo("   Hello, Rust!   ", &' '));

    if let Some((a,b)) = string_methods_demo::to_uppercase_lowercase_demo("Guru junu krisha"){
        println!("Uppercase: {}, Lowercase: {}", a, b);
    }
    else{
        println!("Error converting to uppercase or lowercase");
    }

    println!("Replaced string: {}", string_methods_demo::replace_demo("Hello, Rust!", "Rust", "World"));
    println!("The replace demo count method is {}", string_methods_demo::replace_demo_count("Hello, Rust! Hello again!", "Hello", "Hi", 1));
}
