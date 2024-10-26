use std::str::Chars;

use super::{CharacterSets, ConvertCharFn};

pub struct CharacterConverter<'a> {
    characters: Chars<'a>,
    converter: ConvertCharFn
}

impl CharacterConverter<'_> {
    pub fn new(input: &str, character_set: CharacterSets) -> CharacterConverter {
        let converter = super::get_converter_fn_for(character_set);

        CharacterConverter {
            characters: input.chars(),
            converter: converter
        }
    }
}

impl Iterator for CharacterConverter<'_> {
    type Item = Result<u8, (char, &'static str)>;

    fn next(&mut self) -> Option<Self::Item> {
        // There is a next char or return None
        let next_char = self.characters.next()?;

        // Convert the char, when fail return Some(Err(reason why));
        Some(match (self.converter)(next_char) {
            Some(converted_char) => Ok(converted_char),
            None => Err((next_char, "Character is not valid in this character set!"))
        })
    }
}

#[cfg(test)]
mod character_converter_iterator_tests {
    use super::CharacterSets;

    use super::*;

    #[test]
    pub fn iterate_hello_world_str_into_ascii_bytes() {
        // Arrange
        const HELLO_WORLD: &str = "HELLO_WORLD";

        let converter = CharacterConverter::new(HELLO_WORLD, CharacterSets::DCharacters);

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

        let mut converter = CharacterConverter::new(HELLO_WORLD, CharacterSets::DCharacters);

        // Act & Assert
        assert_eq!(b'H', converter.next().unwrap().unwrap());
        assert_eq!(b'E', converter.next().unwrap().unwrap());
        assert_eq!(b'L', converter.next().unwrap().unwrap());
        assert_eq!(b'L', converter.next().unwrap().unwrap());
        assert_eq!(b'O', converter.next().unwrap().unwrap());
        assert_eq!(' ', converter.next().unwrap().err().unwrap().0);
    }
}