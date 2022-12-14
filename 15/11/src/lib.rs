use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref AMBIG_CHARS: Regex = Regex::new(r"[iol]").unwrap();
}

fn has_ambig_chars(password: &str) -> bool {
    AMBIG_CHARS.is_match(password)
}

fn is_three_straight(s: &[u8]) -> bool {
    s[0] < b'y' && s[0] + 1 == s[1] && s[1] + 1 == s[2]
}

fn has_three_straight(password: &str) -> bool {
    let bytes = password.as_bytes();
    if bytes.len() < 3 {
        return false;
    };
    for i in 0..bytes.len() - 2 {
        if is_three_straight(&bytes[i..i + 3]) {
            return true;
        }
    }
    false
}

fn has_two_pairs(password: &str) -> bool {
    let bytes = password.as_bytes();
    if bytes.len() < 3 {
        return false;
    };
    for i in 0..bytes.len() - 1 {
        if bytes[i] == bytes[i + 1] {
            for k in i + 2..bytes.len() - 1 {
                if bytes[k] == bytes[k + 1] {
                    return true;
                }
            }
            return false;
        }
    }
    false
}

fn is_valid(password: &str) -> bool {
    if has_ambig_chars(password) || !has_three_straight(password) || !has_two_pairs(password) {
        return false;
    }
    true
}

fn inc(password: &mut str) {
    unsafe {
        let bytes = password.as_bytes_mut();
        let mut non_z_idx = bytes.len() - 1;
        while non_z_idx > 0 && bytes[non_z_idx] == b'z' {
            bytes[non_z_idx] = b'a';
            non_z_idx -= 1;
        }
        bytes[non_z_idx] += 1
    }
}

pub fn part1(input: &str) -> String {
    let mut password = String::from(input);
    inc(&mut password);
    while !is_valid(&password) {
        inc(&mut password);
    }
    password
}

pub fn part2(input: &str) -> String {
    part1(&part1(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_ambig_chars() {
        assert!(!has_ambig_chars(""));
        assert!(!has_ambig_chars("a"));
        assert!(has_ambig_chars("i"));
        assert!(has_ambig_chars("o"));
        assert!(has_ambig_chars("l"));
        assert!(!has_ambig_chars("qwertyupkjhgfdsazxcvbnm"));
        assert!(has_ambig_chars("qwertyupkjhgfidsazxcvbnm"));
    }

    #[test]
    fn test_is_three_straight() {
        assert!(is_three_straight(&[b'a', b'b', b'c']));
        assert!(!is_three_straight(&[b'a', b'b', b'b']));
    }

    #[test]
    fn test_has_three_straight() {
        assert!(!has_three_straight(""));
        assert!(!has_three_straight("a"));
        assert!(has_three_straight("abc"));
        assert!(has_three_straight("xyz"));
        assert!(!has_three_straight("abb"));
        assert!(has_three_straight("qwerghiasdf"));
    }

    #[test]
    fn test_has_two_pairs() {
        assert!(!has_two_pairs(""));
        assert!(!has_two_pairs("aaa"));
        assert!(has_two_pairs("aaaa"));
        assert!(has_two_pairs("aabb"));
        assert!(!has_two_pairs("abaaba"));
        assert!(has_two_pairs("3456aaqwerzxcvzz1234"));
    }

    #[test]
    fn test_inc() {
        fn inc_return(word: &str) -> String {
            let mut s = String::from(word);
            inc(&mut s);
            s
        }
        assert_eq!("aab", inc_return("aaa"));
        assert_eq!("aac", inc_return("aab"));
        assert_eq!("aba", inc_return("aaz"));
        assert_eq!("baa", inc_return("azz"));
    }
}
