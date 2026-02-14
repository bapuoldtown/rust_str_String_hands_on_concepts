mod parse_result_some_variations;
fn main() {
    println!("Hello, world!");
    let vector = vec![1,2,3,4,5];
    let index=2;
    match parse_result_some_variations::get_vector_option(&vector, &index){
        Some(value) => println!("The value at the index is: {}", value),
        None => println!("The index is out of bounds"),
    }

    let vector2 = vec![10,20,30,40,50];
    let index2=3;
    //compare direct some in a if statement
    if let Some(value) = parse_result_some_variations::get_vector_option(&vector2, &index2) {
        println!("The value at the index is: {}", value);
    } else {
        println!("The index is out of bounds");
    }

    if let None = parse_result_some_variations::is_index_valid(&vector2, &index2) {
        println!("The index is out of bounds");
     } else {
        println!("The value at the index is: {}", parse_result_some_variations::get_vector_option(&vector2, &index2).unwrap());
     }
    let source_vec: Vec<&str> = vec!["hello", "world", "1", "2", "3"];
    //loop and show in simple code strtuctre
    for item in source_vec.iter(){
        match parse_result_some_variations::is_number(item){
            Ok(true) => println!("{} is a valid number", item),
            Ok(false) => println!("{} is not a valid number", item),
            Err(e) => println!("{}", e),
        }
    }
    //Now use chaining with a iter() -> filter()  -> count()
    let count_valid_count = source_vec.iter().filter(|it| parse_result_some_variations::is_number_ok(it).is_ok()).count();
    println!("The number of valid numbers is: {}", count_valid_count);
    let count_invalid_numbers = source_vec.iter().filter(|x| parse_result_some_variations::is_number_not_ok_error(x).is_err()).count();

    println!("The number of invalid numbers is: {}", count_invalid_numbers);
    let source_vec3: Vec<&str> = vec!["hello","1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12"];

    println!("Refined vector of i32 numbers: {:?}", parse_result_some_variations::refine_all_i32_numbers(&source_vec3)); 
}
