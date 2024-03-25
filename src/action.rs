use console::Term;

use crate::{config::Config, direction::Direction};

pub enum Action {
    Cancel,
    Confirm,
    Move(Direction),
}

impl Action {
    pub fn next(term: &Term, config: &Config) -> Option<Self> {
        let key = term.read_key().expect("Should have been able to read key.");

        if key == config.keybinds.up {
            return Some(Action::Move(Direction::Up));
        }

        if key == config.keybinds.left {
            return Some(Action::Move(Direction::Left));
        }

        if key == config.keybinds.down {
            return Some(Action::Move(Direction::Down));
        }

        if key == config.keybinds.right {
            return Some(Action::Move(Direction::Right));
        }

        if key == config.keybinds.cancel {
            return Some(Action::Cancel);
        }

        if key == config.keybinds.confirm {
            return Some(Action::Confirm);
        }

        None
    }
}
