use action::Action;
use challenge::{Challenge, ChallengeStatus};
use config::Config;
use console::Term;
use std::io::Result;

pub mod action;
pub mod challenge;
pub mod code;
pub mod config;
pub mod direction;
pub mod keybinds;
pub mod strategems;

fn main() -> Result<()> {
    let config = Config::load_from_yaml("./config.yaml")
        .expect("Should have been able to read `config.yaml`");

    let term = Term::stdout();

    term.hide_cursor()?;
    term.set_title("Strategem Legend");

    'main: for strategem in &config.strategems {
        let mut challenge = Challenge::new(strategem);

        println!("{}", strategem.name);
        println!("{}", challenge);

        'inner: loop {
            match Action::next(&term, &config) {
                Some(Action::Move(dir)) => {
                    let status = challenge.attempt(dir);

                    term.move_cursor_up(1)?;
                    term.clear_line()?;
                    println!("{}", challenge);

                    match status {
                        ChallengeStatus::Success => break 'inner,
                        ChallengeStatus::Failure => challenge.reset(),
                        _ => {}
                    }
                }
                Some(Action::Cancel) => break 'main,
                Some(Action::Confirm) => {}
                None => {}
            }
        }

        println!("Press any key to continue");

        Action::next(&term, &config);

        term.move_cursor_up(1)?;
        term.clear_line()?;
        term.move_cursor_up(1)?;
        term.clear_line()?;
        term.move_cursor_up(1)?;
        term.clear_line()?;
    }

    term.show_cursor()?;

    Ok(())
}
