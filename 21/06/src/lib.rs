struct Input(Vec<usize>);

pub fn part1(input: &str) -> i32 {
    let input = Input(
        input
            .split(',')
            .map(|line| line.parse().unwrap())
            .collect::<Vec<_>>(),
    );

    let mut fish = vec![0; 9];

    for &f in &input.0 {
        fish[f] += 1;
    }

    for _ in 0..80 {
        let birthing_fish = fish[0];
        fish = fish[1..].to_vec();
        fish[6] += birthing_fish;
        fish.push(birthing_fish);
    }

    fish.into_iter().sum::<i32>()
}

pub fn part2(input: &str) -> usize {
    let input = Input(
        input
            .split(',')
            .map(|line| line.parse().unwrap())
            .collect::<Vec<_>>(),
    );

    let mut fish: Vec<usize> = vec![0; 9];

    for &f in &input.0 {
        fish[f] += 1;
    }

    for _ in 0..256 {
        let birthing_fish = fish[0];
        fish = fish[1..].to_vec();
        fish[6] += birthing_fish;
        fish.push(birthing_fish);
    }

    fish.into_iter().sum::<usize>()
}
