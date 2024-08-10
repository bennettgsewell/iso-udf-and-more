use std::{fs::{self, File}, io::{Cursor, Read, Seek, SeekFrom, Write}, path::Path};

use volume_descriptors::VolumeDescriptor;

/// Cursor for modifying the buffer of the ISO 9660
pub type IsoCursor = Cursor<Vec<u8>>;

/// First byte of ISO 9660
pub mod volume_descriptors {
    use std::io::Write;

    use crate::IsoCursor;

    pub type VolumeDescriptor = u8;

    /// Primary Volume Descriptor is the default 
    pub const PRIMARY_VOLUME_DESCRIPTOR: VolumeDescriptor = 1u8;

    struct PrimaryVolumeDescriptor {
        // Volume Identifier
        // Volume Set Identifier
        // System Identifier
        // Volume Size
        // Number of Volumes in this Set
        // Number of this Volume in the Set
        // Logical Block Size
        // Size of the Path Table
        // Location of the Path Table
        // Root Directory Record
        // Other Identifiers
        // Time Stamps
    }

    pub fn write_primary_volume_descriptor(mut cursor: IsoCursor, descriptor: PrimaryVolumeDescriptor) {
        const
    }

    // TODO: There are more volume descriptors but I don't know what they are.
}

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


fn wok() {
    // NOTE: Some values need to be stored in big endian,
    // may want to look into the byteorder crate.
    // https://crates.io/crates/byteorder

    // NOTE: All strings are in ISO 646 character set
    // https://en.wikipedia.org/wiki/ISO/IEC_646
    // https://crates.io/crates/encoding_rs

    let buf: Vec<u8> = Vec::new();

    let mut cursor: IsoCursor = Cursor::new(buf);

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

#[cfg(test)]
mod tests {
    use super::*;

    fn read_entire_file(path: &str) -> Result<Vec<u8>, std::io::Error> {
        let mut file_handle = File::open(path)?;
        let mut file_buf: Vec<u8> = Vec::new();
        file_handle.read_to_end(&mut file_buf)?;
        Ok(file_buf)
    }

    fn read_hello_txt() -> Result<(&'static str, Vec<u8>), std::io::Error> {
        const PATH: &str = "HELLO.TXT";
        let buffer = read_entire_file(PATH)?;
        Ok((&PATH, buffer))
    }

    /// Temporary test to re-create an ISO I have locally on my machine.
    #[test]
    fn recreate_example_iso() {
        // Arrange
        let (hello_file_name, hello_buffer) = read_hello_txt()
            .expect("Failed to read HELLO.TXT");

        // Act
        

        // Assert
    }
}