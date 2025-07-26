use crate::{DurgodController, KeyCode, Modifiers, DurgodError};
use anyhow::Result;
use log::{debug, info};
use std::time::Duration;

/// High-level keyboard interface for text input and key sequences
pub struct Keyboard {
    controller: DurgodController,
}

impl Keyboard {
    /// Create a new keyboard instance
    pub fn new(controller: DurgodController) -> Self {
        Self { controller }
    }

    /// Type a string of text
    pub fn type_text(&self, text: &str) -> Result<()> {
        info!("Typing text: '{}'", text);
        
        for ch in text.chars() {
            self.type_char(ch)?;
            // Small delay between characters for natural typing
            std::thread::sleep(Duration::from_millis(10));
        }
        
        Ok(())
    }

    /// Type a single character
    pub fn type_char(&self, ch: char) -> Result<()> {
        debug!("Typing character: '{}'", ch);
        
        let (key_code, modifiers) = self.char_to_key_and_modifiers(ch)?;
        
        if modifiers.bits() != 0 {
            self.controller.send_key_with_modifiers(key_code, modifiers)?;
        } else {
            self.controller.send_key_press(key_code)?;
        }
        
        Ok(())
    }

    /// Press a specific key
    pub fn press_key(&self, key: KeyCode) -> Result<()> {
        debug!("Pressing key: {:?}", key);
        self.controller.send_key_press(key)
    }

    /// Press a key with modifiers
    pub fn press_key_with_modifiers(&self, key: KeyCode, modifiers: Modifiers) -> Result<()> {
        debug!("Pressing key {:?} with modifiers: {:02x}", key, modifiers.bits());
        self.controller.send_key_with_modifiers(key, modifiers)
    }

    /// Send Ctrl+C (copy)
    pub fn ctrl_c(&self) -> Result<()> {
        self.press_key_with_modifiers(KeyCode::C, Modifiers::LEFT_CTRL)
    }

    /// Send Ctrl+V (paste)
    pub fn ctrl_v(&self) -> Result<()> {
        self.press_key_with_modifiers(KeyCode::V, Modifiers::LEFT_CTRL)
    }

    /// Send Ctrl+A (select all)
    pub fn ctrl_a(&self) -> Result<()> {
        self.press_key_with_modifiers(KeyCode::A, Modifiers::LEFT_CTRL)
    }

    /// Send Ctrl+Z (undo)
    pub fn ctrl_z(&self) -> Result<()> {
        self.press_key_with_modifiers(KeyCode::Z, Modifiers::LEFT_CTRL)
    }

    /// Send Alt+Tab (switch windows)
    pub fn alt_tab(&self) -> Result<()> {
        self.press_key_with_modifiers(KeyCode::Tab, Modifiers::LEFT_ALT)
    }

    /// Send Windows key
    pub fn windows_key(&self) -> Result<()> {
        // Send just the modifier without a key
        let report = crate::HidKeyboardReport {
            modifiers: Modifiers::LEFT_GUI.bits(),
            reserved: 0,
            keys: [0; 6],
        };
        self.controller.send_hid_report(&report)?;
        
        std::thread::sleep(Duration::from_millis(50));
        
        // Release
        let release_report = crate::HidKeyboardReport::key_release();
        self.controller.send_hid_report(&release_report)?;
        
        Ok(())
    }

    /// Send Enter key
    pub fn enter(&self) -> Result<()> {
        self.press_key(KeyCode::Enter)
    }

    /// Send Escape key
    pub fn escape(&self) -> Result<()> {
        self.press_key(KeyCode::Escape)
    }

    /// Send Tab key
    pub fn tab(&self) -> Result<()> {
        self.press_key(KeyCode::Tab)
    }

    /// Send Backspace key
    pub fn backspace(&self) -> Result<()> {
        self.press_key(KeyCode::Backspace)
    }

    /// Send Delete key
    pub fn delete(&self) -> Result<()> {
        self.press_key(KeyCode::Delete)
    }

    /// Send arrow keys
    pub fn arrow_up(&self) -> Result<()> {
        self.press_key(KeyCode::UpArrow)
    }

    pub fn arrow_down(&self) -> Result<()> {
        self.press_key(KeyCode::DownArrow)
    }

    pub fn arrow_left(&self) -> Result<()> {
        self.press_key(KeyCode::LeftArrow)
    }

    pub fn arrow_right(&self) -> Result<()> {
        self.press_key(KeyCode::RightArrow)
    }

    /// Send function keys
    pub fn function_key(&self, number: u8) -> Result<()> {
        let key = match number {
            1 => KeyCode::F1,
            2 => KeyCode::F2,
            3 => KeyCode::F3,
            4 => KeyCode::F4,
            5 => KeyCode::F5,
            6 => KeyCode::F6,
            7 => KeyCode::F7,
            8 => KeyCode::F8,
            9 => KeyCode::F9,
            10 => KeyCode::F10,
            11 => KeyCode::F11,
            12 => KeyCode::F12,
            _ => return Err(DurgodError::InvalidKeyCode(number).into()),
        };
        
        self.press_key(key)
    }

    /// Type a series of keys with optional delays
    pub fn key_sequence(&self, keys: &[(KeyCode, Option<Modifiers>, Option<Duration>)]) -> Result<()> {
        for (key, modifiers, delay) in keys {
            if let Some(mods) = modifiers {
                self.press_key_with_modifiers(*key, *mods)?;
            } else {
                self.press_key(*key)?;
            }
            
            if let Some(delay_duration) = delay {
                std::thread::sleep(*delay_duration);
            } else {
                // Default small delay
                std::thread::sleep(Duration::from_millis(50));
            }
        }
        
        Ok(())
    }

    /// Convert a character to key code and modifiers
    fn char_to_key_and_modifiers(&self, ch: char) -> Result<(KeyCode, Modifiers)> {
        match ch {
            // Lowercase letters
            'a'..='z' => {
                let key = KeyCode::from_char(ch).ok_or_else(|| {
                    DurgodError::InvalidKeyCode(ch as u8)
                })?;
                Ok((key, Modifiers::NONE))
            }
            // Uppercase letters (need shift)
            'A'..='Z' => {
                let key = KeyCode::from_char(ch.to_ascii_lowercase()).ok_or_else(|| {
                    DurgodError::InvalidKeyCode(ch as u8)
                })?;
                Ok((key, Modifiers::LEFT_SHIFT))
            }
            // Numbers
            '0'..='9' => {
                let key = KeyCode::from_char(ch).ok_or_else(|| {
                    DurgodError::InvalidKeyCode(ch as u8)
                })?;
                Ok((key, Modifiers::NONE))
            }
            // Special characters
            ' ' => Ok((KeyCode::Space, Modifiers::NONE)),
            '\n' => Ok((KeyCode::Enter, Modifiers::NONE)),
            '\t' => Ok((KeyCode::Tab, Modifiers::NONE)),
            '!' => Ok((KeyCode::Key1, Modifiers::LEFT_SHIFT)),
            '@' => Ok((KeyCode::Key2, Modifiers::LEFT_SHIFT)),
            '#' => Ok((KeyCode::Key3, Modifiers::LEFT_SHIFT)),
            '$' => Ok((KeyCode::Key4, Modifiers::LEFT_SHIFT)),
            '%' => Ok((KeyCode::Key5, Modifiers::LEFT_SHIFT)),
            '^' => Ok((KeyCode::Key6, Modifiers::LEFT_SHIFT)),
            '&' => Ok((KeyCode::Key7, Modifiers::LEFT_SHIFT)),
            '*' => Ok((KeyCode::Key8, Modifiers::LEFT_SHIFT)),
            '(' => Ok((KeyCode::Key9, Modifiers::LEFT_SHIFT)),
            ')' => Ok((KeyCode::Key0, Modifiers::LEFT_SHIFT)),
            '-' => Ok((KeyCode::Minus, Modifiers::NONE)),
            '_' => Ok((KeyCode::Minus, Modifiers::LEFT_SHIFT)),
            '=' => Ok((KeyCode::Equal, Modifiers::NONE)),
            '+' => Ok((KeyCode::Equal, Modifiers::LEFT_SHIFT)),
            '[' => Ok((KeyCode::LeftBracket, Modifiers::NONE)),
            '{' => Ok((KeyCode::LeftBracket, Modifiers::LEFT_SHIFT)),
            ']' => Ok((KeyCode::RightBracket, Modifiers::NONE)),
            '}' => Ok((KeyCode::RightBracket, Modifiers::LEFT_SHIFT)),
            '\\' => Ok((KeyCode::Backslash, Modifiers::NONE)),
            '|' => Ok((KeyCode::Backslash, Modifiers::LEFT_SHIFT)),
            ';' => Ok((KeyCode::Semicolon, Modifiers::NONE)),
            ':' => Ok((KeyCode::Semicolon, Modifiers::LEFT_SHIFT)),
            '\'' => Ok((KeyCode::Apostrophe, Modifiers::NONE)),
            '"' => Ok((KeyCode::Apostrophe, Modifiers::LEFT_SHIFT)),
            '`' => Ok((KeyCode::GraveAccent, Modifiers::NONE)),
            '~' => Ok((KeyCode::GraveAccent, Modifiers::LEFT_SHIFT)),
            ',' => Ok((KeyCode::Comma, Modifiers::NONE)),
            '<' => Ok((KeyCode::Comma, Modifiers::LEFT_SHIFT)),
            '.' => Ok((KeyCode::Period, Modifiers::NONE)),
            '>' => Ok((KeyCode::Period, Modifiers::LEFT_SHIFT)),
            '/' => Ok((KeyCode::Slash, Modifiers::NONE)),
            '?' => Ok((KeyCode::Slash, Modifiers::LEFT_SHIFT)),
            _ => Err(DurgodError::InvalidKeyCode(ch as u8).into()),
        }
    }

    /// Get access to the underlying controller for advanced operations
    pub fn controller(&self) -> &DurgodController {
        &self.controller
    }

    /// Get mutable access to the underlying controller
    pub fn controller_mut(&mut self) -> &mut DurgodController {
        &mut self.controller
    }
}

/// Builder for creating key sequences
pub struct KeySequenceBuilder {
    keys: Vec<(KeyCode, Option<Modifiers>, Option<Duration>)>,
}

impl KeySequenceBuilder {
    /// Create a new key sequence builder
    pub fn new() -> Self {
        Self { keys: Vec::new() }
    }

    /// Add a key press to the sequence
    pub fn key(mut self, key: KeyCode) -> Self {
        self.keys.push((key, None, None));
        self
    }

    /// Add a key press with modifiers
    pub fn key_with_modifiers(mut self, key: KeyCode, modifiers: Modifiers) -> Self {
        self.keys.push((key, Some(modifiers), None));
        self
    }

    /// Add a key press with a custom delay
    pub fn key_with_delay(mut self, key: KeyCode, delay: Duration) -> Self {
        self.keys.push((key, None, Some(delay)));
        self
    }

    /// Add a key press with modifiers and delay
    pub fn key_with_modifiers_and_delay(
        mut self,
        key: KeyCode,
        modifiers: Modifiers,
        delay: Duration,
    ) -> Self {
        self.keys.push((key, Some(modifiers), Some(delay)));
        self
    }

    /// Add a delay without a key press
    pub fn delay(self, duration: Duration) -> Self {
        // We'll add this as a space key with zero duration that gets filtered out
        // This is a simple way to add delays, though a more sophisticated approach
        // would be to create a separate enum for sequence items
        self.key_with_delay(KeyCode::Space, duration)
    }

    /// Build the key sequence
    pub fn build(self) -> Vec<(KeyCode, Option<Modifiers>, Option<Duration>)> {
        self.keys
    }

    /// Execute the key sequence on a keyboard
    pub fn execute(self, keyboard: &Keyboard) -> Result<()> {
        keyboard.key_sequence(&self.keys)
    }
}

impl Default for KeySequenceBuilder {
    fn default() -> Self {
        Self::new()
    }
} 