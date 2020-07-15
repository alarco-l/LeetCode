pub fn is_palindrome(x: i32) -> bool {
    if x == 0 {
        return true
    }
    else if x < 0 || (x % 10 == 0) {
        return false
    }

    let mut x = x;
    let mut revert = 0;
    while x > revert {
        revert *= 10;
        revert += x % 10;
        x /= 10;
    }
    x == revert || x == revert / 10
}

#[cfg(test)]
mod tests {
    #[test]
    fn palindrome_odd_number() {
        let result = super::is_palindrome(121);

        assert_eq!(result, true);
    }

    #[test]
    fn palindrome() {
        let result = super::is_palindrome(1221);

        assert_eq!(result, true);
    }

    #[test]
    fn palindrome_with_zero() {
        let result = super::is_palindrome(0);

        assert_eq!(result, true);
    }

    #[test]
    fn negative_number() {
        let result = super::is_palindrome(-121);

        assert_eq!(result, false);
    }

    #[test]
    fn not_palindrome() {
        let result = super::is_palindrome(2230);

        assert_eq!(result, false);
    }
}
