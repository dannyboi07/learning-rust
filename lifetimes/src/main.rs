struct ImportantExcerpt<'a> {
    part: &'a str
}

fn main() {
    println!("Hello, world!");
    
    let str1: &str = "Daniel";
    
    let str2: &str = "Chettiar";
    println!("str1: {:p}, str2: {:p}", str1, str2);

    println!("{:p}", longer(str1, str2));

    let novel: &str = "Call me Daniel. Some years ago";
    let first_sentence = novel.split(".").next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence
    };
}

fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x > y {
        x
    } else {
        y
    }
}