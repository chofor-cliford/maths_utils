// src/main.rs

use math_utils::{factorial, gcd, prime};

fn main() {
    let fact_result = factorial::factorial(5);
    println!("Factorial of 5 is: {}", fact_result);

    let gcd_result = gcd::gcd(54, 24);
    println!("GCD of 54 and 24 is: {}", gcd_result);

    let prime_result = prime::is_prime(7);
    println!("Is 7 prime? {}", prime_result);
    
    let not_prime_result = prime::is_prime(10);
    println!("Is 10 prime? {}", not_prime_result);
}
