pub struct Triangle(Vec<u32>);

impl Triangle {
    pub fn new(mut vals: Vec<u32>) -> Triangle {
        vals.sort_unstable();
        Triangle(vals)
    }

    pub fn from_tuple(t: (u32, u32, u32)) -> Triangle {
        let mut vals = vec![t.0, t.1, t.2];
        vals.sort_unstable();
        Triangle(vals)
    }

    pub fn is_possible(&self) -> bool {
        (self.0[0] + self.0[1]) > self.0[2]
    }
}

impl std::str::FromStr for Triangle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals = s
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect::<Vec<_>>();
        if vals.len() != 3 {
            return Err(format!(
                "number of values was unequal to three, namely {}",
                vals.len()
            ));
        }
        Ok(Triangle::new(vals))
    }
}
