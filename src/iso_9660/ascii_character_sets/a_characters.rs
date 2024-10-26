/// Attempts to convert `char` into a valid ASCII character from the a-character set.
///
/// # Arguments
///
/// * `input` - Character should be one of these `A-Z, 0-9, [_ !"%'()*+,-./:;<=>?]`
pub fn convert_char_to_a_character_set(input: char) -> Option<u8> {
    // If the input is within the valid subset, return it.
    match input {
        // [ !"]
        ' '..='"'
        // [%]
        | '%'
        // ['()*+,-./0123456789:;<=>?]
        | '\''..='?'
        // [ABCDEFGHIJKLMNOPQRSTUVWXYZ]
        | 'A'..='Z'
        // [_]
        | '_'
        => Some(input as u8),
        _   => return None
    }
}

#[cfg(test)]
mod tests {
    use super::convert_char_to_a_character_set;

    #[test]
    pub fn convert_char_to_a_character_set_valid_characters() {
        const VALID_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ_ !\"%'()*+,-./:;<=>?";

        for input in VALID_CHARS.chars() {
            let output = convert_char_to_a_character_set(input)
            .expect("Failed to convert char!");

            assert_eq!(output, input as u8);
        }
    }

    #[test]
    pub fn convert_char_to_a_character_set_invalid_characters() {
        const VALID_CHARS: &str = "abcdefghijklmnoqrstuvwxyz[]{}^@#";

        for input in VALID_CHARS.chars() {
            let output = convert_char_to_a_character_set(input);

            assert_eq!(None, output);
        }
    }
}