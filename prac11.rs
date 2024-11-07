fn is_palindrome(n: u32) -> bool {
    let original = n.to_string();
    let reversed: String = original.chars().rev().collect();
    original == reversed
}
fn main() {
    let numbers = [121, 123, 202, 12321, 1001, 555];

    for &num in &numbers {
        if is_palindrome(num) {
            println!("{} is a palindrome", num);
        } else {
            println!("{} is not a palindrome", num);
        }
    }
}
