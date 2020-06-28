pub fn reverse(x: i32) -> i32 {
    let str = x.to_string();

    match x.to_string().chars().next() {
        Some(c) => {
            match c {
                '-' => {
                    let tmp = &str[1..str.len()];
                    match tmp.chars().rev().collect::<String>().parse::<i32>() {
                        Ok(value) => value * -1,
                        Err(_) => 0
                    }
                },
                _ => {
                    match str.chars().rev().collect::<String>().parse::<i32>() {
                        Ok(value) => value,
                        Err(_) => 0
                    }
                }
            }
        }
        None => return 0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn reverse_positive() {
        let result = super::reverse(12345);

        assert_eq!(result, 54321);
    }

    #[test]
    fn reverse_positive_ending_by_zero() {
        let result = super::reverse(120);

        assert_eq!(result, 21);
    }

    #[test]
    fn reverse_negative() {
        let result = super::reverse(-753);

        assert_eq!(result, -357);
    }

    #[test]
    fn reverse_negative_ending_by_zero() {
        let result = super::reverse(-9660);

        assert_eq!(result, -669);
    }

    #[test]
    fn reverse_overflow() {
        let result = super::reverse(1234567895);

        assert_eq!(result, 0);
    }
}