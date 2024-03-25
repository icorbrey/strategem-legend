use std::fmt::Display;

use crate::direction::Direction;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum CodeError {
    #[error("Strategem code contains invalid characters: `{0}`")]
    InvalidDirections(String),
}

#[derive(Debug, Clone)]
pub struct Code(pub Vec<Direction>);

impl Code {
    pub fn from_string(code: String) -> Result<Self, CodeError> {
        let directions: Vec<Result<Direction, char>> =
            code.chars().into_iter().map(Direction::from_char).collect();

        if !(&directions).into_iter().any(|d| d.is_err()) {
            Ok(Code(
                directions.into_iter().filter_map(|d| d.ok()).collect(),
            ))
        } else {
            Err(CodeError::InvalidDirections(
                (directions.iter())
                    .filter_map(|d| d.as_ref().err())
                    .collect(),
            ))
        }
    }

    pub fn length(&self) -> usize {
        self.0.len()
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let directions = (self.0.iter())
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "{}", directions)
    }
}
