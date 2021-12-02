use crate::submarine::*;

pub fn compute(commands: impl Iterator<Item = Command>) -> Location {
    let mut position = 0u32;
    let mut depth = 0u32;

    for command in commands {
        match command {
            Command::Forward(value) => position += value,
            Command::Up(value) => depth -= value,
            Command::Down(value) => depth += value,
        }
    }

    Location { position, depth }
}
