#[derive(Clone)]
struct Polymer(Vec<u8>);

impl std::str::FromStr for Polymer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Polymer(s.bytes().collect()))
    }
}

impl Polymer {
    fn do_react(b1: u8, b2: u8) -> bool {
        match () {
            _ if b1 < b2 => b2 - b1 == 32,
            _ => b1 - b2 == 32,
        }
    }

    fn remove(&mut self, i: usize) {
        self.0.remove(i + 1);
        self.0.remove(i);
    }

    fn react(&mut self) {
        let mut i = 0;
        while i < self.0.len() - 1 {
            while i < self.0.len() - 1 && Polymer::do_react(self.0[i], self.0[i + 1]) {
                self.remove(i);
                i = i.saturating_sub(1)
            }
            i += 1;
        }
    }

    fn remove_char(&mut self, b: u8) {
        let mut i = 0;
        while i < self.0.len() {
            while i < self.0.len() && (self.0[i] == b || self.0[i] == b - 32) {
                self.0.remove(i);
            }
            i += 1;
        }
    }

    fn search_shortest(&self) -> usize {
        (b'a'..=b'z')
            .map(|b| {
                let mut new_polymer = self.clone();
                new_polymer.remove_char(b);
                new_polymer.react();
                new_polymer.0.len()
            })
            .min()
            .unwrap()
    }
}

pub fn part1(input: &str) -> usize {
    let mut polymer = input.parse::<Polymer>().unwrap();
    polymer.react();
    polymer.0.len()
}

pub fn part2(input: &str) -> usize {
    let polymer = input.parse::<Polymer>().unwrap();
    polymer.search_shortest()
}
