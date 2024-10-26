use std::io::{Error, Seek, SeekFrom};

/// Number of a sector on an image.
pub type SectorLocation = u64;

// TODO: ISO_9660_SECTOR_SIZE This may not be correct.
/// The size of a sector on a CD-ROM
pub const ISO_9660_SECTOR_SIZE: u64 = 2048;

/// # Get Sector Location
/// 
/// The location of a sector
/// 
/// # Returns 
/// 
/// `sector * ISO_9660_SECTOR_SIZE`
pub fn get_sector_location(sector: SectorLocation) -> u64 {
    sector * ISO_9660_SECTOR_SIZE
}

/// # Seek to Sector
/// 
/// Seeks to a sector location.
/// 
/// # Returns
/// 
/// Seeking can fail, for example because it might involve flushing a buffer.
///
/// Seeking to a negative offset is considered an error.
pub fn seek_to_sector<T: Seek>(seek: &mut T, sector: SectorLocation) -> Result<u64, Error> {
    let sector_location = get_sector_location(sector);
    seek.seek(SeekFrom::Start(sector_location))
}