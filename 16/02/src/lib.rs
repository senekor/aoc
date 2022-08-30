mod direction;
mod keypad;

fn find_bathroom_code(input: &str, keypad_model: keypad::Model) -> u32 {
    let mut res = 0;
    let mut keypad = keypad::new(keypad_model);
    for line in input.lines() {
        line.split("")
            .filter_map(|s| s.parse().ok())
            .for_each(|direction| keypad.change_pos(direction));
        res *= keypad.get_base();
        res += keypad.get_pos() as u32;
    }
    res
}

pub fn part1(input: &str) -> u32 {
    find_bathroom_code(input, keypad::Model::Normal)
}

pub fn part2(input: &str) -> u32 {
    find_bathroom_code(input, keypad::Model::Fancy)
}
