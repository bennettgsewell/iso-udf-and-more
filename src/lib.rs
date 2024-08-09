use std::{fs::{self, File}, io::Write, path::Path};

use volume_descriptors::VolumeDescriptor;

/// First byte of ISO 9660
pub mod volume_descriptors {
    pub type VolumeDescriptor = u8;

    /// Primary Volume Descriptor is the default 
    pub const PRIMARY_VOLUME_DESCRIPTOR: VolumeDescriptor = 1u8;

    // TODO: There are more volume descriptors but I don't know what they are.
}

fn work() {
    // NOTE: Some values need to be stored in big endian,
    // may want to look into the byteorder crate.
    // https://crates.io/crates/byteorder

    // NOTE: All strings are in ISO 646 character set
    // https://en.wikipedia.org/wiki/ISO/IEC_646
    // https://crates.io/crates/encoding_rs

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

    #[test]
    fn test() {
        work();
    }
}