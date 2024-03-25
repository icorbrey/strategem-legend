use std::fmt::Display;

use console::style;

use crate::{direction::Direction, strategems::Strategem};

#[derive(Clone, Copy)]
pub enum ChallengeStatus {
    Idle,
    PartialSuccess,
    Success,
    Failure,
}

pub struct Challenge<'a> {
    status: ChallengeStatus,
    strategem: &'a Strategem,
    index: usize,
}

impl<'a> Challenge<'a> {
    pub fn new(strategem: &'a Strategem) -> Self {
        Self {
            status: ChallengeStatus::Idle,
            strategem,
            index: 0,
        }
    }

    pub fn attempt(&mut self, direction: Direction) -> ChallengeStatus {
        if direction == self.current_direction() {
            if self.index == self.strategem.code.length() - 1 {
                self.status = ChallengeStatus::Success;
            } else {
                self.status = ChallengeStatus::PartialSuccess;
                self.index += 1;
            }
        } else {
            self.status = ChallengeStatus::Failure;
        }

        self.status
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }

    fn current_direction(&self) -> Direction {
        self.strategem.code.0[self.index]
    }
}

impl<'a> Display for Challenge<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.status {
            ChallengeStatus::Idle => write!(f, "{}", self.strategem.code),
            ChallengeStatus::PartialSuccess => {
                let (head, tail) = self.strategem.code.0.split_at(self.index);
                write!(f, "{} {}", style(assemble(head)).yellow(), assemble(tail))
            }
            ChallengeStatus::Success => write!(
                f,
                "{}",
                style(assemble(self.strategem.code.0.as_slice())).green()
            ),
            ChallengeStatus::Failure => write!(
                f,
                "{}",
                style(assemble(self.strategem.code.0.as_slice())).red()
            ),
        }
    }
}

fn assemble(directions: &[Direction]) -> String {
    directions
        .iter()
        .map(|x| format!("{}", x))
        .collect::<Vec<String>>()
        .join(" ")
}
