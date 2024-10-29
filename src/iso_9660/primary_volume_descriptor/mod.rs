use std::io::{Error, Read, Seek, SeekFrom, Write};

use super::{
    ascii_character_sets::{self, convert_str_into_buf, CharacterSets},
    sector,
};

/// # Primary Volume Descriptor
///
/// The header of an ISO-9660 CD-ROM located at sector 16.
///
/// It contains information such as the label, pointer to the root directory, and volume information.
#[derive(Default)]
pub struct PrimaryVolumeDescriptor {
    /// An ASCII string that can only contain the following characters, ignore the square brackets;
    ///
    /// `A-Z, 0-9, [_ !"%'()*+,-./:;<=>?]`
    pub system_identififer: String,

    /// An ASCII string that can only contain the following characters, ignore the square brackets;
    ///
    /// `A-Z, 0-9, [_]`
    pub volume_identifier: String,

    /// Logical Volume block count
    pub volume_space_size: u32,

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
    pub path_table_size: u32,

    /// Logical block number of LE Path Table
    pub le_path_table: u32,
}

impl PrimaryVolumeDescriptor {
    /// # Is Primary Volume Descriptor header
    ///
    /// Detects the
    pub fn read_primary_volume_descriptor<T: Read + Seek>(
        input: &mut T,
    ) -> Result<Option<PrimaryVolumeDescriptor>, Error> {
        sector::seek_to_sector(input, 16)?;

        // This short ASCII string indicates that this filesystem is ISO-9660
        const EXPECTED_DATA: &[u8; 6] = b"\x01CD001";
        const BUF_SIZE: usize = EXPECTED_DATA.len();

        let mut buf = [0u8; BUF_SIZE];
        let read = input.read(&mut buf)?;

        if BUF_SIZE != read || EXPECTED_DATA != &buf {
            return Ok(None);
        }

        fn invalid_data(err_msg: &'static str) -> Error {
            Error::new(std::io::ErrorKind::InvalidData, err_msg)
        }

        fn bomb_out(err_msg: &'static str) -> Result<Option<PrimaryVolumeDescriptor>, Error> {
            Err(invalid_data(err_msg))
        }

        // The value `1` indicates the "Primary Volume Descriptor" structure is being used here.
        // TODO: I am unaware of any other descriptor models in existence.
        if buf[0] != 1 {
            return bomb_out("Volume Descriptor Type is not supported!");
        }

        fn read_byte<T: Read>(mut input: T) -> Result<u8, Error> {
            let mut buf = [0u8; 1];
            input.read_exact(&mut buf)?;
            Ok(buf[0])
        }
        
        let mut return_data = PrimaryVolumeDescriptor::default();

        // Version of the "Primary Volume Descriptor"
        // TODO: I am unaware if there are/were future versions or not.
        let primary_descriptor_volume_ver = read_byte(input)?;
        if primary_descriptor_volume_ver != 1 {
            return bomb_out("Primary Volume Descriptor version is not supported!")
        }

        // Nothing like an unused byte!
        input.seek(SeekFrom::Current(1))?;

        let mut system_identifier_buf = [b' '; 32];
        input.read_exact(&mut system_identifier_buf)?;
        return_data.system_identififer = String::from_utf8_lossy(&system_identifier_buf[..])
        .map_err(|_| invalid_data("Failed to parse system identifier"))?;

        let mut volume_identifier_buf = [b' '; 32];
        input.read_exact(&mut volume_identifier_buf)?;
    
    /////////////////////////////////HERE
        output.write(&system_identifier_buf)?;
        output.write(&volume_identifier_buf)?;

        // Nothing like an unused bytes!
        output.write(&[0u8; 8])?;

        // TODO: Make a bi-endian struct so these are simpler
        output.write(&self.volume_space_size.to_le_bytes())?;
        output.write(&self.volume_space_size.to_be_bytes())?;

        // Nothing like an unused bytes!
        output.write(&[0u8; 32])?;

        output.write(&self.volume_set_size.to_le_bytes())?;
        output.write(&self.volume_set_size.to_be_bytes())?;

        output.write(&self.volume_sequence_number.to_le_bytes())?;
        output.write(&self.volume_sequence_number.to_be_bytes())?;

        output.write(&self.logical_block_size.to_le_bytes())?;
        output.write(&self.logical_block_size.to_be_bytes())?;

        output.write(&self.path_table_size.to_le_bytes())?;
        output.write(&self.path_table_size.to_be_bytes())?;

        output.write(&self.le_path_table.to_le_bytes())?;



    }

    /*
    pub fn read_primary_volume_descriptor<T: Read + Seek>(input: &mut T) -> Result<PrimaryVolumeDescriptor, Error>{
        sector::seek_to_sector(input, 16)?;

        // The value `1` indicates the "Primary Volume Descriptor" structure is being used here.
        // TODO: I am unaware of any other descriptor models in existence.'
        input.read_exact(buf).write(&[1u8; 1])?;

        // This short ASCII string indicates that this filesystem is ISO-9660
        output.write(b"CD001")?;

        // Version of the "Primary Volume Descriptor"
        // TODO: I am unaware if there are/were future versions or not.
        output.write(&[1u8; 1])?;

        // Nothing like an unused byte!
        output.write(&[0u8; 1])?;

        let mut system_identifier_buf = [b' '; 32];
        convert_str_into_buf(
            &self.system_identififer,
            CharacterSets::ACharacters,
            &mut system_identifier_buf,
            b' ')
            .map_err(|_| Error::new(std::io::ErrorKind::InvalidInput, "Failed to convert primary volume descriptor system identifier into supported ASCII"))?;

        let mut volume_identifier_buf = [b' '; 32];
        convert_str_into_buf(
            &self.volume_identifier,
            CharacterSets::ACharacters,
            &mut volume_identifier_buf,
            b' ')
            .map_err(|_| Error::new(std::io::ErrorKind::InvalidInput, "Failed to convert primary volume descriptor volume identifier into supported ASCII"))?;

        output.write(&system_identifier_buf)?;
        output.write(&volume_identifier_buf)?;

        // Nothing like an unused bytes!
        output.write(&[0u8; 8])?;

        // TODO: Make a bi-endian struct so these are simpler
        output.write(&self.volume_space_size.to_le_bytes())?;
        output.write(&self.volume_space_size.to_be_bytes())?;

        // Nothing like an unused bytes!
        output.write(&[0u8; 32])?;

        output.write(&self.volume_set_size.to_le_bytes())?;
        output.write(&self.volume_set_size.to_be_bytes())?;

        output.write(&self.volume_sequence_number.to_le_bytes())?;
        output.write(&self.volume_sequence_number.to_be_bytes())?;

        output.write(&self.logical_block_size.to_le_bytes())?;
        output.write(&self.logical_block_size.to_be_bytes())?;

        output.write(&self.path_table_size.to_le_bytes())?;
        output.write(&self.path_table_size.to_be_bytes())?;

        output.write(&self.le_path_table.to_le_bytes())?;

        Ok(PrimaryVolumeDescriptor {

        })
    }
    */
    /// # Write Primary Volume Descriptor
    ///
    /// Writes the `PrimaryVolumeDescriptor` to the output file to sector 16
    pub fn write_primary_volume_descriptor<IsoFile: Write + Seek>(
        self: &PrimaryVolumeDescriptor,
        mut output: IsoFile,
    ) -> Result<(), Error> {
        // Seek to the 16th sector of the file.
        sector::seek_to_sector(&mut output, 16)?;

        // The value `1` indicates the "Primary Volume Descriptor" structure is being used here.
        // TODO: I am unaware of any other descriptor models in existence.
        output.write(&[1u8; 1])?;

        // This short ASCII string indicates that this filesystem is ISO-9660
        output.write(b"CD001")?;

        // Version of the "Primary Volume Descriptor"
        // TODO: I am unaware if there are/were future versions or not.
        output.write(&[1u8; 1])?;

        // Nothing like an unused byte!
        output.write(&[0u8; 1])?;

        let mut system_identifier_buf = [b' '; 32];
        convert_str_into_buf(
            &self.system_identififer,
            CharacterSets::ACharacters,
            &mut system_identifier_buf,
            b' ')
            .map_err(|_| Error::new(std::io::ErrorKind::InvalidInput, "Failed to convert primary volume descriptor system identifier into supported ASCII"))?;

        let mut volume_identifier_buf = [b' '; 32];
        convert_str_into_buf(
            &self.volume_identifier,
            CharacterSets::ACharacters,
            &mut volume_identifier_buf,
            b' ')
            .map_err(|_| Error::new(std::io::ErrorKind::InvalidInput, "Failed to convert primary volume descriptor volume identifier into supported ASCII"))?;

        output.write(&system_identifier_buf)?;
        output.write(&volume_identifier_buf)?;

        // Nothing like an unused bytes!
        output.write(&[0u8; 8])?;

        // TODO: Make a bi-endian struct so these are simpler
        output.write(&self.volume_space_size.to_le_bytes())?;
        output.write(&self.volume_space_size.to_be_bytes())?;

        // Nothing like an unused bytes!
        output.write(&[0u8; 32])?;

        output.write(&self.volume_set_size.to_le_bytes())?;
        output.write(&self.volume_set_size.to_be_bytes())?;

        output.write(&self.volume_sequence_number.to_le_bytes())?;
        output.write(&self.volume_sequence_number.to_be_bytes())?;

        output.write(&self.logical_block_size.to_le_bytes())?;
        output.write(&self.logical_block_size.to_be_bytes())?;

        output.write(&self.path_table_size.to_le_bytes())?;
        output.write(&self.path_table_size.to_be_bytes())?;

        output.write(&self.le_path_table.to_le_bytes())?;

        output.flush()?;

        Ok(())
    }
}
