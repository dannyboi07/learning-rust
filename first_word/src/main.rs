fn main() {
    println!("Hello, world!");
    const STR_TO_PARSE: &str = "this is the string to parse";

    get_first_word(STR_TO_PARSE);
}

fn get_first_word(s: &str) -> String {
    let mut result: String = String::new();
    
    for c in s.as_bytes().iter() {
        result.push_str(&String::from_utf8_lossy(&[*c]));
    }

    return result;
}