use crate::submarine::*;

#[derive(Default)]
struct State {
    position: u32,
    depth: u32,
    pitch: u32,
}

pub fn compute(commands: impl Iterator<Item = Command>) -> Location {
    let result = commands.fold(State::default(), |mut state, command| {
        match command {
            Command::Forward(value) => {
                state.position += value;
                state.depth += value * state.pitch;
            }
            Command::Up(value) => state.pitch -= value,
            Command::Down(value) => state.pitch += value,
        };

        state
    });

    Location {
        position: result.position,
        depth: result.depth,
    }
}
