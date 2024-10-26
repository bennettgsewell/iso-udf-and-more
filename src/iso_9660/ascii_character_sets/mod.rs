mod a_characters;
mod d_characters;
pub mod converter;

/// ISO-9660 character sets
pub enum CharacterSets {
    /// # a-characters
    /// 
    /// The following ASCII characters are valid;
    /// 
    /// `ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_ !"%'()*+,-./:;<=>?`
    ACharacters,

    /// # d-characters
    /// 
    /// The following ASCII characters are valid;
    /// 
    /// `ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_`
    DCharacters
}

/// A function that attempts to convert a character to a valid ASCII bytecharacter set.
type ConvertCharFn = fn(char) -> Option<u8>;

fn get_converter_fn_for(character_set: CharacterSets) -> ConvertCharFn {
    match character_set {
        CharacterSets::ACharacters => a_characters::convert_char_to_a_character_set,
        CharacterSets::DCharacters => d_characters::convert_char_to_d_character_set
    }
}

pub fn get_converter(input: &str, character_set: CharacterSets) -> converter::CharacterConverter<'_> {
    converter::CharacterConverter::new(input, character_set)
}
