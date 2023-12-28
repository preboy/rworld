use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["src/protobuf/a.proto"], &["src/"])?;
    Ok(())
}
