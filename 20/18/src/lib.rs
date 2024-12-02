use std::iter::Peekable;

#[derive(Debug, PartialEq)]
enum Token {
    OpenParen,
    CloseParen,
    Plus,
    Star,
    Num(i64),
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        match value {
            "(" => Token::OpenParen,
            ")" => Token::CloseParen,
            "+" => Token::Plus,
            "*" => Token::Star,
            n => Token::Num(n.parse().unwrap()),
        }
    }
}

struct Tokenizer<'a>(Peekable<Box<dyn Iterator<Item = &'a str> + 'a>>);

impl<'a> Tokenizer<'a> {
    fn new(value: &'a str) -> Self {
        let iter: Box<dyn Iterator<Item = &'a str> + 'a> =
            Box::new(value.split("").filter(|&t| !t.is_empty() && t != " "));
        Tokenizer(iter.peekable())
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.next()?.into())
    }
}

impl Tokenizer<'_> {
    fn peek(&mut self) -> Option<<Self as Iterator>::Item> {
        self.0.peek().map(|&s| Token::from(s))
    }

    fn has(&mut self, t: Token) -> bool {
        if self.peek() == Some(t) {
            self.next();
            return true;
        }
        false
    }

    fn expect(&mut self, t: Token) -> Result<(), ()> {
        if self.peek() == Some(t) {
            self.next();
            return Ok(());
        }
        Err(())
    }

    fn term(&mut self) -> Result<i64, ()> {
        if self.has(Token::OpenParen) {
            let val = self.expr()?;
            self.expect(Token::CloseParen)?;
            return Ok(val);
        }
        if let Some(Token::Num(n)) = self.next() {
            return Ok(n);
        }
        Err(())
    }

    fn expr(&mut self) -> Result<i64, ()> {
        let mut val = self.term()?;
        loop {
            if self.has(Token::Plus) {
                val += self.term()?
            } else if self.has(Token::Star) {
                val *= self.term()?
            } else {
                break;
            }
        }
        Ok(val)
    }

    fn term_2(&mut self) -> Result<i64, ()> {
        if self.has(Token::OpenParen) {
            let val = self.mul_chain()?;
            self.expect(Token::CloseParen)?;
            return Ok(val);
        }
        if let Some(Token::Num(n)) = self.next() {
            return Ok(n);
        }
        Err(())
    }

    fn add_chain(&mut self) -> Result<i64, ()> {
        let mut val = self.term_2()?;
        while self.has(Token::Plus) {
            val += self.term_2()?
        }
        Ok(val)
    }

    fn mul_chain(&mut self) -> Result<i64, ()> {
        let mut val = self.add_chain()?;
        while self.has(Token::Star) {
            val *= self.add_chain()?
        }
        Ok(val)
    }
}

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| Tokenizer::new(l).expr().unwrap())
        .sum()
}

pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|l| Tokenizer::new(l).mul_chain().unwrap())
        .sum()
}
