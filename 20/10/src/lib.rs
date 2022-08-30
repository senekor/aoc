fn get_nums(input: &str) -> Vec<i32> {
    let mut nums = input
        .lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();
    nums.push(0);
    nums.sort_unstable();
    nums.push(nums.last().unwrap() + 3);
    nums
}

pub fn part1(input: &str) -> i32 {
    let nums = get_nums(input);

    let (mut ones, mut threes) = (0, 0);
    for w in nums.windows(2) {
        match w[1] - w[0] {
            1 => ones += 1,
            3 => threes += 1,
            _ => {}
        }
    }

    ones * threes
}

pub fn part2(input: &str) -> usize {
    let nums = get_nums(input);
    let mut dp_table = vec![0_usize; nums.len()];

    dp_table[0] = 1;
    dp_table[1] = 1;
    dp_table[2] = 1;
    if nums[2] - nums[0] <= 3 {
        dp_table[2] += dp_table[0];
    }

    for i in 3..nums.len() {
        dp_table[i] = dp_table[i - 1];

        if nums[i] - nums[i - 2] <= 3 {
            dp_table[i] += dp_table[i - 2];
        }

        if nums[i] - nums[i - 3] <= 3 {
            dp_table[i] += dp_table[i - 3];
        }
    }

    *dp_table.last().unwrap()
}
