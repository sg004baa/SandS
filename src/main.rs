enum KeyEvent {
    SpaceDown,
    SpaceUp,
    OtherDown,
    OtherUp,
}

struct SandS {
    space_pressed: bool,
}

impl SandS {
    fn new() -> Self {
        Self {
            space_pressed: false,
        }
    }

    fn handle(&mut self, event: KeyEvent) {
        match event {
            KeyEvent::SpaceDown => self.space_pressed = true,
            KeyEvent::SpaceUp => self.space_pressed = false,
            _ => {}
        }
    }
}

fn main() {
    let mut sands = SandS::new();
    sands.handle(KeyEvent::SpaceDown);
    sands.handle(KeyEvent::OtherUp);
    println!("{:?}", sands.space_pressed);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(false, KeyEvent::SpaceDown, true)]
    #[case(true, KeyEvent::SpaceDown, true)]
    #[case(false, KeyEvent::SpaceUp, false)]
    #[case(true, KeyEvent::SpaceUp, false)]
    #[case(false, KeyEvent::OtherDown, false)]
    #[case(true, KeyEvent::OtherDown, true)]
    #[case(false, KeyEvent::OtherUp, false)]
    #[case(true, KeyEvent::OtherUp, true)]
    fn test_sands_handle(#[case] prev_cond: bool, #[case] event: KeyEvent, #[case] expected: bool) {
        let mut sands = SandS {
            space_pressed: prev_cond,
        };
        sands.handle(event);
        let actual = sands.space_pressed;
        assert_eq!(actual, expected)
    }
}
