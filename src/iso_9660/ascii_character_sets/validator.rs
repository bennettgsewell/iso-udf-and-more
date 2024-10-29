use std::str::Chars;

use super::{CharacterSets, ValidateCharFn};

/// # Iterator that validates char collection
pub struct CharacterValidator<'a> {
    characters: Chars<'a>,
    converter: ValidateCharFn
}

impl CharacterValidator<'_> {
    /// # Iterator that validates char collection
    /// 
    /// Iterates over every `char` in a `str`
    /// 
    /// # Returns
    /// 
    /// `Some` or `None` if it's valid/invalid
    pub fn new(input: &str, character_set: CharacterSets) -> CharacterValidator {
        let converter = super::get_validator_fn_for(character_set);

        CharacterValidator {
            characters: input.chars(),
            converter: converter
        }
    }
}

impl Iterator for CharacterValidator<'_> {
    type Item = Result<u8, char>;

    fn next(&mut self) -> Option<Self::Item> {
        // There is a next char or return None
        let next_char = self.characters.next()?;

        // Convert the char, when fail return Some(Err(reason why));
        Some(match (self.converter)(next_char) {
            // Valid char, return the u8 it represents in ASCII
            Some(converted_char) => Ok(converted_char),
            // Invalid char, return the char in the Err()
            None => Err(next_char)
        })
    }
}

#[cfg(test)]
mod character_validator_iterator_tests {
    use super::CharacterSets;

    use super::*;

    #[test]
    pub fn iterate_hello_world_str_into_ascii_bytes() {
        // Arrange
        const HELLO_WORLD: &str = "HELLO_WORLD";

        let converter = CharacterValidator::new(HELLO_WORLD, CharacterSets::DCharacters);

        // Act
        let result: Vec<u8> = converter
        .map(|x| x.expect("Failed to convert character"))
        .collect();

        // Assert
        let expected_result: Vec<u8> = HELLO_WORLD.as_bytes().to_vec();
        assert_eq!(expected_result, result);
    }

    #[test]
    pub fn iterate_invalid_str_returns_none() {
        // Arrange
        const HELLO_WORLD: &str = "HELLO WORLD";

        let mut converter = CharacterValidator::new(HELLO_WORLD, CharacterSets::DCharacters);

        // Act & Assert
        assert_eq!(b'H', converter.next().unwrap().unwrap());
        assert_eq!(b'E', converter.next().unwrap().unwrap());
        assert_eq!(b'L', converter.next().unwrap().unwrap());
        assert_eq!(b'L', converter.next().unwrap().unwrap());
        assert_eq!(b'O', converter.next().unwrap().unwrap());
        assert_eq!(' ', converter.next().unwrap().err().unwrap().0);
    }
}