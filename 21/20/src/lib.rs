use utils::Itertools;

fn pixel_square_to_index(mut pixel_square: Vec<char>) -> usize {
    let mut result = 0;
    pixel_square.reverse();
    for (i, &item) in pixel_square.iter().enumerate() {
        if item == '#' {
            result += 1 << i;
        }
    }
    result
}

#[derive(Debug, Clone)]
struct Image {
    pixels: Vec<Vec<char>>,
    infinite: char,
}

impl Image {
    fn get_pixel(&self, i: i32, j: i32) -> char {
        if i < 0 || i as usize >= self.pixels.len() || j < 0 || j as usize >= self.pixels[0].len() {
            self.infinite
        } else {
            self.pixels[i as usize][j as usize]
        }
    }

    fn get_next_infinite(&self, algo: &str) -> char {
        let index = if self.infinite == '#' {
            (1 << 9) - 1
        } else {
            0
        };
        algo.chars().nth(index).unwrap()
    }

    fn get_next(self, algo: &str) -> Image {
        let mut new_pixels = Vec::new();
        for i in -1..=(self.pixels.len() as i32) {
            new_pixels.push(Vec::new());

            for j in -1..=(self.pixels.len() as i32) {
                let pixel_square = vec![
                    self.get_pixel(i - 1, j - 1),
                    self.get_pixel(i - 1, j),
                    self.get_pixel(i - 1, j + 1),
                    self.get_pixel(i, j - 1),
                    self.get_pixel(i, j),
                    self.get_pixel(i, j + 1),
                    self.get_pixel(i + 1, j - 1),
                    self.get_pixel(i + 1, j),
                    self.get_pixel(i + 1, j + 1),
                ];
                let algo_index = pixel_square_to_index(pixel_square);
                new_pixels[(i + 1) as usize].push(algo.chars().nth(algo_index).unwrap());
            }
        }
        Image {
            pixels: new_pixels,
            infinite: self.get_next_infinite(algo),
        }
    }

    fn count_light_pixels(&self) -> usize {
        self.pixels
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&c| c == '#')
            .count()
    }
}

fn get_algo_image(input: &str) -> (&str, Image) {
    let mut iter = input.split("\n\n");
    let algo = iter.next().unwrap();
    let pixels = iter
        .next()
        .unwrap()
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let image = Image {
        pixels,
        infinite: '.',
    };

    (algo, image)
}

pub fn part1(input: &str) -> usize {
    let (algo, mut image) = get_algo_image(input);

    for _ in 0..2 {
        image = image.get_next(algo);
    }

    image.count_light_pixels()
}

pub fn part2(input: &str) -> usize {
    let (algo, mut image) = get_algo_image(input);

    for _ in 0..50 {
        image = image.get_next(algo);
    }

    image.count_light_pixels()
}
