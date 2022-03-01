fn parse_i32(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

fn compare_sums(a: &str, b: &str, c: &str, x: &str, y: &str, z: &str) -> bool {
    parse_i32(a) + parse_i32(b) + parse_i32(c) < parse_i32(x) + parse_i32(y) + parse_i32(z)
}

pub fn lib_main() {
    let i = include_str!("input.txt");
    let mut s = i.split('\n');
    let mut x = s.next().unwrap();
    let mut y = s.next().unwrap();
    let mut c = 0;
    loop {
        if parse_i32(x) < parse_i32(y) {
            c += 1;
        }
        x = y;
        let n = s.next();
        match n {
            None => break,
            Some(some_n) => y = some_n,
        }
    }
    println!("{}", c);
    s = i.split('\n');
    x = s.next().unwrap();
    y = s.next().unwrap();
    let mut z = s.next().unwrap();
    let mut zz = s.next().unwrap();
    c = 0;
    loop {
        if compare_sums(x, y, z, y, z, zz) {
            c += 1;
        }
        x = y;
        y = z;
        z = zz;
        let n = s.next();
        match n {
            None => break,
            Some(some_n) => zz = some_n,
        }
    }
    println!("{}", c)
}
