fn parse_i32(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
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
        if parse_i32(x) + parse_i32(y) + parse_i32(z)
            < parse_i32(y) + parse_i32(z) + parse_i32(zz)
        {
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