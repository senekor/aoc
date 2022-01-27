use std::cmp;
use std::str::FromStr;

#[derive(Debug)]
struct Reindeer {
    speed: i32,     // km/s
    fly_time: i32,  // sec
    rest_time: i32, // sec
}

type StrErr = &'static str;

impl FromStr for Reindeer {
    type Err = StrErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        iter.next().expect("no name found");
        Ok(Reindeer {
            speed: iter
                .nth(2)
                .ok_or("speed")?
                .parse()
                .map_err(|_| "speed parse int")?,
            fly_time: iter
                .nth(2)
                .ok_or("fly")?
                .parse()
                .map_err(|_| "fly parse int")?,
            rest_time: iter
                .nth(6)
                .ok_or("rest")?
                .parse()
                .map_err(|_| "fly parse int")?,
        })
    }
}

#[derive(Debug)]
struct Fleet(Vec<Reindeer>);

impl FromStr for Fleet {
    type Err = StrErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Fleet(
            s.lines()
                .map(|line| line.parse::<Reindeer>())
                .collect::<Result<Vec<Reindeer>, Self::Err>>()?,
        ))
    }
}

impl IntoIterator for Fleet {
    type Item = Reindeer;
    type IntoIter = <Vec<Reindeer> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Fleet {
    fn len(&self) -> usize {
        self.0.len()
    }
}

fn race_one(time: i32, candidate: Reindeer) -> i32 {
    let one_cycle = candidate.fly_time + candidate.rest_time;
    let completed_cycles = time / one_cycle;
    let rem_time = time % one_cycle;
    let rem_secs_flown = cmp::min(candidate.fly_time, rem_time);
    let total_secs_flown = completed_cycles * candidate.fly_time + rem_secs_flown;
    total_secs_flown * candidate.speed
}

fn part1(input: &'static str) {
    let max_distance = input
        .parse::<Fleet>()
        .expect("error parsing fleet")
        .into_iter()
        .map(|r| race_one(RACE_TIME, r))
        .reduce(cmp::max)
        .expect("the fleet was empty! no winner found.");
    println!("distance flown by fastest reindeer: {}", max_distance)
}

fn part2(input: &str) {
    let fleet = input.parse::<Fleet>().expect("error parsing fleet");
    let mut positions_and_points = Vec::with_capacity(fleet.len());
    positions_and_points.resize(fleet.len(), (0, 0));
    for current_time in 0..RACE_TIME {
        // calculate new positions
        for (reindeer_idx, pos_and_point) in positions_and_points.iter_mut().enumerate().take(fleet.len()) {
            let reindeer = &fleet.0[reindeer_idx];
            let cycle_time = reindeer.fly_time + reindeer.rest_time;
            let time_of_current_cycle = current_time % cycle_time;
            let is_flying = time_of_current_cycle < reindeer.fly_time;
            if is_flying {
                pos_and_point.0 += reindeer.speed;
            }
        }
        // assign points to leaders
        let leading_position = positions_and_points
            .iter()
            .fold(0, |acc, (pos, _)| cmp::max(acc, *pos));
        positions_and_points.iter_mut().for_each(|x| {
            if x.0 == leading_position {
                x.1 += 1
            }
        });
    }
    let winning_points = positions_and_points
        .iter()
        .fold(0, |acc, (_, points)| cmp::max(acc, *points));
    println!("{}", winning_points)
}

const RACE_TIME: i32 = 2503;

fn main() {
    let input = include_str!("../input/input.txt");

    part1(input);

    part2(input);

    let source = "";

    let split_iterator = source.split(' ');
    dbg!(split_iterator);
}
