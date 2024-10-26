
use super::super::ascii_character_sets;

/// An ASCII string that can only contain the following characters, ignore the square brackets;
///
/// `A-Z, 0-9, [_ !"%'()*+,-./:;<=>?]`
pub struct SystemIdentifierString {
    string_data: [u8; 32],
}

impl SystemIdentifierString {
    /// # Returns
    ///
    /// The valid string currently stored in the `SystemIdentifierString` as an array of bytes.
    pub fn get_ascii_data(&self) -> &[u8; 32] {
        &self.string_data
    }

    /// # Convert str to Valid System Identifier
    ///
    /// Is the `&str` a valid ASCII string that only contains the following characters; (square brackets not included)
    ///
    /// `A-Z, 0-9, [_ !"%'()*+,-./:;<=>?]`
    ///
    /// ## Arguments
    ///
    /// * `input` - Should be no longer than 32 characters.
    ///
    /// ## Returns
    ///
    /// * `Ok([u8; 32])` - A valid 32 byte ASCII array that contains only valid characters.
    /// * `Err((Option<char>, &str))` - Offending character and the error the message.
    pub fn convert_str_to_valid_system_identifier<'a>(
        input: &str,
    ) -> Result<[u8; 32], (char, &'a str)> {
        // Str must be <= 32 chars
        if input.chars().count() > 32 {
            return Err(('_', "System Identifiers cannot be larger than 32 characters!"));
        }

        // Convert the 
        let converter = ascii_character_sets::get_converter(input, ascii_character_sets::CharacterSets::ACharacters);

        let mut output_buf = [b' '; 32];

        let mut i: usize = 0;
        for c in converter {
            output_buf[i] = c?;
            i += 1;
        }

        // Success!
        Ok(output_buf)
    }

    /// # Construct `SystemIdentifier` from `&str`
    ///
    /// Attempts to convert a `&str` to a valid ASCII subset for a System Identifier string.
    ///
    /// # Arguments
    ///
    /// * `input` - Valid characters are, ignoring the square brackets, `A-Z, 0-9, [_ !"%'()*+,-./:;<=>?]`
    ///
    /// # Returns
    ///
    /// * `Ok(SystemIdentifierString)` - The string converts successfully!
    /// * `Err((char, &str))` - Failed, offending char and error message.
    pub fn new<'a>(input: &str) -> Result<SystemIdentifierString, (char, &'a str)> {
        Ok(SystemIdentifierString {
            string_data: SystemIdentifierString::convert_str_to_valid_system_identifier(
                input,
            )?,
        })
    }
}