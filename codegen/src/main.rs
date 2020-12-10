mod errors;
mod sqlstate;
mod type_gen;

pub use errors::*;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    version: u8,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    download("src/backend/utils/errcodes.txt", opt.version)?;
    download("src/include/catalog/pg_type.dat", opt.version)?;
    download("src/include/catalog/pg_range.dat", opt.version)?;

    type_gen::build(concat!(env!("CARGO_MANIFEST_DIR"), "/../src/types/gen.rs"))?;
    sqlstate::build(concat!(env!("CARGO_MANIFEST_DIR"), "/../src/state/gen.rs"))?;

    Ok(())
}

fn download(file: &str, version: u8) -> Result<()> {
    let url = format!("https://git.postgresql.org/gitweb/?p=postgresql.git;a=blob_plain;f={};hb=refs/heads/REL_{}_STABLE", file, version);
    let path = std::path::Path::new(file);
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let output = format!("{}/src/{}", env!("CARGO_MANIFEST_DIR"), file_name);
    let file = std::fs::File::create(output)?;

    attohttpc::get(&url)
        .send()?
        .write_to(&file)?;

    Ok(())
}
