type Chip = usize;
type Bot = usize;

fn parse_bot_source(line: &str) -> (Chip, Bot) {
    let mut iter = line.split_whitespace().filter_map(|w| w.parse().ok());
    (iter.next().expect("no chip"), iter.next().expect("no bot"))
}

fn parse_bot_distribution_rule(line: &str) -> (Bot, Output, Output) {
    let mut iter = line.split_whitespace();
    (
        iter.nth(1).map(|s| s.parse().unwrap()).expect("no bot"),
        if iter.nth(3).unwrap() == "bot" {
            Output::Bot(iter.next().map(|s| s.parse().unwrap()).expect("no low bot"))
        } else {
            Output::Bin(iter.next().map(|s| s.parse().unwrap()).expect("no low bin"))
        },
        if iter.nth(3).unwrap() == "bot" {
            Output::Bot(
                iter.next()
                    .map(|s| s.parse().unwrap())
                    .expect("no high bot"),
            )
        } else {
            Output::Bin(
                iter.next()
                    .map(|s| s.parse().unwrap())
                    .expect("no high bin"),
            )
        },
    )
}

#[derive(Debug, Clone, Copy)]
enum Output {
    Bot(Bot),
    Bin(usize),
}

#[derive(Debug, Clone, Default)]
struct BotInstructions(Vec<(Output, Output)>);

#[derive(Debug, Clone, Default)]
struct ChipStorage {
    bots: Vec<Vec<Chip>>,
    bins: Vec<Option<Chip>>,
}

fn parse_input(input: &str) -> (BotInstructions, ChipStorage) {
    let mut instructions = BotInstructions::default();
    let mut storage = ChipStorage::default();
    for line in input.lines() {
        if &line[0..1] == "v" {
            let (chip, bot) = parse_bot_source(line);
            storage.insert(Output::Bot(bot), chip);
        } else {
            let (bot, low, high) = parse_bot_distribution_rule(line);
            if instructions.0.len() <= bot {
                instructions
                    .0
                    .resize(bot * 2 + 1, (Output::Bot(0), Output::Bot(0)))
            }
            instructions.0[bot] = (low, high);
        }
    }
    (instructions, storage)
}

impl ChipStorage {
    fn insert(&mut self, output: Output, chip: Chip) {
        match output {
            Output::Bot(bot) => {
                if self.bots.len() <= bot {
                    self.bots.resize(bot * 2 + 1, vec![])
                }
                self.bots[bot].push(chip)
            }
            Output::Bin(bin) => {
                if self.bins.len() <= bin {
                    self.bins.resize(bin * 2 + 1, None)
                }
                self.bins[bin] = Some(chip)
            }
        }
    }

    fn insert_rec(&mut self, output: Output, chip: Chip, instructions: &BotInstructions) {
        match output {
            Output::Bot(bot) => {
                if self.bots.len() <= bot {
                    self.bots.resize(bot * 2 + 1, vec![])
                }
                self.bots[bot].push(chip);
                if self.bots[bot].len() == 2 {
                    self.bots[bot].sort_unstable();

                    let low_chip = self.bots[bot][0];
                    let high_chip = self.bots[bot][1];

                    self.insert_rec(instructions.0[bot].0, low_chip, instructions);
                    self.insert_rec(instructions.0[bot].1, high_chip, instructions);
                }
            }
            Output::Bin(bin) => {
                if self.bins.len() <= bin {
                    self.bins.resize(bin * 2 + 1, None)
                }
                self.bins[bin] = Some(chip)
            }
        }
    }

    fn execute(&mut self, instructions: BotInstructions) {
        for bot in 0..self.bots.len() {
            if self.bots[bot].len() == 2 {
                self.bots[bot].sort_unstable();

                let low_chip = self.bots[bot][0];
                let high_chip = self.bots[bot][1];

                self.insert_rec(instructions.0[bot].0, low_chip, &instructions);
                self.insert_rec(instructions.0[bot].1, high_chip, &instructions);
            }
        }
    }
}

pub fn part1(input: &str, v1: usize, v2: usize) -> Bot {
    let (instructions, mut storage) = parse_input(input);

    storage.execute(instructions);

    for (bot, chips) in storage.bots.iter().enumerate() {
        if chips.contains(&v1) && chips.contains(&v2) {
            return bot;
        }
    }

    panic!("no matching bot found")
}

pub fn part2(input: &str) -> usize {
    let (instructions, mut storage) = parse_input(input);

    storage.execute(instructions);

    storage.bins[0].unwrap() * storage.bins[1].unwrap() * storage.bins[2].unwrap()
}
