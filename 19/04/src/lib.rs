#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_adjacent_duplicate_positive() {
        assert!(has_adjacent_duplicate("111111"))
    }

    #[test]
    fn test_has_adjacent_duplicate_negative() {
        assert!(!has_adjacent_duplicate("123789"))
    }

    #[test]
    fn test_digits_never_decrease_positive() {
        assert!(digits_never_decrease("111111"))
    }

    #[test]
    fn test_digits_never_decrease_negative() {
        assert!(!digits_never_decrease("223450"))
    }

    #[test]
    fn test_has_isolated_adjacent_duplicate_positive() {
        assert!(has_isolated_adjacent_duplicate("112233"))
    }

    #[test]
    fn test_has_isolated_adjacent_duplicate_negative() {
        assert!(!has_isolated_adjacent_duplicate("123444"))
    }

    #[test]
    fn test_has_isolated_adjacent_duplicate_bonus() {
        assert!(has_isolated_adjacent_duplicate("111122"))
    }
}

fn has_adjacent_duplicate(s: &str) -> bool {
    s.as_bytes().windows(2).any(|w| w[0] == w[1])
}

fn digits_never_decrease(s: &str) -> bool {
    s.as_bytes().windows(2).all(|w| w[0] <= w[1])
}

pub fn part1(input: &str) -> usize {
    let mut iter = input.split('-').map(|x| x.parse::<i32>().unwrap());
    (iter.next().unwrap()..iter.next().unwrap())
        .filter(|n| {
            let s = n.to_string();
            has_adjacent_duplicate(&s) && digits_never_decrease(&s)
        })
        .count()
}

fn has_isolated_adjacent_duplicate(s: &str) -> bool {
    let b = s.as_bytes();
    let n = b.len();
    (b[0] == b[1] && b[1] != b[2])
        || b.windows(4)
            .any(|w| w[0] != w[1] && w[1] == w[2] && w[2] != w[3])
        || (b[n - 1] == b[n - 2] && b[n - 2] != b[n - 3])
}

pub fn part2(input: &str) -> usize {
    let mut iter = input.split('-').map(|x| x.parse::<i32>().unwrap());
    (iter.next().unwrap()..iter.next().unwrap())
        .filter(|n| {
            let s = n.to_string();
            has_isolated_adjacent_duplicate(&s) && digits_never_decrease(&s)
        })
        .count()
}
