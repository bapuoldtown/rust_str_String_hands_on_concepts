//We will analyse all hands on with the variations of result amd Some
//EXERCISE SET 1 — Checking (is_some, is_none, is_ok, is_err)
//we will create a method which is going to thoe Option and handle that wity os_ok and is_none
//Takeaway: .get() returns Option<&T> — it never panics unlike direct indexing scores[10] which would crash. is_some/is_none are just quick bool checks without extracting the value.

pub fn get_vector_option(input_vec:&Vec<i32>, index:&i32) -> Option<i32> {
    //ptraverse a vector of splices and peek into a index and gives the Some
    if *index >= input_vec.len() as i32 || *index <0 {
        return None;
    }
    if input_vec.get(*index as usize).is_none(){
        return None;
    }
    return Some(*input_vec.get(*index as usize).unwrap());
  
}
//is_some_demo return type lets do bool :)
pub fn is_index_valid(input_vec: &Vec<i32>, index: &i32) -> Option<bool>{
    if *index >=input_vec.len() as i32 || *index <0 {
        return Some(false);
    }
    return Some(input_vec.get(*index as usize).is_some());
}

pub fn is_number(input_val: &str) -> Result<(bool), String>{
    match input_val.parse::<usize>(){
        Ok(_) => Ok(true),
        Err(_) => Err(format!("'{}' is not a valid number", input_val)),
    }

}

pub fn is_number_ok(input_val: &str) -> Result<(), String>{

    if input_val.parse::<usize>().is_ok(){
        return Ok(());
    }
    else{
        return Err(format!("'{}' is not a valid number", input_val));
    }
    

}

pub fn is_number_not_ok_error(input_val: &str) -> Result<(), String>{
    if input_val.parse::<usize>().is_err(){
        return Err(format!("'{}' is not a valid number", input_val));
    }
    else{
        return Ok(());
    }

}

pub fn refine_all_i32_numbers(input_vec: &Vec<&str>) -> Vec<i32>{
    let mut result_vector: Vec<i32> = Vec::new();
    //we will use clone and copied as vector is owned and returning in function
    for item in input_vec.iter(){
        if item.parse::<i32>().is_ok(){
            result_vector.push(item.parse::<i32>().unwrap());

        }
    }
    return result_vector;

}
