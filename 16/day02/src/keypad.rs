mod normal_keypad;
use normal_keypad::NormalKeypad;

mod fancy_keypad;
use fancy_keypad::FancyKeypad;

use crate::direction::Direction;

pub trait Keypad {
    fn change_pos(&mut self, direction: Direction);

    fn get_pos(&self) -> u8;

    fn get_base(&self) -> u32;
}

pub enum Model {
    Normal,
    Fancy,
}
use Model::*;

pub fn new(model: Model) -> Box<dyn Keypad> {
    match model {
        Normal => Box::new(NormalKeypad::default()),
        Fancy => Box::new(FancyKeypad::default()),
    }
}
