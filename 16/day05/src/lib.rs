use std::fmt::Write;

pub fn part1(input: &str) -> String {
    let mut res = String::new();
    let mut i = 0;
    loop {
        i += 1;
        let hash = md5::compute(format!("{input}{i}"));
        if hash.starts_with(&[0; 2]) && hash[2] <= 0x0f {
            write!(res, "{:x}", hash[2]).expect("failed to write");
            if res.len() >= 8 {
                return res;
            }
        }
    }
}

pub fn part2(input: &str) -> String {
    let mut res = [None; 8];
    let mut i = 0;
    loop {
        i += 1;
        let hash = md5::compute(format!("{input}{i}"));
        if hash.starts_with(&[0; 2]) && hash[2] < 8 {
            if res[hash[2] as usize].is_some() {
                continue;
            }
            res[hash[2] as usize] = Some(hash[3] >> 4);
            if res.iter().all(|x| x.is_some()) {
                break;
            }
        }
    }
    res.into_iter()
        .fold(String::new(), |acc, x| format!("{}{:x}", acc, x.unwrap()))
}
