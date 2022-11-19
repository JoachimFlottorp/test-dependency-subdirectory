use std::io::Result;

static PROTOS: &'static [&'static str] = &["src/shirts.proto"];

fn main() -> Result<()> {
    for proto in PROTOS {
        print!("cargo:rerun-if-changed={}", proto);
    }
    
    prost_build::compile_protos(PROTOS, &["src/"])?;

    Ok(())
}