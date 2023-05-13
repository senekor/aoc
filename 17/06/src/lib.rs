use std::{array::from_fn, collections::HashMap, hash::Hash};

#[derive(Clone, Default, PartialEq, Eq, Hash, PartialOrd)]
pub struct MemoryBank {
    blocks: usize,
}

pub type Banks<const NUM_BANKS: usize> = [MemoryBank; NUM_BANKS];

#[derive(Clone, Default)]
struct ReallocRoutine<const NUM_BANKS: usize>
where
    Banks<NUM_BANKS>: Default + PartialEq + Eq + Hash,
{
    current: Banks<NUM_BANKS>,
    seen: HashMap<Banks<NUM_BANKS>, usize>,
}

impl<const NUM_BANKS: usize> From<&str> for ReallocRoutine<NUM_BANKS>
where
    Banks<NUM_BANKS>: Default + PartialEq + Eq + Hash,
{
    fn from(s: &str) -> Self {
        let mut iter = s.split_ascii_whitespace().flat_map(|s| s.parse());
        Self {
            current: from_fn(|_| MemoryBank {
                blocks: iter.next().expect("not enough memory banks"),
            }),
            seen: Default::default(),
        }
    }
}

impl<const NUM_BANKS: usize> ReallocRoutine<NUM_BANKS>
where
    Banks<NUM_BANKS>: Default + PartialEq + Eq + Hash,
{
    fn find_biggest_bank_idx(&self) -> usize {
        // ties won by lowest-numbered memory bank
        let mut biggest_idx = 0;
        for i in 0..NUM_BANKS {
            if self.current[biggest_idx] < self.current[i] {
                biggest_idx = i;
            }
        }
        biggest_idx
    }

    fn exec_one_cycle(&mut self) {
        let biggest_bank_idx = self.find_biggest_bank_idx();
        let biggest_bank = std::mem::take(&mut self.current[biggest_bank_idx]);
        for idx in (0..NUM_BANKS)
            .cycle()
            .skip(biggest_bank_idx + 1)
            .take(biggest_bank.blocks)
        {
            self.current[idx].blocks += 1;
        }
    }

    fn find_infinite_loop(&mut self) -> usize {
        let mut count = 0;
        while !self.seen.contains_key(&self.current) {
            self.seen.insert(self.current.clone(), count);
            self.exec_one_cycle();
            count += 1;
        }
        count
    }
}

pub fn part1_impl<const NUM_BANKS: usize>(input: &str) -> usize
where
    Banks<NUM_BANKS>: Default + PartialEq + Eq + Hash,
{
    ReallocRoutine::<NUM_BANKS>::from(input).find_infinite_loop()
}

pub fn part1(input: &str) -> usize {
    part1_impl::<16>(input)
}

pub fn part2_impl<const NUM_BANKS: usize>(input: &str) -> usize
where
    Banks<NUM_BANKS>: Default + PartialEq + Eq + Hash,
{
    let mut routine = ReallocRoutine::<NUM_BANKS>::from(input);
    let count = routine.find_infinite_loop();
    count - routine.seen[&routine.current]
}

pub fn part2(input: &str) -> usize {
    part2_impl::<16>(input)
}
