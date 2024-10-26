pub mod iso_9660;

#[cfg(test)]
mod tests {
    use std::{fs::File, path::Path};
    use iso_9660::primary_volume_descriptor::*;

    use super::*;

    /// Temporary test to re-create an ISO I have locally on my machine.
    #[test]
    fn recreate_example_iso() -> Result<(), &'static str>{
        //const original_filename: &str = "example.iso";
        const OUTPUT_FILENAME: &str = "example_copy.iso";

        let output_path = Path::new(OUTPUT_FILENAME);

        // Delete the output iso if it exists.
        if output_path.exists() {
            std::fs::remove_file(output_path).expect("Failed to remove output file!");
        }

        // Create a new output file to write to
        let output_file = File::create_new(output_path).expect("Failed to create output file!");

        let primary_volume_descriptor = PrimaryVolumeDescriptor::new(
            "",
            "MY_EXAMPLE_ISO",
            600, 
            1,
            1,
            2048,
            10
        )?;

        primary_volume_descriptor.write_primary_volume_descriptor(output_file)
        .expect("Failed!");

        Ok(())
    }
}
