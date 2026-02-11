/*
**Why?** Because `String` automatically **derefs** to `&str`!
```
String  ─── Deref ───►  &str
        (automatic!)
        All mehhods on &tr and String are in sync bruv
 */

pub fn convert_to_owned(s: &str) -> String{
    return s.to_string();

}
pub fn convert_to_owned_using_method(s: &str) -> String{
    return s.to_owned();
}

pub fn string_len(s: &str) -> usize{
    return s.len();
}

pub fn string_is_empty(s: &str) -> bool{
    return s.is_empty();
}


pub fn string_contains(s: &str, substring: &str) -> bool{
    return s.contains(substring)
}

pub fn string_trim(s: &str) -> String{
    return s.trim().to_string();
}

pub fn string_chars_collect(s: &str) -> Vec<char>{
    return s.chars().collect::<Vec<char>>();
}

pub fn string_bytes_collect(s: &str) -> Vec<u8>{
    return s.bytes().collect::<Vec<u8>>();
}

pub fn string_lines_collect(s: &str) -> Vec<&str>{
    return s.lines().collect::<Vec<&str>>();
}

pub fn split_string_pattern<'a>(s: &'a str, pattern: &str) -> Vec<&'a str>{
    return s.split(pattern).collect::<Vec<&'a str>>();
}

