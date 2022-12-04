use std::{fs::File, io::ErrorKind};

fn main() {
    println!("Hello, world!");

    let greeting_file_result = File::open("hello.txt");

    let greeting_file: File = match greeting_file_result {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(created_file) => created_file,
                Err(e) => panic!("Problem creating the file: {}", e)
            }
            other_err => panic!("Problem opening the file: {}", other_err)
        }
    };
}
