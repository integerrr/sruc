use std::io::Result;

fn main() -> Result<()> {
    let mut prost_builder = prost_build::Config::new();
    prost_builder
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&["src/ei.proto"], &["src/"])?;
    Ok(())
}
