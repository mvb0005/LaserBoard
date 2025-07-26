/// USB HID keyboard protocol implementation for the Durgod K320

/// Standard USB HID keyboard report (8 bytes)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HidKeyboardReport {
    /// Modifier keys byte (Ctrl, Alt, Shift, etc.)
    pub modifiers: u8,
    /// Reserved byte (always 0x00)
    pub reserved: u8,
    /// Key codes (up to 6 simultaneous key presses)
    pub keys: [u8; 6],
}

impl HidKeyboardReport {
    /// Create a new empty keyboard report
    pub fn new() -> Self {
        Self {
            modifiers: 0,
            reserved: 0,
            keys: [0; 6],
        }
    }

    /// Create a key press report for a single key
    pub fn key_press(key: KeyCode) -> Self {
        let mut report = Self::new();
        report.keys[0] = key as u8;
        report
    }

    /// Create a key press report with modifiers
    pub fn key_press_with_modifiers(key: KeyCode, modifiers: Modifiers) -> Self {
        let mut report = Self::new();
        report.modifiers = modifiers.bits();
        report.keys[0] = key as u8;
        report
    }

    /// Create a key release report (all zeros)
    pub fn key_release() -> Self {
        Self::new()
    }

    /// Create a report with multiple keys pressed
    pub fn multi_key_press(keys: &[KeyCode], modifiers: Modifiers) -> Self {
        let mut report = Self::new();
        report.modifiers = modifiers.bits();
        
        for (i, &key) in keys.iter().enumerate().take(6) {
            report.keys[i] = key as u8;
        }
        
        report
    }

    /// Convert the report to a byte array for transmission
    pub fn to_bytes(&self) -> [u8; 8] {
        [
            self.modifiers,
            self.reserved,
            self.keys[0],
            self.keys[1],
            self.keys[2],
            self.keys[3],
            self.keys[4],
            self.keys[5],
        ]
    }

    /// Parse a byte array into a keyboard report
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, crate::DurgodError> {
        if bytes.len() != 8 {
            return Err(crate::DurgodError::InvalidReport(
                format!("Expected 8 bytes, got {}", bytes.len())
            ));
        }

        Ok(Self {
            modifiers: bytes[0],
            reserved: bytes[1],
            keys: [bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]],
        })
    }

    /// Check if this is an empty report (no keys pressed)
    pub fn is_empty(&self) -> bool {
        self.modifiers == 0 && self.keys == [0; 6]
    }

    /// Get the pressed keys as a vector
    pub fn get_pressed_keys(&self) -> Vec<KeyCode> {
        self.keys
            .iter()
            .filter(|&&key| key != 0)
            .filter_map(|&key| KeyCode::from_u8(key))
            .collect()
    }
}

impl Default for HidKeyboardReport {
    fn default() -> Self {
        Self::new()
    }
}

/// USB HID modifier keys bitmask
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Modifiers {
    bits: u8,
}

impl Modifiers {
    pub const NONE: Self = Self { bits: 0x00 };
    pub const LEFT_CTRL: Self = Self { bits: 0x01 };
    pub const LEFT_SHIFT: Self = Self { bits: 0x02 };
    pub const LEFT_ALT: Self = Self { bits: 0x04 };
    pub const LEFT_GUI: Self = Self { bits: 0x08 };  // Windows key
    pub const RIGHT_CTRL: Self = Self { bits: 0x10 };
    pub const RIGHT_SHIFT: Self = Self { bits: 0x20 };
    pub const RIGHT_ALT: Self = Self { bits: 0x40 };
    pub const RIGHT_GUI: Self = Self { bits: 0x80 };

    /// Create modifiers from a bitmask
    pub fn from_bits(bits: u8) -> Self {
        Self { bits }
    }

    /// Get the raw bitmask
    pub fn bits(self) -> u8 {
        self.bits
    }

    /// Combine multiple modifiers
    pub fn combine(modifiers: &[Self]) -> Self {
        let bits = modifiers.iter().fold(0, |acc, m| acc | m.bits);
        Self { bits }
    }

    /// Check if a specific modifier is active
    pub fn contains(self, other: Self) -> bool {
        (self.bits & other.bits) == other.bits
    }
}

impl std::ops::BitOr for Modifiers {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self { bits: self.bits | rhs.bits }
    }
}

/// USB HID keyboard scan codes
/// Based on USB HID Usage Tables specification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum KeyCode {
    // Letters
    A = 0x04,
    B = 0x05,
    C = 0x06,
    D = 0x07,
    E = 0x08,
    F = 0x09,
    G = 0x0A,
    H = 0x0B,
    I = 0x0C,
    J = 0x0D,
    K = 0x0E,
    L = 0x0F,
    M = 0x10,
    N = 0x11,
    O = 0x12,
    P = 0x13,
    Q = 0x14,
    R = 0x15,
    S = 0x16,
    T = 0x17,
    U = 0x18,
    V = 0x19,
    W = 0x1A,
    X = 0x1B,
    Y = 0x1C,
    Z = 0x1D,

    // Numbers
    Key1 = 0x1E,
    Key2 = 0x1F,
    Key3 = 0x20,
    Key4 = 0x21,
    Key5 = 0x22,
    Key6 = 0x23,
    Key7 = 0x24,
    Key8 = 0x25,
    Key9 = 0x26,
    Key0 = 0x27,

    // Special keys
    Enter = 0x28,
    Escape = 0x29,
    Backspace = 0x2A,
    Tab = 0x2B,
    Space = 0x2C,

    // Symbols
    Minus = 0x2D,        // -
    Equal = 0x2E,        // =
    LeftBracket = 0x2F,  // [
    RightBracket = 0x30, // ]
    Backslash = 0x31,    // \
    Semicolon = 0x33,    // ;
    Apostrophe = 0x34,   // '
    GraveAccent = 0x35,  // `
    Comma = 0x36,        // ,
    Period = 0x37,       // .
    Slash = 0x38,        // /

    // Function keys
    F1 = 0x3A,
    F2 = 0x3B,
    F3 = 0x3C,
    F4 = 0x3D,
    F5 = 0x3E,
    F6 = 0x3F,
    F7 = 0x40,
    F8 = 0x41,
    F9 = 0x42,
    F10 = 0x43,
    F11 = 0x44,
    F12 = 0x45,

    // Navigation
    Insert = 0x49,
    Home = 0x4A,
    PageUp = 0x4B,
    Delete = 0x4C,
    End = 0x4D,
    PageDown = 0x4E,
    RightArrow = 0x4F,
    LeftArrow = 0x50,
    DownArrow = 0x51,
    UpArrow = 0x52,

    // Numpad
    NumLock = 0x53,
    NumpadDivide = 0x54,
    NumpadMultiply = 0x55,
    NumpadMinus = 0x56,
    NumpadPlus = 0x57,
    NumpadEnter = 0x58,
    Numpad1 = 0x59,
    Numpad2 = 0x5A,
    Numpad3 = 0x5B,
    Numpad4 = 0x5C,
    Numpad5 = 0x5D,
    Numpad6 = 0x5E,
    Numpad7 = 0x5F,
    Numpad8 = 0x60,
    Numpad9 = 0x61,
    Numpad0 = 0x62,
    NumpadPeriod = 0x63,
}

impl KeyCode {
    /// Convert a u8 value to a KeyCode if valid
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x04 => Some(KeyCode::A),
            0x05 => Some(KeyCode::B),
            0x06 => Some(KeyCode::C),
            0x07 => Some(KeyCode::D),
            0x08 => Some(KeyCode::E),
            0x09 => Some(KeyCode::F),
            0x0A => Some(KeyCode::G),
            0x0B => Some(KeyCode::H),
            0x0C => Some(KeyCode::I),
            0x0D => Some(KeyCode::J),
            0x0E => Some(KeyCode::K),
            0x0F => Some(KeyCode::L),
            0x10 => Some(KeyCode::M),
            0x11 => Some(KeyCode::N),
            0x12 => Some(KeyCode::O),
            0x13 => Some(KeyCode::P),
            0x14 => Some(KeyCode::Q),
            0x15 => Some(KeyCode::R),
            0x16 => Some(KeyCode::S),
            0x17 => Some(KeyCode::T),
            0x18 => Some(KeyCode::U),
            0x19 => Some(KeyCode::V),
            0x1A => Some(KeyCode::W),
            0x1B => Some(KeyCode::X),
            0x1C => Some(KeyCode::Y),
            0x1D => Some(KeyCode::Z),
            0x1E => Some(KeyCode::Key1),
            0x1F => Some(KeyCode::Key2),
            0x20 => Some(KeyCode::Key3),
            0x21 => Some(KeyCode::Key4),
            0x22 => Some(KeyCode::Key5),
            0x23 => Some(KeyCode::Key6),
            0x24 => Some(KeyCode::Key7),
            0x25 => Some(KeyCode::Key8),
            0x26 => Some(KeyCode::Key9),
            0x27 => Some(KeyCode::Key0),
            0x28 => Some(KeyCode::Enter),
            0x29 => Some(KeyCode::Escape),
            0x2A => Some(KeyCode::Backspace),
            0x2B => Some(KeyCode::Tab),
            0x2C => Some(KeyCode::Space),
            0x3A => Some(KeyCode::F1),
            0x3B => Some(KeyCode::F2),
            0x3C => Some(KeyCode::F3),
            0x3D => Some(KeyCode::F4),
            0x3E => Some(KeyCode::F5),
            0x3F => Some(KeyCode::F6),
            0x40 => Some(KeyCode::F7),
            0x41 => Some(KeyCode::F8),
            0x42 => Some(KeyCode::F9),
            0x43 => Some(KeyCode::F10),
            0x44 => Some(KeyCode::F11),
            0x45 => Some(KeyCode::F12),
            _ => None,
        }
    }

    /// Get the character representation of this key code (if applicable)
    pub fn to_char(self) -> Option<char> {
        match self {
            KeyCode::A => Some('a'),
            KeyCode::B => Some('b'),
            KeyCode::C => Some('c'),
            KeyCode::D => Some('d'),
            KeyCode::E => Some('e'),
            KeyCode::F => Some('f'),
            KeyCode::G => Some('g'),
            KeyCode::H => Some('h'),
            KeyCode::I => Some('i'),
            KeyCode::J => Some('j'),
            KeyCode::K => Some('k'),
            KeyCode::L => Some('l'),
            KeyCode::M => Some('m'),
            KeyCode::N => Some('n'),
            KeyCode::O => Some('o'),
            KeyCode::P => Some('p'),
            KeyCode::Q => Some('q'),
            KeyCode::R => Some('r'),
            KeyCode::S => Some('s'),
            KeyCode::T => Some('t'),
            KeyCode::U => Some('u'),
            KeyCode::V => Some('v'),
            KeyCode::W => Some('w'),
            KeyCode::X => Some('x'),
            KeyCode::Y => Some('y'),
            KeyCode::Z => Some('z'),
            KeyCode::Key1 => Some('1'),
            KeyCode::Key2 => Some('2'),
            KeyCode::Key3 => Some('3'),
            KeyCode::Key4 => Some('4'),
            KeyCode::Key5 => Some('5'),
            KeyCode::Key6 => Some('6'),
            KeyCode::Key7 => Some('7'),
            KeyCode::Key8 => Some('8'),
            KeyCode::Key9 => Some('9'),
            KeyCode::Key0 => Some('0'),
            KeyCode::Space => Some(' '),
            _ => None,
        }
    }

    /// Convert a character to a key code (if possible)
    pub fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_lowercase() {
            'a' => Some(KeyCode::A),
            'b' => Some(KeyCode::B),
            'c' => Some(KeyCode::C),
            'd' => Some(KeyCode::D),
            'e' => Some(KeyCode::E),
            'f' => Some(KeyCode::F),
            'g' => Some(KeyCode::G),
            'h' => Some(KeyCode::H),
            'i' => Some(KeyCode::I),
            'j' => Some(KeyCode::J),
            'k' => Some(KeyCode::K),
            'l' => Some(KeyCode::L),
            'm' => Some(KeyCode::M),
            'n' => Some(KeyCode::N),
            'o' => Some(KeyCode::O),
            'p' => Some(KeyCode::P),
            'q' => Some(KeyCode::Q),
            'r' => Some(KeyCode::R),
            's' => Some(KeyCode::S),
            't' => Some(KeyCode::T),
            'u' => Some(KeyCode::U),
            'v' => Some(KeyCode::V),
            'w' => Some(KeyCode::W),
            'x' => Some(KeyCode::X),
            'y' => Some(KeyCode::Y),
            'z' => Some(KeyCode::Z),
            '1' => Some(KeyCode::Key1),
            '2' => Some(KeyCode::Key2),
            '3' => Some(KeyCode::Key3),
            '4' => Some(KeyCode::Key4),
            '5' => Some(KeyCode::Key5),
            '6' => Some(KeyCode::Key6),
            '7' => Some(KeyCode::Key7),
            '8' => Some(KeyCode::Key8),
            '9' => Some(KeyCode::Key9),
            '0' => Some(KeyCode::Key0),
            ' ' => Some(KeyCode::Space),
            _ => None,
        }
    }
} 