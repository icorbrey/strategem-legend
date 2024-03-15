use config::Config;
use console::Term;

pub mod code;
pub mod config;
pub mod direction;
pub mod keybinds;
pub mod strategems;

fn main() {
    let config = Config::load_from_yaml("./config.yaml")
        .expect("Should have been able to read `config.yaml`");

    let term = Term::stdout();

    'main: loop {
        let action =
            Action::get_next(&term, config).expect("Should have been able to get next action.");
    }
}
