struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    length: u32,
}

impl std::str::FromStr for Claim {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(" @ ");
        let id = {
            let pref_id = iter.next().unwrap();
            pref_id.split('#').nth(1).unwrap().parse().unwrap()
        };
        iter = iter.next().unwrap().split(": ");
        let (x, y) = {
            let mut x_and_y = iter.next().unwrap().split(',');
            (
                x_and_y.next().unwrap().parse().unwrap(),
                x_and_y.next().unwrap().parse().unwrap(),
            )
        };
        let (width, length) = {
            let mut w_and_l = iter.next().unwrap().split('x');
            (
                w_and_l.next().unwrap().parse().unwrap(),
                w_and_l.next().unwrap().parse().unwrap(),
            )
        };
        Ok(Claim {
            id,
            x,
            y,
            width,
            length,
        })
    }
}

type Cloth = Vec<Vec<u32>>;

impl Claim {
    fn cut(&self, cloth: &mut Cloth) {
        for i in self.x..self.x + self.width {
            for j in self.y..self.y + self.length {
                cloth[i as usize][j as usize] += 1;
            }
        }
    }

    fn has_overlap(&self, cloth: &Cloth) -> bool {
        for i in self.x..self.x + self.width {
            for j in self.y..self.y + self.length {
                if cloth[i as usize][j as usize] > 1 {
                    return true;
                }
            }
        }
        false
    }
}

pub fn part1(input: &str) -> usize {
    let claims: Vec<Claim> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut cloth: Cloth = vec![vec![0; 1000]; 1000];
    for claim in claims {
        claim.cut(&mut cloth);
    }
    cloth.into_iter().flatten().filter(|&x| x > 1).count()
}

pub fn part2(input: &str) -> u32 {
    let claims: Vec<Claim> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut cloth: Cloth = vec![vec![0; 1000]; 1000];
    for claim in claims.iter() {
        claim.cut(&mut cloth);
    }
    for claim in claims {
        if !claim.has_overlap(&cloth) {
            return claim.id;
        }
    }
    panic!()
}
