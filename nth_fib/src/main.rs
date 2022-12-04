use std::io;

fn main() {
    println!("Hello, world!");

    let nth: i64 = read_nth();
    let answer: i64 = fib(nth);

    println!("The answer is {answer}");
}

fn read_nth() -> i64 {
    println!("Enter the nth number...");

    loop {
        let mut nth: String = String::new();
        match io::stdin().read_line(&mut nth) {
            Ok(_) => {}
            Err(err) => {
                println!("Failed to read your input, try again..., err: {err}");
                continue;
            }
        }

        let _: i64 = match nth.trim().parse() {
            Err(err) => {
                println!("Failed to parse your input, try again..., err: {err}");
                continue;
            }
            Ok(num) => return num,
        };
    }
}

fn fib(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }

    return fib(n - 2) + fib(n - 1);
}
