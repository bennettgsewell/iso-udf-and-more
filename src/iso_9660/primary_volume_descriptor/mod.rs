use std::io::{Error, Seek, Write};

mod system_identifier_string;
mod volume_identifier_string;

use system_identifier_string::*;
use volume_identifier_string::*;

use super::sector;

/// # Primary Volume Descriptor
/// 
/// The header of an ISO-9660 CD-ROM
/// 
/// It contains information such as the label, pointer to the root directory, and volume information.
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

    /// An ASCII string that can only contain the following characters, ignore the square brackets;
    ///
    /// `A-Z, 0-9, [_]`
    pub volume_identifier: VolumeIdentifierString,

    /// Nothing like an unused bytes!
    ///
    /// `[0u8; 8]`
    pub unused_fields2: [u8; 8],

    /// Logical Volume block count
    pub volume_space_size: u32,

    /// Nothing like an unused bytes!
    ///
    /// `[0u8; 32]`
    pub unused_fields3: [u8; 32],

    /// The volume set size
    /// 
    /// It's how many CDs are in a set. For instance a video game came with
    /// five installation CD-ROMs in its box, or a music album may contain two discs.
    pub volume_set_size: u16,

    /// Which disc this is in a volume set.
    pub volume_sequence_number: u16,

    /// How big a local block of data is on the disc.
    /// Usually this is the same as the physical sector size, on a CD-ROM that would be 2048.
    pub logical_block_size: u16,

    /// The size of the path table in bytes.
    pub path_table_size: u32
}

impl PrimaryVolumeDescriptor {
    /// Creates a new empty `PrimaryVolumeDescriptor`
    pub fn new<'a>(
        system_identifier: &str,
        volume_identifier: &str,
        volume_space_size: u32,
        volume_set_size: u16,
        volume_sequence_number: u16,
        logical_block_size: u16,
        path_table_size: u32
    ) -> Result<PrimaryVolumeDescriptor, &'a str> {
        Ok(PrimaryVolumeDescriptor {
            volume_descriptor_type: [1u8],
            standard_identifier: b"CD001".to_owned(),
            volume_descriptor_version: [1u8],
            unused_field: [0u8],
            system_identififer: SystemIdentifierString::new(system_identifier).map_err(|err| err.1)?,
            volume_identifier: VolumeIdentifierString::new(volume_identifier).map_err(|err| err.1)?,
            unused_fields2: [0u8; 8],
            volume_space_size: volume_space_size,
            unused_fields3: [0u8; 32],
            volume_set_size: volume_set_size,
            volume_sequence_number: volume_sequence_number,
            logical_block_size: logical_block_size,
            path_table_size: path_table_size
        })
    }
    
    /// Writes the `PrimaryVolumeDescriptor` to the output file on sector 16
    pub fn write_primary_volume_descriptor<IsoFile: Write + Seek>(
        self: &PrimaryVolumeDescriptor,
        mut output: IsoFile,
    ) -> Result<(), Error> {
        // Seek to the 16th sector of the file.
        sector::seek_to_sector(&mut output, 16)?;
        
        output.write(&self.volume_descriptor_type)?;
        output.write(&self.standard_identifier)?;
        output.write(&self.volume_descriptor_version)?;
        output.write(&self.unused_field)?;
        output.write(self.system_identififer.get_ascii_data())?;
        output.write(self.volume_identifier.get_ascii_data())?;
        output.write(&self.unused_fields2)?;
        output.write(&self.volume_space_size.to_le_bytes())?;
        output.write(&self.volume_space_size.to_be_bytes())?;
        output.write(&self.unused_fields3)?;
        output.write(&self.volume_set_size.to_le_bytes())?;
        output.write(&self.volume_set_size.to_be_bytes())?;
        output.write(&self.volume_sequence_number.to_le_bytes())?;
        output.write(&self.volume_sequence_number.to_be_bytes())?;
        output.write(&self.logical_block_size.to_le_bytes())?;
        output.write(&self.logical_block_size.to_be_bytes())?;
        output.write(&self.path_table_size.to_le_bytes())?;
        output.write(&self.path_table_size.to_be_bytes())?;

        output.flush()?;

        Ok(())
    }
}