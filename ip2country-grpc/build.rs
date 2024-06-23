fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR")?);
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("ip2c_descriptor.bin"))
        .compile(&["proto/ip2c.proto"], &["proto"])?;

    Ok(())
}
