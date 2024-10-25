use std::{fs::File, path::Path};

pub mod iso_9660 {
    pub type SectorLocation = u64;

    /// The size of a sector on a CD-ROM
    pub const SECTOR_SIZE: u64 = 2048;

    /// `sector * SECTOR_SIZE`
    pub fn get_sector_location(sector: SectorLocation) -> u64 {
        sector * SECTOR_SIZE
    }

    pub mod volume_descriptors {
        use std::io::{Error, Seek, SeekFrom, Write};

        use super::get_sector_location;

        pub struct PrimaryVolumeDescriptor {
            /// The value `1` indicates the "Primary Volume Descriptor" is being used.
            ///
            /// I am unaware of any other descriptor models in existence.
            ///
            /// `1u8`
            pub volume_descriptor_type: [u8; 1],

            /// This short ASCII string indicates that this disc is an ISO-9660
            ///
            /// ASCII: `"CD001"`
            pub standard_identifier: [u8; 5],

            /// Version of the "Primary Volume Descriptor"
            ///
            /// `1u8`
            pub volume_descriptor_version: [u8; 1],

            /// Nothing like an unused byte!
            ///
            /// `0u8` I believe this byte is to pad strings into a binary spots.
            pub unused_field: [u8; 1],

            /// An ASCII string that can only contain the following characters, ignore the square brackets;
            ///
            /// `A-Z, 0-9, [_ !"%'()*+,-./:;<=>?]`
            pub system_identififer: SystemIdentifierString,
        }

        impl PrimaryVolumeDescriptor {
            /// Creates a new empty `PrimaryVolumeDescriptor`
            pub fn new() -> Result<PrimaryVolumeDescriptor, &'static str> {
                Ok(PrimaryVolumeDescriptor {
                    volume_descriptor_type: [1u8],
                    standard_identifier: b"CD001".to_owned(),
                    volume_descriptor_version: [1u8],
                    unused_field: [0u8],
                    system_identififer: match SystemIdentifierString::new(
                        &"                                ",
                    ) {
                        Ok(valid_system_identifier) => valid_system_identifier,
                        Err(_) => return Err("Failed to initialize system identifier."),
                    },
                })
            }
        }

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

            /// Attempts to convert `char` into a valid ASCII character for a System Identifier
            ///
            /// # Arguments
            ///
            /// * `input` - Character should be one of these `A-Z, 0-9, [_ !"%'()*+,-./:;<=>?]`
            ///
            /// # Returns
            ///
            /// `Some<u8>` on success!
            pub fn try_to_convert_char(input: char) -> Option<u8> {
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
            pub fn convert_str_to_valid_system_identifier(
                input: &str,
            ) -> Result<[u8; 32], (char, &str)> {
                // The output we're generating, initiate the entire string with ' ' spaces.
                let mut output_buf = [b' '; 32];
                let mut i: usize = 0;

                for c in input.chars() {
                    // Cannot go over 32 ASCII chars.
                    if i >= 32 {
                        return Err((c, "Input str is too long to be a valid system identifier."));
                    }

                    // Convert from char to valid subset of ASCII
                    output_buf[i] = match SystemIdentifierString::try_to_convert_char(c) {
                        // Parsed
                        Some(valid_char) => valid_char,
                        // Failed to parse, bomb out!
                        None => return Err((c, "Not a valid character for a system identifier.")),
                    };

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
            pub fn new(input: &str) -> Result<SystemIdentifierString, (char, &str)> {
                Ok(SystemIdentifierString {
                    string_data: SystemIdentifierString::convert_str_to_valid_system_identifier(
                        input,
                    )?,
                })
            }
        }

        /// Writes the `PrimaryVolumeDescriptor` to the output file on sector 16
        pub fn write_primary_volume_descriptor<IsoFile: Write + Seek>(
            mut output: IsoFile,
            descriptor: PrimaryVolumeDescriptor,
        ) -> Result<(), Error> {
            // Seek to the 16th sector of the file.
            let starting_location = get_sector_location(16);

            output.seek(SeekFrom::Start(starting_location))?;

            output.write(&descriptor.volume_descriptor_type)?;
            output.write(&descriptor.standard_identifier)?;
            output.write(&descriptor.volume_descriptor_version)?;
            output.write(&descriptor.unused_field)?;
            output.write(&descriptor.system_identififer.string_data)?;

            Ok(())
        }
    }
}

/*
fn write_zeroes(mut cursor: IsoCursor, quantity: usize) -> Result<(), std::io::Error> {
    let zero_buffer = vec![0u8; quantity];
    cursor.write_all(&zero_buffer)
}

/// System area (32,768 B) - Unused by ISO 9660
fn system_area(mut cursor: IsoCursor) {
    const SYSTEM_AREA_LENGTH: usize = 32768;

    cursor.seek(SeekFrom::Start(0))
    .expect("Failed to seek cursor!");

write_zeroes(cursor, SYSTEM_AREA_LENGTH)
.expect("Failed to write System Area");
}


fn create_iso(mut cursor: IsoCursor) {
    // NOTE: All strings are in ISO 646 character set
    // https://en.wikipedia.org/wiki/ISO/IEC_646
    // https://crates.io/crates/encoding_rs

    let buf: Vec<u8> = Vec::new();


    // Just write out the ISO here to start, figure out a good pattern for this later.
    system_area(cursor);


    let x: VolumeDescriptor = volume_descriptors::PRIMARY_VOLUME_DESCRIPTOR;

    let buffer = [x];

    let path = Path::new("my.iso");

    if path.exists() {
        fs::remove_file(&path).expect("Failed to remove file!");
        println!("File already existed, replacing it!")
    }

    println!("Creating file...");
    let mut file = File::create(&path)
    .expect("Failed to create file!");

println!("Writing data...");
file.write_all(&buffer)
.expect("Failed to write to file!");

println!("Finished!");
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    /// Temporary test to re-create an ISO I have locally on my machine.
    #[test]
    fn recreate_example_iso() {
        //const original_filename: &str = "example.iso";
        const OUTPUT_FILENAME: &str = "example_copy.iso";

        let output_path = Path::new(OUTPUT_FILENAME);

        // Delete the output iso if it exists.
        if output_path.exists() {
            std::fs::remove_file(output_path).expect("Failed to remove output file!");
        }

        // Create a new output file to write to
        let output_file = File::create_new(output_path).expect("Failed to create output file!");

        let primary_volume_descriptor =
            iso_9660::volume_descriptors::PrimaryVolumeDescriptor::new()
                .expect("Failed to generate primary volume descriptor!");

        iso_9660::volume_descriptors::write_primary_volume_descriptor(
            output_file,
            primary_volume_descriptor,
        )
        .expect("Failed to write primary volume descriptor!");
    }
}
