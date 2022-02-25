mod direction;
mod finger_pos;
use finger_pos::FingerPosition;

pub fn part1(input: &str) -> u32 {
    let mut res = 0;
    let mut finger_pos = FingerPosition::default();
    for line in input.lines() {
        for substr in line.split("").filter(|s| !s.is_empty()) {
            let direction = substr.parse().unwrap();
            finger_pos.change_pos(direction);
        }
        res *= 10;
        res += finger_pos.get_pos() as u32;
    }
    res
}
