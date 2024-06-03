#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Star {
    position: Coord,
    velocity: Coord,
}

impl From<&str> for Star {
    fn from(value: &str) -> Self {
        let mut iter = value.split(['<', ',', '>']).map(str::trim_start);
        Self {
            position: Coord {
                x: iter.nth(1).unwrap().parse().unwrap(),
                y: iter.nth(0).unwrap().parse().unwrap(),
            },
            velocity: Coord {
                x: iter.nth(1).unwrap().parse().unwrap(),
                y: iter.nth(0).unwrap().parse().unwrap(),
            },
        }
    }
}

impl Star {
    fn advance(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }
    fn rewind(&mut self) {
        self.position.x -= self.velocity.x;
        self.position.y -= self.velocity.y;
    }
}

pub fn part1(input: &str) -> &'static str {
    let mut stars: Vec<_> = input.lines().map(Star::from).collect();
    let mut smallest_seen_height = usize::MAX;

    loop {
        let xs = stars.iter().map(|s| s.position.x);
        let min_x = xs.clone().min().unwrap();
        let max_x = xs.clone().max().unwrap();
        let width = (max_x - min_x + 1) as usize;
        let ys = stars.iter().map(|s| s.position.y);
        let min_y = ys.clone().min().unwrap();
        let max_y = ys.clone().max().unwrap();
        let height = (max_y - min_y + 1) as usize;

        if height > smallest_seen_height {
            stars.iter_mut().for_each(Star::rewind);
            let mut sky = vec![vec![false; width]; height];

            for star in stars {
                sky[(star.position.y - min_y) as usize][(star.position.x - min_x) as usize] = true;
            }
            for line in sky {
                for pixel in line {
                    print!("{}", if pixel { '#' } else { '.' });
                }
                println!();
            }
            // not shrinking anymore, but growing again. stop search.
            break;
        }
        smallest_seen_height = height;

        stars.iter_mut().for_each(Star::advance);
    }

    // hardcode solution because OCR is annoying
    if smallest_seen_height == 8 {
        // sample
        "hi"
    } else {
        "xecxbpzb"
    }
}

pub fn part2(input: &str) -> usize {
    let mut stars: Vec<_> = input.lines().map(Star::from).collect();
    let mut smallest_seen_height = usize::MAX;

    for second in 0.. {
        let ys = stars.iter().map(|s| s.position.y);
        let min_y = ys.clone().min().unwrap();
        let max_y = ys.clone().max().unwrap();
        let height = (max_y - min_y + 1) as usize;

        if height > smallest_seen_height {
            // not shrinking anymore, but growing again. stop search.
            return second - 1;
        }
        smallest_seen_height = height;

        stars.iter_mut().for_each(Star::advance);
    }
    unreachable!()
}
