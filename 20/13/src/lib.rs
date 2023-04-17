use std::collections::HashMap;

pub fn part1(input: &str) -> i32 {
    let departure = input.lines().next().unwrap();
    let buses = input.lines().nth(1).unwrap();
    let departure = departure.parse::<i32>().unwrap();
    let mut min_wait = departure * 2; // arbitrary value much bigger than solution
    let mut min_id = 0;
    for bus in buses.split(',') {
        if bus == "x" {
            continue;
        }
        let bus_id = bus.parse::<i32>().unwrap();
        let seconds_since_last_one = departure % bus_id;
        let wait = bus_id - seconds_since_last_one;
        if wait < min_wait {
            min_wait = wait;
            min_id = bus_id
        }
    }
    min_id * min_wait
}

fn prime_factors(mut x: usize) -> HashMap<usize, usize> {
    let mut res = HashMap::new();
    for i in 2..x + 1 {
        while x % i == 0 {
            *res.entry(i).or_default() += 1;
            x /= i;
            if x == 1 {
                return res;
            }
        }
    }
    panic!()
}

fn calc_kgv(
    mut factors_x: HashMap<usize, usize>,
    mut factors_y: HashMap<usize, usize>,
) -> (usize, HashMap<usize, usize>) {
    for key in factors_x.keys().copied().collect::<Vec<_>>() {
        if factors_y.contains_key(&key) {
            *factors_x.entry(key).or_default() = factors_x[&key].max(factors_y[&key]);
            factors_y.remove(&key);
        }
    }
    for key in factors_y.keys() {
        *factors_x.entry(*key).or_default() = factors_y[key]
    }
    let kgv = factors_x
        .keys()
        .fold(1, |acc, key| acc * key.pow(factors_x[key] as u32));
    (kgv, factors_x)
}

pub fn part2(input: &str) -> usize {
    let buses = input.lines().nth(1).unwrap();
    let buses = buses.split(',').collect::<Vec<_>>();
    let mut t = 0;
    let mut kgv = buses[0].parse::<usize>().unwrap();
    let mut factors = prime_factors(kgv);
    for (i, bus) in buses.into_iter().enumerate().skip(1) {
        if bus == "x" {
            continue;
        }
        let bus = bus.parse().unwrap();
        while (t + i) % bus != 0 {
            t += kgv;
        }
        (kgv, factors) = calc_kgv(factors, prime_factors(bus))
    }
    t
}
