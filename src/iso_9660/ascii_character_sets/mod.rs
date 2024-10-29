use validator::CharacterValidator;

mod a_characters;
mod d_characters;
pub mod validator;

/// ISO-9660 ASCII character subsets
pub enum CharacterSets {
    /// # a-characters
    /// 
    /// The following are valid ASCII characters;
    /// 
    /// `ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_ !"%'()*+,-./:;<=>?`
    ACharacters,

    /// # d-characters
    /// 
    /// The following are valid ASCII characters;
    /// 
    /// `ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_`
    DCharacters
}

/// A function that attempts to validate a character as a valid ASCII byte character.
type ValidateCharFn = fn(char) -> Option<u8>;

/// Returns a function pointer for the character validater associated with this ASCII Subset
fn get_validator_fn_for(character_set: CharacterSets) -> ValidateCharFn {
    match character_set {
        CharacterSets::ACharacters => a_characters::convert_char_to_a_character_set,
        CharacterSets::DCharacters => d_characters::convert_char_to_d_character_set
    }
}

/// # Convert str into buffer
/// 
/// Converts a string into a valid ASCII character subset.
/// 
/// # Arguments
/// 
/// * `input` - Input UTF-8 str to be validated.
/// * `character_set` - The ISO-9660 ASCII character set to validate against.
/// * `output` - The buffer to copy the data into.
/// * `filler` - Fills the rest of the array with this byte.
/// 
/// # Returns
/// 
/// Error with the offending character and reason message.
pub fn convert_str_into_buf(
    input: &str,
    character_set: CharacterSets,
    output: &mut [u8],
    filler: u8
) -> Result<(), char>{
    let mut converter = CharacterValidator::new(input, character_set);

    for i in 0..=output.len() {
        output[i] = match converter.next() {
            Some(character) => character?,
            None => filler
        }
    }

    Ok(())
}