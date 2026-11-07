#[derive(Debug, PartialEq)]
enum KeyInput {
    SpaceDown,
    SpaceUp,
    OtherDown,
    OtherUp,
}

#[derive(Debug, PartialEq)]
enum KeyOutput {
    SpaceDown,
    SpaceUp,
    ShiftDown,
    ShiftUp,
    OtherDown,
    OtherUp,
}

#[derive(Default, Debug, PartialEq)]
enum State {
    #[default]
    Idle,
    Pending,
    ShiftActive,
}

#[derive(Default)]
struct SandS {
    state: State,
}

impl SandS {
    fn handle(&mut self, event: KeyInput) -> Vec<KeyOutput> {
        match (&self.state, event) {
            (State::Idle, KeyInput::SpaceDown) => {
                self.state = State::Pending;
                vec![]
            }
            (State::Pending | State::ShiftActive, KeyInput::SpaceDown) => vec![],
            (State::Idle, KeyInput::SpaceUp) => vec![],
            (State::Pending, KeyInput::SpaceUp) => {
                self.state = State::Idle;
                vec![KeyOutput::SpaceDown, KeyOutput::SpaceUp]
            }
            (State::ShiftActive, KeyInput::SpaceUp) => {
                self.state = State::Idle;
                vec![KeyOutput::ShiftUp]
            }
            (State::Idle | State::ShiftActive, KeyInput::OtherDown) => vec![KeyOutput::OtherDown],
            (State::Pending, KeyInput::OtherDown) => {
                self.state = State::ShiftActive;
                vec![KeyOutput::ShiftDown, KeyOutput::OtherDown]
            }
            (State::Idle | State::Pending | State::ShiftActive, KeyInput::OtherUp) => {
                vec![KeyOutput::OtherUp]
            }
        }
    }
}

fn main() {
    let mut sands = SandS::default();
    sands.handle(KeyInput::SpaceDown);
    sands.handle(KeyInput::SpaceUp);
    println!("{:?}", sands.state);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(State::Idle, KeyInput::SpaceDown, State::Pending, vec![])]
    #[case(State::Pending, KeyInput::SpaceDown, State::Pending, vec![])]
    #[case(State::ShiftActive, KeyInput::SpaceDown, State::ShiftActive, vec![])]
    #[case(State::Idle, KeyInput::SpaceUp, State::Idle, vec![])]
    #[case(State::Pending, KeyInput::SpaceUp, State::Idle, vec![KeyOutput::SpaceDown, KeyOutput::SpaceUp])]
    #[case(State::ShiftActive, KeyInput::SpaceUp, State::Idle, vec![KeyOutput::ShiftUp])]
    #[case(State::Idle, KeyInput::OtherDown, State::Idle, vec![KeyOutput::OtherDown])]
    #[case(State::Pending, KeyInput::OtherDown, State::ShiftActive,  vec![KeyOutput::ShiftDown, KeyOutput::OtherDown])]
    #[case(State::ShiftActive, KeyInput::OtherDown, State::ShiftActive,  vec![KeyOutput::OtherDown])]
    #[case(State::Idle, KeyInput::OtherUp, State::Idle,  vec![KeyOutput::OtherUp])]
    #[case(State::Pending, KeyInput::OtherUp, State::Pending,  vec![KeyOutput::OtherUp])]
    #[case(State::ShiftActive, KeyInput::OtherUp, State::ShiftActive,  vec![KeyOutput::OtherUp])]
    fn test_sands_handle(
        #[case] initial_state: State,
        #[case] event: KeyInput,
        #[case] expected_state: State,
        #[case] expected_output: Vec<KeyOutput>,
    ) {
        let mut sands = SandS {
            state: initial_state,
        };
        assert_eq!(sands.handle(event), expected_output);
        assert_eq!(sands.state, expected_state);
    }

    #[rstest]
    #[case(vec![KeyInput::SpaceDown, KeyInput::SpaceUp], State::Idle, vec![KeyOutput::SpaceDown, KeyOutput::SpaceUp])]
    #[case(vec![KeyInput::SpaceDown, KeyInput::OtherDown, KeyInput::OtherUp], State::ShiftActive, vec![KeyOutput::ShiftDown, KeyOutput::OtherDown, KeyOutput::OtherUp])]
    #[case(vec![KeyInput::SpaceDown, KeyInput::OtherDown, KeyInput::OtherUp, KeyInput::SpaceUp], State::Idle, vec![KeyOutput::ShiftDown, KeyOutput::OtherDown, KeyOutput::OtherUp, KeyOutput::ShiftUp])]
    fn test_state_transition(
        #[case] events: Vec<KeyInput>,
        #[case] expected_state: State,
        #[case] expected_output: Vec<KeyOutput>,
    ) {
        let mut sands = SandS::default();
        let outputs: Vec<KeyOutput> = events
            .into_iter()
            .flat_map(|event| sands.handle(event))
            .collect();
        assert_eq!(sands.state, expected_state);
        assert_eq!(outputs, expected_output);
    }
}
