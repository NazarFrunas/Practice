fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let limit = (n as f64).sqrt() as u32;
    for i in 3..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}
fn main() {
    let numbers = [1, 2, 3, 4, 5, 16, 17, 18, 19, 20];

    for &num in &numbers {
        if is_prime(num) {
            println!("{} is prime", num);
        } else {
            println!("{} is not prime", num);
        }
    }
}
