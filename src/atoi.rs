fn is_digit(c: char) -> bool {
    if c < '0' || c > '9' {
        return false
    }
    true
}

pub fn my_atoi(str: String) -> i32 {
    let s = str.trim();
    let mut index = 0;

    match s.chars().next() {
        Some(c) => {
            if c != '-' && c != '+' && !is_digit(c) {
                return 0
            }

            for (i, c) in s.chars().enumerate() {
                if i > 0 && !is_digit(c) {
                    index = i;
                    break;
                }
            }

            index = if index == 0 { s.len() } else { index };

            match s[0..index].parse::<i32>() {
                Ok(value) => value,
                Err(err) => {
                    if err.to_string() == "number too small to fit in target type" {
                        return i32::MIN
                    }
                    else if err.to_string() == "number too large to fit in target type" {
                        return i32::MAX
                    }
                    0
                }
            }
        }
        None => 0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn negative_number() {
        let result = super::my_atoi(String::from("               -42                  "));

        assert_eq!(result, -42);
    }

    #[test]
    fn postive_number() {
        let result = super::my_atoi(String::from("        +42 abc"));

        assert_eq!(result, 42);
    }

    #[test]
    fn invalid_input() {
        let result = super::my_atoi(String::from("qwer       3421 abc"));

        assert_eq!(result, 0);
    }

    #[test]
    fn underflow() {
        let result = super::my_atoi(String::from("-91283472332"));

        assert_eq!(result, i32::MIN);
    }

    #[test]
    fn ending_sign() {
        let result = super::my_atoi(String::from("-9-"));

        assert_eq!(result, -9);
    }
}