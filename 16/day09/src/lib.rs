fn decompress(mut s: &str) -> String {
    let mut builder = String::new();

    while !s.is_empty() {
        let (uncompressed_sequence, t1) = s.split_once('(').unwrap();
        builder += uncompressed_sequence;

        let (rep_spec, t2) = t1.split_once(')').unwrap();
        let (len, num) = rep_spec.split_once('x').unwrap();
        let (compressed_sequence, t3) = t2.split_at(len.parse().unwrap());
        for _ in 0..num.parse().unwrap() {
            builder += compressed_sequence;
        }

        s = t3
    }

    builder
}

pub fn part1(input: &str) -> usize {
    decompress(input).len()
}

fn len_decomp_v2(mut s: &str) -> usize {
    let mut res = 0;

    while let Some((uncompressed_sequence, t1)) = s.split_once('(') {
        res += uncompressed_sequence.len();

        let (rep_spec, t2) = t1.split_once(')').unwrap();
        let (len, num) = rep_spec.split_once('x').unwrap();
        let (compressed_sequence, t3) = t2.split_at(len.parse().unwrap());
        res += len_decomp_v2(&compressed_sequence[..len.parse().unwrap()])
            * num.parse::<usize>().unwrap();

        s = t3
    }

    res + s.len()
}

pub fn part2(input: &str) -> usize {
    len_decomp_v2(input)
}
