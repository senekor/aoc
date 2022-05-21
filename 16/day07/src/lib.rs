use std::collections::HashSet;

enum Sequence {
    Supernet(String),
    Hypernet(String),
}

#[cfg(test)]
mod test_sequence {
    use super::*;
    use std::ops::Not;

    #[test]
    fn test_has_abba_1() {
        assert!(Sequence::Supernet(String::from("aaaa")).has_abba().not())
    }

    #[test]
    fn test_has_abba_2() {
        assert!(Sequence::Hypernet(String::from("qwer")).has_abba().not())
    }

    #[test]
    fn test_has_abba_3() {
        assert!(Sequence::Supernet(String::from("tyui")).has_abba().not())
    }
}

impl Sequence {
    fn has_abba(&self) -> bool {
        let supernet = match self {
            Self::Supernet(s) => s,
            Self::Hypernet(s) => s,
        };
        supernet
            .as_bytes()
            .windows(4)
            .any(|a| a[0] == a[3] && a[1] == a[2] && a[0] != a[1])
    }

    fn get_every_aba(&self) -> HashSet<String> {
        let supernet = match self {
            Sequence::Supernet(s) => s,
            Sequence::Hypernet(_) => return HashSet::default(),
        };
        supernet
            .as_bytes()
            .windows(3)
            .filter(|a| a[0] == a[2] && a[0] != a[1])
            .map(|a| String::from_utf8(a.to_vec()).unwrap())
            .collect()
    }

    fn has_any_bab(&self, every_bab: &HashSet<String>) -> bool {
        let hypernet = match self {
            Sequence::Supernet(_) => return false,
            Sequence::Hypernet(s) => s,
        };
        every_bab.iter().any(|bab| hypernet.contains(bab))
    }
}

struct IPv7(Vec<Sequence>);

impl std::str::FromStr for IPv7 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sequences = vec![];
        let mut iter = s.split(&['[', ']']);
        sequences.push(Sequence::Supernet(iter.next().unwrap().to_string()));

        while let Some(hypernet_sequence) = iter.next() {
            sequences.push(Sequence::Hypernet(hypernet_sequence.to_string()));
            sequences.push(Sequence::Supernet(iter.next().unwrap().to_string()));
        }
        Ok(IPv7(sequences))
    }
}

#[cfg(test)]
mod test_ipv7 {
    use super::*;
    use std::ops::Not;

    #[test]
    fn test_supports_tls_1() {
        assert!(IPv7(vec![
            Sequence::Supernet(String::from("abba")),
            Sequence::Hypernet(String::from("mnop")),
            Sequence::Supernet(String::from("qrst")),
        ])
        .supports_TLS())
    }

    #[test]
    fn test_supports_tls_2() {
        assert!(IPv7(vec![
            Sequence::Supernet(String::from("abcd")),
            Sequence::Hypernet(String::from("bddb")),
            Sequence::Supernet(String::from("xyyx")),
        ])
        .supports_TLS()
        .not())
    }

    #[test]
    fn test_supports_tls_3() {
        assert!(IPv7(vec![
            Sequence::Supernet(String::from("aaaa")),
            Sequence::Hypernet(String::from("qwer")),
            Sequence::Supernet(String::from("tyui")),
        ])
        .supports_TLS()
        .not())
    }

    #[test]
    fn test_supports_tls_4() {
        assert!(IPv7(vec![
            Sequence::Supernet(String::from("ioxxoj")),
            Sequence::Hypernet(String::from("asdfgh")),
            Sequence::Supernet(String::from("zxcvbn")),
        ])
        .supports_TLS())
    }

    #[test]
    fn test_supports_ssl_1() {
        assert!(IPv7(vec![
            Sequence::Supernet(String::from("aba")),
            Sequence::Hypernet(String::from("bab")),
            Sequence::Supernet(String::from("xyz")),
        ])
        .supports_SSL())
    }

    #[test]
    fn test_supports_ssl_2() {
        assert!(IPv7(vec![
            Sequence::Supernet(String::from("xyx")),
            Sequence::Hypernet(String::from("xyx")),
            Sequence::Supernet(String::from("xyx")),
        ])
        .supports_SSL()
        .not())
    }

    #[test]
    fn test_supports_ssl_3() {
        assert!(IPv7(vec![
            Sequence::Supernet(String::from("aaa")),
            Sequence::Hypernet(String::from("kek")),
            Sequence::Supernet(String::from("eke")),
        ])
        .supports_SSL())
    }

    #[test]
    fn test_supports_ssl_4() {
        assert!(IPv7(vec![
            Sequence::Supernet(String::from("zazbz")),
            Sequence::Hypernet(String::from("bzb")),
            Sequence::Supernet(String::from("cdb")),
        ])
        .supports_SSL())
    }
}

impl IPv7 {
    #[allow(non_snake_case)]
    fn supports_TLS(&self) -> bool {
        let mut found_regular_abba = false;
        for seq in self.0.iter() {
            if seq.has_abba() {
                match seq {
                    Sequence::Supernet(_) => found_regular_abba = true,
                    Sequence::Hypernet(_) => return false,
                }
            }
        }
        found_regular_abba
    }

    #[allow(non_snake_case)]
    fn supports_SSL(&self) -> bool {
        let every_aba = self.0.iter().flat_map(|a| a.get_every_aba());

        let every_bab: HashSet<String> = every_aba
            .map(|aba| format!("{}{}{}", &aba[1..2], &aba[0..1], &aba[1..2]))
            .collect();

        self.0.iter().any(|seq| seq.has_any_bab(&every_bab))
    }
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .filter(IPv7::supports_TLS)
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .filter(IPv7::supports_SSL)
        .count()
}
