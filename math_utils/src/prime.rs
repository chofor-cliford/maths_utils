// src/prime.rs

/// Checks if a number is prime.
/// 
/// # Examples
/// 
/// ```
/// let result = math_utils::prime::is_prime(7);
/// assert!(result);
/// 
/// let result = math_utils::prime::is_prime(10);
/// assert!(!result);
/// ```
pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
    }
}