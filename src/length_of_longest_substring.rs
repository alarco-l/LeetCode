use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut occurences = HashSet::new();
    let mut len = 0;
    let mut max_length = 0;

    for (i, _) in s.chars().enumerate() {

        let tmp = &s[i..];
        for c in tmp.chars() {
            if !occurences.insert(c) {
                break;
            }
            len += 1;
        }

        if len >= max_length {
            max_length = len;
        }

        occurences.clear();
        len = 0;
    }
    max_length
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let s = String::from("abcabcbb");

        let result = super::length_of_longest_substring(s);

        assert_eq!(result, 3);
    }

    #[test]
    fn case2() {
        let s = String::from("pwwkew");

        let result = super::length_of_longest_substring(s);

        assert_eq!(result, 3);
    }

    #[test]
    fn case3() {
        let s = String::from("bdb");

        let result = super::length_of_longest_substring(s);

        assert_eq!(result, 2);
    }

    #[test]
    fn case4() {
        let s = String::from("dvdf");

        let result = super::length_of_longest_substring(s);

        assert_eq!(result, 3);
    }
}