

fn collision(input: &str, prefix: &str) {
    for i in 0.. {
        let hash = format!("{:x}", md5::compute(format!("{}{}", input, i)));
        if hash.get(0..prefix.len()).unwrap() == prefix {
            println!("{}", i);
            break;
        }
    }
}

fn main() {
    let input = include_str!("../input/input.txt");
    collision(input, "00000");
    collision(input, "000000");
}
