use std::str::FromStr;

pub struct Location {
    pub position: u32,
    pub depth: u32,
}

#[derive(Debug)]
pub enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (instruction, value) = input.split_once(" ").ok_or("invalid format")?;
        let value_int = value.parse::<u32>().map_err(|_| "invalid value")?;

        Ok(match instruction {
            "forward" => Command::Forward(value_int),
            "up" => Command::Up(value_int),
            "down" => Command::Down(value_int),
            _ => return Err("invalid command"),
        })
    }
}
