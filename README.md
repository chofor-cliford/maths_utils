# Math Utils Library 📐✨

Welcome to `math_utils`! This Rust library provides essential mathematical utility functions to make your development easier. Whether you need to calculate factorials, find the greatest common divisor (GCD), or check if a number is prime, `math_utils` has got you covered! 🚀

## Features 🌟

- **Factorial Calculation**: Compute the factorial of a number with ease. 
- **Greatest Common Divisor**: Determine the GCD of two numbers.
- **Prime Check**: Quickly check if a number is prime.

## Installation 📦

Add `math_utils` to your `Cargo.toml`:

```toml
[dependencies]
math_utils = "0.1.0"
```
## Usage 🛠️
Here's a quick example to get you started:

```
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
```

## Documentation 📚
For detailed documentation, run:
```
cargo doc --open
```
## Contributing 🤝
Contributions are welcome! If you have ideas for new features or improvements, feel free to open an issue or submit a pull request.

## License 📜
This project is licensed under the MIT License. See the LICENSE file for more details.

Enjoy using math_utils! If you like it, give it a ⭐ on GitHub.
