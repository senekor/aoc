use std::cmp::Ordering;
use std::collections::HashMap;

impl std::str::FromStr for Room {
    type Err = String;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut room = Room::default();
        let mut iter = line.split(&['-', '[', ']']);
        while let Some(token) = iter.next() {
            if let Ok(number) = token.parse::<i32>() {
                room.sector_id = number;
                let mut char_iter = iter.next().unwrap().chars();
                room.checksum = [
                    char_iter.next().unwrap(),
                    char_iter.next().unwrap(),
                    char_iter.next().unwrap(),
                    char_iter.next().unwrap(),
                    char_iter.next().unwrap(),
                ];
                break;
            }
            room.name.push(token.to_string());
        }
        Ok(room)
    }
}

#[derive(Default, Debug)]
struct Room {
    name: Vec<String>,
    sector_id: i32,
    checksum: [char; 5],
}

impl Room {
    fn tally_chars(&self) -> HashMap<char, u32> {
        let mut tally = HashMap::new();
        for c in self.name.iter().flat_map(|s| s.chars()) {
            *tally.entry(c).or_default() += 1;
        }
        tally
    }

    fn calc_checksum(&self) -> [char; 5] {
        let tallied_chars = self.tally_chars();
        let sorted_chars = {
            let mut v = tallied_chars.into_iter().collect::<Vec<_>>();
            v.sort_by(|a, b| match a.1.cmp(&b.1) {
                Ordering::Greater => Ordering::Less,
                Ordering::Equal => a.0.cmp(&b.0),
                Ordering::Less => Ordering::Greater,
            });
            v
        };
        [
            sorted_chars[0].0,
            sorted_chars[1].0,
            sorted_chars[2].0,
            sorted_chars[3].0,
            sorted_chars[4].0,
        ]
    }

    fn is_real(&self) -> bool {
        self.calc_checksum() == self.checksum
    }

    fn get_decrypted_name(&self) -> String {
        let mut res = String::new();
        let num_rotations = (self.sector_id % 26) as u8;
        for word in self.name.iter() {
            res.push(' ');
            for b in word.bytes() {
                let mut new_b = b + num_rotations;
                if new_b > 122 {
                    new_b -= 26
                }
                res.push(new_b.into())
            }
        }
        res
    }
}

pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.parse::<Room>().unwrap())
        .filter(|room| room.is_real())
        .map(|room| room.sector_id)
        .sum()
}

pub fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.parse::<Room>().unwrap())
        .filter(|room| room.is_real())
        .find(|room| room.get_decrypted_name().contains("north"))
        .unwrap()
        .sector_id
}
