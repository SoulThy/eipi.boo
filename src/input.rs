#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Browse,
    Compose,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyEvent {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Tab,
    Backspace,
    Escape,
    Char(char),
}

pub fn parse(data: &[u8]) -> Vec<KeyEvent> {
    let mut events = Vec::new();
    let mut i = 0;

    while i < data.len() {
        match data[i] {
            0x1b => {
                if i + 2 < data.len() && data[i + 1] == b'[' {
                    match data[i + 2] {
                        b'A' => events.push(KeyEvent::Up),
                        b'B' => events.push(KeyEvent::Down),
                        b'C' => events.push(KeyEvent::Right),
                        b'D' => events.push(KeyEvent::Left),
                        b'Z' => events.push(KeyEvent::Tab),
                        _ => {}
                    }
                    i += 3;
                } else {
                    events.push(KeyEvent::Escape);
                    i += 1;
                }
            }
            0x0d | 0x0a => {
                events.push(KeyEvent::Enter);
                i += 1;
            }
            0x09 => {
                events.push(KeyEvent::Tab);
                i += 1;
            }
            0x7f | 0x08 => {
                events.push(KeyEvent::Backspace);
                i += 1;
            }
            0x03 => {
                events.push(KeyEvent::Char('q'));
                i += 1;
            }
            b if b >= 0x20 && b < 0x7f => {
                events.push(KeyEvent::Char(b as char));
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    events
}
