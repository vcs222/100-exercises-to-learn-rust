// Rewrite the factorial function using a `while` loop.
pub fn factorial(n: u32) -> u32 {
    if n >= 1 {
        let mut result = n;
        let mut i = n - 1;
        while i > 1 {
            result *= i;
            i = i - 1;
        }
        return result;
    }

    1
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
