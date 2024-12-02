const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const NUM_PIX_PER_LAYER: usize = WIDTH * HEIGHT;

const BLACK: u8 = b'0';
const WHITE: u8 = b'1';
const TRANSPARENT: u8 = b'2';

struct Row<'a> {
    pixels: &'a [u8; WIDTH],
}

struct Layer<'a> {
    rows: [Row<'a>; HEIGHT],
}

impl Layer<'_> {
    fn count_digits(&self, digit: u8) -> usize {
        self.rows
            .iter()
            .flat_map(|r| r.pixels.iter())
            .filter(|&&b| b == digit)
            .count()
    }

    fn pixel_at(&self, row: usize, col: usize) -> u8 {
        self.rows[row].pixels[col]
    }
}

struct Image<'a> {
    layers: Vec<Layer<'a>>,
}

impl<'a> From<&'a str> for Image<'a> {
    fn from(s: &'a str) -> Self {
        let layers = s
            .as_bytes()
            .chunks_exact(NUM_PIX_PER_LAYER)
            .map(|layer_chunk| {
                let mut row_iter = layer_chunk.chunks(WIDTH);
                let rows = std::array::from_fn(|_| {
                    let pixels = row_iter
                        .next()
                        .expect("row_iter should have HEIGHT number of elements")
                        .try_into()
                        .unwrap();
                    Row { pixels }
                });
                Layer { rows }
            })
            .collect();
        Self { layers }
    }
}

impl<'a> Image<'a> {
    fn get_layer_with_fewest_zeros(&self) -> &Layer<'a> {
        self.layers
            .iter()
            .min_by_key(|l| l.count_digits(b'0'))
            .unwrap()
    }

    fn pixel_at(&self, row: usize, col: usize) -> u8 {
        for layer in self.layers.iter() {
            let pixel = layer.pixel_at(row, col);
            if pixel != TRANSPARENT {
                return pixel;
            }
        }
        unreachable!("no non-transparent pixels found at ({row},{col})");
    }

    fn decode(&self) -> [[u8; WIDTH]; HEIGHT] {
        let mut row_iter = (0..HEIGHT).map(|row| {
            let mut pixel_iter = (0..WIDTH).map(|col| self.pixel_at(row, col));
            std::array::from_fn(|_| pixel_iter.next().unwrap())
        });
        std::array::from_fn(|_| row_iter.next().unwrap())
    }
}

pub fn part1(input: &str) -> usize {
    let image = Image::from(input);
    let layer = image.get_layer_with_fewest_zeros();
    layer.count_digits(b'1') * layer.count_digits(b'2')
}

pub fn part2(input: &str) -> &'static str {
    let image = Image::from(input);
    let decoded_image = image.decode();

    for row in decoded_image.iter() {
        for pixel in row.iter() {
            match *pixel {
                BLACK => print!("."),
                WHITE => print!("#"),
                _ => unreachable!("pixel should be black or white"),
            }
        }
        println!();
    }

    // TODO implement OCR
    "FAHEF"
}
