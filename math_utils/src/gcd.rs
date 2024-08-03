// src/gcd.rs

/// Calculates the greatest common divisor (GCD) of two numbers.
/// 
/// # Examples
/// 
/// ```
/// let result = math_utils::gcd::gcd(54, 24);
/// assert_eq!(result, 6);
/// ```
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(54, 24), 6);
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(101, 103), 1);
        assert_eq!(gcd(0, 5), 5);
    }
}