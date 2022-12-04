fn main() {
    println!("Hello, world!");

    println!("{}", is_palindrome(10));
}

fn is_palindrome(x: i32) -> bool {
    let binding = x.to_string();
    let num = binding.as_bytes();
    if num.len() <= 1 {
        return true;
    }
    let (mut left, mut right): (usize, usize) = (0, num.len() - 1);
    while left <= right {

        if num[left] != num[right] {
            return false;
        }

        left += 1;
        right -= 1;
    }

    true
}