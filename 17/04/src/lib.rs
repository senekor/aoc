use std::collections::HashSet;

use itertools::Itertools;
use utils::HashCount;

#[cfg(test)]
mod test_is_valid_passphrase {
    use super::*;

    #[test]
    fn a() {
        assert!(is_valid_passphrase("aa bb cc dd ee"))
    }

    #[test]
    fn b() {
        assert!(!is_valid_passphrase("aa bb cc dd aa"))
    }

    #[test]
    fn c() {
        assert!(is_valid_passphrase("aa bb cc dd aaa"))
    }
}

fn contains_duplicates(s: &str) -> bool {
    s.split_ascii_whitespace().count()
        != s.split_ascii_whitespace().collect::<HashSet<&str>>().len()
}

fn is_valid_passphrase(s: &str) -> bool {
    !contains_duplicates(s)
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| is_valid_passphrase(line))
        .count()
}

#[cfg(test)]
mod test_is_strong_passphrase {
    use super::*;

    #[test]
    fn a() {
        assert!(is_strong_passphrase("abcde fghij"))
    }

    #[test]
    fn b() {
        assert!(!is_strong_passphrase("abcde xyz ecdab"))
    }

    #[test]
    fn c() {
        assert!(is_strong_passphrase("a ab abc abd abf abj"))
    }

    #[test]
    fn d() {
        assert!(is_strong_passphrase("iiii oiii ooii oooi oooo"))
    }

    #[test]
    fn e() {
        assert!(!is_strong_passphrase("oiii ioii iioi iiio"))
    }
}

fn are_anagrams(a: &str, b: &str) -> bool {
    a.chars().collect::<HashCount<_>>() == b.chars().collect::<HashCount<_>>()
}

fn contains_anagrams(s: &str) -> bool {
    s.split_ascii_whitespace()
        .combinations(2)
        .any(|comb| are_anagrams(comb[0], comb[1]))
}

fn is_strong_passphrase(s: &str) -> bool {
    is_valid_passphrase(s) && !contains_anagrams(s)
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| is_strong_passphrase(line))
        .count()
}
