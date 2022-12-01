struct Probe {
    len: i32,
    pos: i32,
    incr: i32,
}

impl Probe {
    fn left() -> Probe {
        Probe {
            len: 0,
            pos: 1,
            incr: 5,
        }
    }
    fn right() -> Probe {
        Probe {
            len: 0,
            pos: 1,
            incr: 1,
        }
    }
    fn up() -> Probe {
        Probe {
            len: 0,
            pos: 1,
            incr: 3,
        }
    }
    fn down() -> Probe {
        Probe {
            len: 0,
            pos: 1,
            incr: 7,
        }
    }

    fn extend(&mut self) {
        self.len += 1;
        self.pos += self.incr;
        self.incr += 8;
    }

    fn approach(&mut self, target: i32) {
        let mut next_pos = self.pos + self.incr;
        while next_pos < target {
            self.extend();
            next_pos = self.pos + self.incr;
        }
        let current_dist = (self.pos - target).abs();
        let next_dist = (next_pos - target).abs();
        if next_dist < current_dist {
            self.extend();
        }
    }

    fn get_dist_to(&self, target: i32) -> i32 {
        (self.pos - target).abs() + self.len
    }
}

struct Probes([Probe; 4]);

impl Probes {
    fn new() -> Probes {
        Probes([Probe::left(), Probe::right(), Probe::up(), Probe::down()])
    }

    fn approach(&mut self, target: i32) {
        self.0[0].approach(target);
        self.0[1].approach(target);
        self.0[2].approach(target);
        self.0[3].approach(target);
    }

    fn find_min_dist(&mut self, target: i32) -> i32 {
        self.approach(target);
        self.0
            .iter()
            .map(|probe| probe.get_dist_to(target))
            .min()
            .unwrap()
    }
}

pub fn part1(input: &str) -> i32 {
    Probes::new().find_min_dist(input.parse().unwrap())
}

struct SpiralGrid {
    nums: Vec<i32>,
    mem: (usize, usize, usize),
    side_len: i32,
}

impl SpiralGrid {
    fn new() -> SpiralGrid {
        SpiralGrid {
            nums: vec![1, 1, 2, 4, 5, 10, 11, 23, 25],
            mem: (1, 8, 0),
            side_len: 2,
        }
    }

    fn add_first_of_layer(&mut self) {
        self.side_len += 2;
        self.mem.1 = self.nums.len() - 1;
        self.nums
            .push(self.nums[self.mem.0] + self.nums[self.mem.1]);
    }

    fn add_middle_of_side(&mut self) {
        self.mem = (self.mem.0 + 1, self.mem.0, self.mem.1);
        self.nums.push(
            self.nums[self.mem.0]
                + self.nums[self.mem.1]
                + self.nums[self.mem.2]
                + self.nums.last().unwrap(),
        );
    }

    fn add_2nd_to_last_of_side(&mut self) {
        self.nums
            .push(self.nums[self.mem.0] + self.nums[self.mem.1] + self.nums.last().unwrap());
    }

    fn add_last_of_side(&mut self) {
        self.nums
            .push(self.nums[self.mem.0] + self.nums.last().unwrap());
    }

    fn add_first_side(&mut self) {
        self.add_first_of_layer();
        for _ in 1..self.side_len - 2 {
            self.add_middle_of_side();
        }
        self.add_2nd_to_last_of_side();
        self.add_last_of_side();
    }

    fn add_first_of_side(&mut self) {
        self.mem = (self.mem.0 + 1, self.mem.0, self.nums.len() - 2);
        self.nums.push(
            self.nums[self.mem.0]
                + self.nums[self.mem.1]
                + self.nums[self.mem.2]
                + self.nums.last().unwrap(),
        );
    }

    fn add_middle_side(&mut self) {
        self.add_first_of_side();
        for _ in 1..self.side_len - 2 {
            self.add_middle_of_side();
        }
        self.add_2nd_to_last_of_side();
        self.add_last_of_side();
    }

    fn add_2nd_to_last_of_layer(&mut self) {
        self.add_middle_of_side()
    }

    fn add_last_of_layer(&mut self) {
        self.add_2nd_to_last_of_side();
    }

    fn add_last_side(&mut self) {
        self.add_first_of_side();
        for _ in 1..self.side_len - 2 {
            self.add_middle_of_side();
        }
        self.add_2nd_to_last_of_layer();
        self.add_last_of_layer();
    }

    fn add_layer(&mut self) {
        self.add_first_side();
        self.add_middle_side();
        self.add_middle_side();
        self.add_last_side();
    }
}

pub fn part2(input: &str) -> i32 {
    let input = input.parse().unwrap();
    let mut grid = SpiralGrid::new();
    while *grid.nums.last().unwrap() < input {
        grid.add_layer();
    }
    let mut res = grid.nums.pop().unwrap();
    while *grid.nums.last().unwrap() > input {
        res = grid.nums.pop().unwrap();
    }
    res
}
