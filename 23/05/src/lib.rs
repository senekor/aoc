use winnow::{combinator::preceded, Parser};

mod parse;
mod std_trait_impls;

trait Category: std::fmt::Debug {
    const NAME: &'static str;
}

macro_rules! categories {
    ( $( $category: ident )+ ) => { $(
        #[derive(Debug)]
        enum $category {}
        impl Category for $category {
            const NAME: &'static str = stringify!($category);
        }
    )+ };
}
categories!(
    Seed
    Soil
    Fertilizer
    Water
    Light
    Temperature
    Humidity
    Location
);

type Int = u32;

#[derive(Debug)]
struct Number<T: Category> {
    inner: Int,
    _t: std::marker::PhantomData<T>,
}

impl<T: Category> From<Int> for Number<T> {
    fn from(value: Int) -> Self {
        Self {
            inner: value,
            _t: std::marker::PhantomData,
        }
    }
}
impl<T: Category> From<Number<T>> for Int {
    fn from(value: Number<T>) -> Self {
        value.inner
    }
}

impl<T: Category> std::ops::Add<Int> for Number<T> {
    type Output = Self;
    fn add(self, rhs: Int) -> Self::Output {
        Self {
            inner: self.inner + rhs,
            _t: std::marker::PhantomData,
        }
    }
}
impl<T: Category> std::ops::Sub for Number<T> {
    type Output = Int;
    fn sub(self, rhs: Self) -> Self::Output {
        self.inner - rhs.inner
    }
}

impl<T: Category> Number<T> {
    fn to_dest<Dest: Category>(self) -> Number<Dest> {
        Number {
            inner: self.inner,
            _t: std::marker::PhantomData,
        }
    }
    fn to_single_range(self) -> Range<T> {
        Range {
            start: self,
            length: 1,
        }
    }
}

#[derive(Debug)]
struct Range<T: Category> {
    start: Number<T>,
    length: Int,
}

impl<T: Category> Range<T> {
    /// cannot work with exclusive end because of overflow
    fn end_incl(&self) -> Number<T> {
        self.start + (self.length - 1)
    }
    fn to_dest<Dest: Category>(self) -> Range<Dest> {
        Range {
            start: self.start.to_dest(),
            length: self.length,
        }
    }
}

#[derive(Debug)]
struct MapRange<Src: Category, Dest: Category> {
    src: Range<Src>,
    dest: Range<Dest>,
}

impl<T: Category> Range<T> {
    // one source range may map to several destination ranges,
    // depending on overlaps with `map.ranges`.
    fn map<Dest: Category>(mut self, map: &Map<T, Dest>) -> Vec<Range<Dest>> {
        let mut res = Vec::new();
        for map_range in map.ranges.iter() {
            let src = map_range.src;
            if src.end_incl() < self.start {
                continue;
            } else if self.end_incl() < src.start {
                break;
            }
            // range overlap detected
            if self.start < src.start {
                if self.length <= (src.start - self.start) {
                    res.push(self.to_dest());
                    return res;
                }
                let start = self.start.to_dest();
                let length = src.start - self.start;
                res.push(Range { start, length });

                self.start = src.start;
                self.length -= length;
            }
            if self.start <= src.end_incl() {
                let to_mapped = |length| Range {
                    start: map_range.dest.start + (self.start - src.start),
                    length,
                };
                let length = src.end_incl() - self.start + 1;
                if self.length <= length {
                    res.push(to_mapped(self.length));
                    return res;
                }
                res.push(to_mapped(length));

                self.start = src.end_incl() + 1;
                self.length -= length;
            }
        }
        res.push(self.to_dest());
        res
    }
}

#[derive(Debug)]
struct Map<Src: Category, Dest: Category> {
    // sorted by start of range
    ranges: Vec<MapRange<Src, Dest>>,
}

struct Almanac {
    seed_to_soil: Map<Seed, Soil>,
    soil_to_fertilizer: Map<Soil, Fertilizer>,
    fertilizer_to_water: Map<Fertilizer, Water>,
    water_to_light: Map<Water, Light>,
    light_to_temperature: Map<Light, Temperature>,
    temperature_to_humidity: Map<Temperature, Humidity>,
    humidity_to_location: Map<Humidity, Location>,
}

impl Almanac {
    fn fully_map(&self, num: Vec<Range<Seed>>) -> Vec<Range<Location>> {
        num.into_iter()
            .flat_map(|r| r.map(&self.seed_to_soil))
            .flat_map(|r| r.map(&self.soil_to_fertilizer))
            .flat_map(|r| r.map(&self.fertilizer_to_water))
            .flat_map(|r| r.map(&self.water_to_light))
            .flat_map(|r| r.map(&self.light_to_temperature))
            .flat_map(|r| r.map(&self.temperature_to_humidity))
            .flat_map(|r| r.map(&self.humidity_to_location))
            .collect()
    }

    fn closest_location(&self, seeds: Vec<Range<Seed>>) -> Number<Location> {
        self.fully_map(seeds)
            .into_iter()
            .min_by_key(|loc| loc.start)
            .map(|r| r.start)
            .unwrap()
    }
}

pub fn part1(mut input: &str) -> u32 {
    let input = &mut input;
    let seeds = parse::seed_numbers(input).unwrap();
    preceded("\n\n", parse::almanac)
        .parse_next(input)
        .unwrap()
        .closest_location(seeds)
        .into()
}

pub fn part2(mut input: &str) -> u32 {
    let input = &mut input;
    let seeds = parse::seed_ranges(input).unwrap();
    preceded("\n\n", parse::almanac)
        .parse_next(input)
        .unwrap()
        .closest_location(seeds)
        .into()
}
