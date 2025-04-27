use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compile the proto files and enerate the file descriptor set
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("reflection_descriptor_set.bin"))
        .compile_protos(&["proto/calculator.proto"], &["proto"])?;

    Ok(())
}
