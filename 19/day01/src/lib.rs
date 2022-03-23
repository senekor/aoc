#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass_to_fuel() {
        assert_eq!(mass_to_fuel(12), 2);
        assert_eq!(mass_to_fuel(14), 2);
        assert_eq!(mass_to_fuel(1969), 654);
        assert_eq!(mass_to_fuel(100756), 33583);
    }

    #[test]
    fn test_mass_to_fuel_rec() {
        assert_eq!(mass_to_fuel_rec(14), 2);
        assert_eq!(mass_to_fuel_rec(1969), 966);
        assert_eq!(mass_to_fuel_rec(100756), 50346);
    }
}

fn mass_to_fuel(mass: u32) -> u32 {
    mass / 3 - 2
}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| mass_to_fuel(line.parse().unwrap()))
        .sum()
}

fn mass_to_fuel_rec(mass: u32) -> u32 {
    let x = mass / 3 - 2;
    if x > 8 {
        x + mass_to_fuel_rec(x)
    } else {
        x
    }
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| mass_to_fuel_rec(line.parse().unwrap()))
        .sum()
}
