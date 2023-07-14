use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["src/igdbapi.proto"], &["src/"])?;
    Ok(())
}