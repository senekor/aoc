fn collision(input: &str, prefix: &str) -> u32 {
    for i in 0.. {
        let hash = format!("{:x}", md5::compute(format!("{}{}", input, i)));
        if hash.starts_with(prefix) {
            return i;
        }
    }
    panic!()
}

fn main() {
    let input = include_str!("../input/input.txt");
    println!("{}", collision(input, "00000"));
    println!("{}", collision(input, "000000"));
}
