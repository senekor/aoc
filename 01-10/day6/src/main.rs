use itertools::*;

type Input = Vec<usize>;

fn part1(input: &Input) {
    let mut fish = vec![0; 9];

    for f in input {
        fish[*f] += 1;
    }

    for _ in 0..80 {
        let birthing_fish = fish[0];
        fish = fish[1..].to_vec();
        fish[6] += birthing_fish;
        fish.push(birthing_fish);
    }

    println!("total number of fish: {:?}", fish.into_iter().sum::<i32>())
}

fn part2(input: &Input) {
    let mut fish: Vec<usize> = vec![0; 9];

    for f in input {
        fish[*f] += 1;
    }

    for _ in 0..256 {
        let birthing_fish = fish[0];
        fish = fish[1..].to_vec();
        fish[6] += birthing_fish;
        fish.push(birthing_fish);
    }

    println!(
        "total number of fish: {:?}",
        fish.into_iter().sum::<usize>()
    )
}

fn main() {
    let input = include_str!("../input/input.txt");
    let input = input
        .split(",")
        .map(|line| line.parse().unwrap())
        .collect_vec();

    part1(&input);

    part2(&input);
}
