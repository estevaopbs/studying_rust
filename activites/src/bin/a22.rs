// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn clamp_lower() {
        assert_eq!(clamp(0, 1, 5), 1, "0 should clamp to 1");
    }

    #[test]
    fn clamp_upper() {
        assert_eq!(clamp(10, 1, 5), 5, "10 should clamp to 5");
    }

    #[test]
    fn clamp_within() {
        assert_eq!(clamp(3, 1, 5), 3, "3 should clamp to 3");
    }

    #[test]
    fn div_valid() {
        assert_eq!(div(10, 2), Some(5), "10 / 2 should be 5");
    }

    #[test]
    fn div_1_2() {
        assert_eq!(div(-10, 2), Some(-5), "-10 / 2 should be -5");
    }

    #[test]
    fn concat_valid() {
        assert_eq!(concat("hello", "world"), "hello world", "hello world");
    }

    #[test]
    fn concat_empty() {
        assert_eq!(concat("", ""), " ", "empty strings should be a space");
    }
}
