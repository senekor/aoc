use std::{iter::Peekable, str::Chars};

struct Stream<'a> {
    stream: Peekable<Chars<'a>>,
    groups: usize,
    garbage_chars: usize,
}

impl<'a> Stream<'a> {
    fn new(stream: &'a str) -> Self {
        Self {
            stream: stream.chars().peekable(),
            groups: 0,
            garbage_chars: 0,
        }
    }

    fn group(&mut self) -> usize {
        assert_eq!(self.stream.next(), Some('{'));
        self.groups += 1;
        let mut res = self.groups;
        loop {
            match *self.stream.peek().unwrap() {
                '<' => self.garbage(),
                '{' => res += self.group(),
                ',' => {
                    self.stream.next().unwrap();
                }
                '}' => break,
                _ => unreachable!(),
            }
        }
        assert_eq!(self.stream.next(), Some('}'));
        self.groups -= 1;
        res
    }

    fn garbage(&mut self) {
        assert_eq!(self.stream.next(), Some('<'));
        while let Some(ch) = self.stream.next() {
            match ch {
                '>' => return,
                '!' => {
                    self.stream.next();
                }
                _ => self.garbage_chars += 1,
            }
        }
        unreachable!()
    }
}

pub fn part1(input: &str) -> usize {
    Stream::new(input).group()
}

pub fn part2(input: &str) -> usize {
    let mut stream = Stream::new(input);
    stream.group();
    stream.garbage_chars
}

// #[cfg(test)]
// mod unit_tests {
//     use super::*;

//     #[test]
//     fn sanity_check() {
//         assert_eq!(1 + 1, 2)
//     }
// }
