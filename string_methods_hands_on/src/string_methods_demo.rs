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

pub fn find_substring(main: &str, s:&str)->Option<usize>{
    return main.find(s);

}
//rfind() - same but searches from RIGHT
//fn rfind<P>(&self, pat: P) -> Option<usize>
pub fn rfind_demo(main: &str, pattern: &str) -> Option<usize>{
    return main.rfind(pattern);
}

pub fn starts_with_demo(main: &str, pattern: &str) -> bool{
    return main.starts_with(pattern);
}

pub fn ends_with_demo(main: &str, pattern: &str) ->bool{
    return main.ends_with(pattern)
}
//fn trim(&self) -> &str
//      ▲         ▲
//      │         └── Returns: borrowed slice (NO allocation!)
//      └── Takes: &self
pub fn trim_start_demo(main: &str) -> &str{
    return main.trim_start();
}

pub fn trim_end_demo(main: &str) -> &str{
    return main.trim_end();
}

pub fn trim_matches_demo<'a>(main: &'a str, pattern: &char) ->&'a str{
    return main.trim_matches(*pattern)
}

//Some transform string functions
//to_uppercase()    │ &self          │ String
//to_lowercase()    │ &self          │ String
//replace(from,to)  │ &self,P,&str   │ String
//replacen(f,t,n)   │ &self,P,&str,n │ String
//repeat(n)         │ &self, usize   │ String
pub fn to_uppercase_lowercase_demo(main: &str) -> Option<(String, String)>{
    return Some((main.to_uppercase(), main.to_lowercase()));
}

//fn replace<P>(&self, from: P, to: &str) -> String
//        ▲    ▲        ▲        ▲          ▲
//        │    │        │        │          └── Returns: NEW String
//        │    │        │        └── Replacement string
//        │    │        └── Pattern to find
//        │    └── Takes: &self
//        └── Generic over pattern

pub fn replace_demo(main: &str, repl_patter: &str, what_pattern: &str)-> String{
    return  main.replace(repl_patter, what_pattern);

}
//fn replacen<P>(&self, from: P, to: &str, count: usize) -> String
pub fn replace_demo_count(main: &str, repl_pattern: &str, what_pattern: &str, count: usize) -> String{
    return main.replacen(repl_pattern, what_pattern, count);

}