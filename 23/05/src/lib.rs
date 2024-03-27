use std::str::FromStr;

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

impl<Src: Category> Number<Src> {
    fn map<Dest: Category>(self, map: &Map<Src, Dest>) -> Number<Dest> {
        let range =
            map.ranges.iter().copied().find(|r| {
                r.source_range_start <= self && self < r.source_range_start + r.range_length
            });
        match range {
            Some(r) => r.destination_range_start + (self - r.source_range_start),
            None => Number::from(self.inner),
        }
    }
}

#[derive(Debug)]
struct Range<Src: Category, Dest: Category> {
    destination_range_start: Number<Dest>,
    source_range_start: Number<Src>,
    range_length: Int,
}

#[derive(Debug)]
struct Map<Src: Category, Dest: Category> {
    ranges: Vec<Range<Src, Dest>>,
}

struct Almanac {
    seeds: Vec<Number<Seed>>,
    seed_to_soil: Map<Seed, Soil>,
    soil_to_fertilizer: Map<Soil, Fertilizer>,
    fertilizer_to_water: Map<Fertilizer, Water>,
    water_to_light: Map<Water, Light>,
    light_to_temperature: Map<Light, Temperature>,
    temperature_to_humidity: Map<Temperature, Humidity>,
    humidity_to_location: Map<Humidity, Location>,
}

impl FromStr for Almanac {
    type Err = ();

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        parse::almanac(&mut s).map_err(|_| ())
    }
}

impl Almanac {
    fn fully_map(&self, num: Number<Seed>) -> Number<Location> {
        num.map(&self.seed_to_soil)
            .map(&self.soil_to_fertilizer)
            .map(&self.fertilizer_to_water)
            .map(&self.water_to_light)
            .map(&self.light_to_temperature)
            .map(&self.temperature_to_humidity)
            .map(&self.humidity_to_location)
    }

    fn closest_location(&self) -> Number<Location> {
        self.seeds
            .iter()
            .map(|&s| self.fully_map(s))
            .min_by_key(|loc| loc.inner)
            .unwrap()
    }
}

pub fn part1(input: &str) -> u32 {
    input.parse::<Almanac>().unwrap().closest_location().into()
}

pub fn part2(input: &str) -> usize {
    0
}
