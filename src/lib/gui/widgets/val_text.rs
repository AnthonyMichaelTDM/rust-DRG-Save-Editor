use std::{fmt::Display, str::FromStr};

use eframe::egui::TextBuffer;

/// A mutable TextBuffer that will validate it's contents when changed.
///
/// The default validator will simply attempt to parse the text as `T`,
/// but a custom validator function can be provided.
pub struct ValText<T> {
    text: String,
    val: Option<T>,
    #[allow(clippy::type_complexity)]
    validator: Box<dyn Fn(&str) -> Option<T>>,
}

impl<T> ValText<T> {
    #[allow(dead_code)]
    pub fn with_validator(validator: impl Fn(&str) -> Option<T> + 'static) -> Self {
        Self {
            text: Default::default(),
            val: Default::default(),
            validator: Box::new(validator),
        }
    }

    pub fn get_val(&self) -> Option<&T> {
        self.val.as_ref()
    }

    pub fn is_valid(&self) -> bool {
        self.val.is_some()
    }
}

impl<T: Display> ValText<T> {
    #[allow(dead_code)]
    pub fn set_val(&mut self, val: T) {
        self.text = val.to_string();
        self.val = Some(val);
    }
}

impl<T: FromStr> Default for ValText<T> {
    fn default() -> Self {
        Self {
            text: Default::default(),
            val: Default::default(),
            validator: Box::new(|text| text.parse::<T>().ok()),
        }
    }
}

impl<T> TextBuffer for ValText<T> {
    fn is_mutable(&self) -> bool {
        true
    }

    fn as_str(&self) -> &str {
        self.text.as_str()
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        let n = self.text.insert_text(text, char_index);
        self.val = (self.validator)(&self.text);
        n
    }

    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        self.text.delete_char_range(char_range);
        self.val = (self.validator)(&self.text);
    }
}


#[cfg(test)]
mod tests {
    use egui::TextBuffer;

    use super::ValText;

    //unit tests
    #[test]
    fn test_valid_input() {
        let mut validator = ValText::<usize>::default();
        validator.insert_text("10", 0);
        assert!(validator.is_valid());
        assert!(validator.val == Some(10));
    }
    #[test]
    fn test_invalid_numeric_input() {
        let mut validator = ValText::<usize>::default();
        validator.insert_text("-10", 0);
        assert!(!validator.is_valid());
        assert!(validator.val == None);
    }
    
    #[test]
    fn test_invalid_text_input() {
        let mut validator = ValText::<usize>::default();
        validator.insert_text("I'm not a number", 0);
        assert!(!validator.is_valid());
        assert!(validator.val == None);
    }
}
