fn collision(input: &str, prefix: &str) -> u32 {
    for i in 0.. {
        let hash = format!("{:x}", md5::compute(format!("{}{}", input, i)));
        if hash.starts_with(prefix) {
            return i;
        }
    }
    panic!()
}

pub fn part1(input: &str) -> u32 {
    collision(input, "00000")
}

pub fn part2(input: &str) -> u32 {
    collision(input, "000000")
}
