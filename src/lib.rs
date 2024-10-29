use std::io::{Error, Read, Seek};

pub mod iso_9660;

struct ImageFS {
    is_iso_9660: bool
}

impl ImageFS {
    pub fn LoadFrom<T: Read + Seek>(data: T) -> Result<ImageFS, Error> {
        iso_9660::
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, path::Path};
    use iso_9660::primary_volume_descriptor::*;

    use super::*;

    /// Temporary test to re-create an ISO I have locally on my machine.
    #[test]
    fn copy_iso() {
        const INPUT_FILENAME: &str = "example.iso";
        const OUTPUT_FILENAME: &str = "example_copy.iso";

        let input_file = File::open(INPUT_FILENAME)
        .expect("Failed to open input file");

        ImageFS::LoadFrom(input_file);

        /*
        let output_path = Path::new(OUTPUT_FILENAME);

        // Delete the output iso if it exists.
        if output_path.exists() {
            std::fs::remove_file(output_path).expect("Failed to remove output file!");
        }

        // Create a new output file to write to
        let output_file = File::create_new(output_path).expect("Failed to create output file!");

        primary_volume_descriptor.write_primary_volume_descriptor(output_file)
        .expect("Failed!");

        Ok(())
        */
    }
}
