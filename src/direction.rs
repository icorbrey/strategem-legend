use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    pub fn from_char(value: char) -> Result<Direction, char> {
        match value {
            'U' => Ok(Direction::Up),
            'L' => Ok(Direction::Left),
            'D' => Ok(Direction::Down),
            'R' => Ok(Direction::Right),
            ch => Err(ch),
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
