use std::io::{Error, Read, Seek};

use primary_volume_descriptor::PrimaryVolumeDescriptor;

pub mod sector;
pub mod primary_volume_descriptor;
pub mod ascii_character_sets;

/// # Detect if ISO-9660
/// 
/// Detects if the input data is an ISO-9660 filesystem.
pub fn try_read_iso_9660<T: Read + Seek>(input: &mut T) -> Result<bool, Error> {
    sector::seek_to_sector(input, 16);
    Ok(PrimaryVolumeDescriptor::is_primary_volume_descriptor_header(input)?)
}

pub struct Iso9660 {
    pub primary_volume_descriptor: PrimaryVolumeDescriptor
}