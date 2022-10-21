mod errors;
mod sqlstate;
mod type_gen;

pub use errors::*;

use clap::Parser;

#[derive(Parser)]
struct Opt {
    version: u8,
}

fn main() -> Result {
    let opt = Opt::parse();

    download("src/backend/utils/errcodes.txt", opt.version)?;
    download("src/include/catalog/pg_type.dat", opt.version)?;
    download("src/include/catalog/pg_range.dat", opt.version)?;

    type_gen::build(concat!(env!("CARGO_MANIFEST_DIR"), "/../src/types/gen.rs"))?;
    sqlstate::build(concat!(env!("CARGO_MANIFEST_DIR"), "/../src/state/gen.rs"))?;

    Ok(())
}

fn download(file: &str, version: u8) -> Result {
    let url = format!("https://git.postgresql.org/gitweb/?p=postgresql.git;a=blob_plain;f={file};hb=refs/heads/REL_{version}_STABLE");
    let path = std::path::Path::new(file);
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let output = format!("{}/src/{file_name}", env!("CARGO_MANIFEST_DIR"));
    let file = std::fs::File::create(output)?;

    attohttpc::get(&url).send()?.write_to(&file)?;

    Ok(())
}
