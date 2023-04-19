use std::{array, fmt::Display, iter::once};

const HASH_LEN: usize = 256;
struct Hash<const L: usize>([u8; L]);

fn hash(input_lengths: impl Iterator<Item = usize> + Clone, rounds: usize) -> Hash<HASH_LEN> {
    let mut hash = array::from_fn(|i| i.try_into().unwrap());
    let mut cur_pos = 0;
    for (skip_size, input_len) in once(input_lengths)
        .cycle()
        .take(rounds)
        .flatten()
        .enumerate()
    {
        for (l, r) in (0..)
            .map(|i| (i, input_len.saturating_sub(1 + i)))
            .take_while(|(l, r)| l < r)
            .map(|(l, r)| ((cur_pos + l) % HASH_LEN, (cur_pos + r) % HASH_LEN))
        {
            (hash[l], hash[r]) = (hash[r], hash[l])
        }
        cur_pos = (cur_pos + input_len + skip_size) % HASH_LEN;
    }
    Hash(hash)
}

pub fn part1(input: &str) -> u32 {
    let input_lengths = input.split(',').map(|l| l.parse::<usize>().unwrap());
    let hash = hash(input_lengths, 1);
    u32::from(hash.0[0]) * u32::from(hash.0[1])
}

const DENSE_HASH_LEN: usize = 16;
const SPARSE_DENSE_RATIO: usize = HASH_LEN / DENSE_HASH_LEN;

impl Hash<HASH_LEN> {
    fn dense(self) -> Hash<DENSE_HASH_LEN> {
        Hash(array::from_fn(|i| {
            self.0[i * SPARSE_DENSE_RATIO..(i + 1) * SPARSE_DENSE_RATIO]
                .iter()
                .copied()
                .reduce(|a, b| a ^ b)
                .unwrap()
        }))
    }
}

impl Display for Hash<DENSE_HASH_LEN> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

pub fn part2(input: &str) -> String {
    let input_lengths = input.bytes().map(usize::from).chain([17, 31, 73, 47, 23]);
    hash(input_lengths, 64).dense().to_string()
}
