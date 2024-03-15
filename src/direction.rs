use std::fmt::Display;

pub enum DirectionError {
    InvalidDirection(char),
}

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    pub fn from_char(value: char) -> Result<Direction, DirectionError> {
        match value {
            'U' => Ok(Direction::Up),
            'L' => Ok(Direction::Left),
            'D' => Ok(Direction::Down),
            'R' => Ok(Direction::Right),
            ch => Err(DirectionError::InvalidDirection(ch)),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Up => "↑",
                Self::Left => "←",
                Self::Down => "↓",
                Self::Right => "→",
            }
        )
    }
}
